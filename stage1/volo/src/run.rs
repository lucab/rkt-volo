use envy;
use errors;
use libc;
use nix::{fcntl, unistd};
use rkt_stage1::{self, appc, cli};
use serde_json;
use slog;
use std::{fs, path, process};
use std::collections::BTreeMap;
use uuid;

use std::io::Write;
use std::os::unix::io::IntoRawFd;
use std::os::unix::process::CommandExt;

pub fn volo_run(logger: slog::Logger) -> errors::Result<Option<process::Command>> {

    // Parse command-line config.
    let matches = try!(cli::run_flags().get_matches_safe());
    let stage1_env = try!(envy::from_env::<cli::RunEnvFlags>());
    // TODO(lucab): implement dns-conf modes
    let _dns_conf = match value_t!(matches, "dns-conf-mode", cli::RunDnsConf) {
        Ok(d) => d,
        Err(_) => Default::default(),
    };
    let _rt = rkt_stage1::RuntimePod {
        Debug: matches.is_present("debug"),
        Mutable: false,
        Hostname: None,
        MDSToken: None,
        NetList: None,
        PrivateUsers: None,
        InsecureOptions: rkt_stage1::InsecureOptions {
            DisableCapabilities: true,
            DisablePaths: true,
            DisableSeccomp: true,
        },
        ..Default::default()
    };
    let _local_config = {
        let u = try!(value_t!(matches, "local-config", String));
        try!(path::Path::new(&u).canonicalize())
    };
    let pod_fd_lock = stage1_env.rkt_lock_fd;
    let pod_dir = {
        let d = try!(unistd::getcwd());
        try!(d.canonicalize())
    };
    let pod_uuid = try!(value_t!(matches, "uuid", uuid::Uuid));
    slog_debug!(logger, "volo runtime pod";
                "pod UUID" => pod_uuid.hyphenated().to_string(),
                "pod dir" => format!("{}", pod_dir.display()));

    // Load pod manifest.
    let f = try!(fs::File::open("pod"));
    let pm: appc::PodManifest = try!(serde_json::from_reader(f));
    slog_debug!(logger, "manifest loaded";
                "version" => pm.acVersion);
    // TODO: move to better serde types
    let mut empty_vols: BTreeMap<&str, ()> = BTreeMap::new();
    let mut host_vols: BTreeMap<&str, rkt_stage1::BindTuple> = BTreeMap::new();
    if let Some(ref volumes) = pm.volumes {
        for v in volumes.iter() {
            match v.kind {
                appc::VolumeKind::Empty => {
                    empty_vols.insert(&v.name, ());
                }
                appc::VolumeKind::Host => {
                    let b = (v.source.clone().unwrap_or("".into()),
                             path::PathBuf::from(""),
                             v.recursive.unwrap_or(true),
                             0u64);
                    host_vols.insert(&v.name, b);
                }
            };
        }
    };

    // Load app manifest.
    if pm.apps.len() != 1 {
        bail!("only one app supported in volo pods");
    }
    let ref app = pm.apps[0].app;
    let ref app_name = pm.apps[0].name;
    let app_mounts = pm.apps[0].mounts.as_ref();
    let app_rootfs = pod_dir.join("stage1")
        .join("rootfs")
        .join("opt")
        .join("stage2")
        .join(app_name)
        .join("rootfs");
    slog_debug!(logger, "single app found";
                "app name" => *app_name);

    // Parse app details.
    let (exec_cmd, exec_args) = {
        let e = try!(app.exec.as_ref().ok_or("no exec entrypoint".to_string()));
        try!(e.split_first().ok_or("empty exec entrypoint".to_string()))
    };
    let workdir = match app.workingDirectory {
        Some(ref p) => p,
        None => path::Path::new("/"),
    };
    let exec_env = match app.environment {
        Some(ref v) => v.as_slice(),
        None => &[],
    };
    let mountpoints = try!(parse_mounts(empty_vols, host_vols, app_mounts));
    let uid = try!(appc::resolve_uid(&app.user, &app_rootfs));
    let gid = try!(appc::resolve_gid(&app.group, &app_rootfs));
    slog_debug!(logger, "stage2 entrypoint ready";
                "uid" => uid,
                "gid" => gid,
                "exec cmd" => exec_cmd.to_string(),
                "exec args" => format!("{:?}", exec_args),
                "exec env" => format!("{:?}", exec_env),
                "workdir" => format!("{}", workdir.display()));

    // rkt-stage1 interface: pidfile
    let podpid = unistd::getpid();
    let mut pidfile = try!(fs::File::create(pod_dir.join("pid")));
    try!(pidfile.write_fmt(format_args!("{}", podpid)));
    try!(pidfile.flush());
    slog_debug!(logger, "pod pidfile written";
                "pidfile path" => format!("{}", pod_dir.join("pid").display()),
                "pid" => podpid);

    // rkt-volo internal: pgidfile
    let podpgid = unsafe { libc::getpgid(0) };
    let mut pgidfile = try!(fs::File::create(pod_dir.join("pgid")));
    try!(pgidfile.write_fmt(format_args!("{}", podpgid)));
    try!(pgidfile.flush());
    slog_debug!(logger, "pod pgidfile written";
                "pgidfile path" => format!("{}", pod_dir.join("pgid").display()),
                "pgid" => podpgid);


    // Prepare mounts
    for mut m in mountpoints {
        let hostfd = try!(fs::File::open("/")).into_raw_fd();
        let canon = match m {
            rkt_stage1::AppMount::BindMount(ref mut bm) => {
                if !bm.source.exists() {
                    bail!("missing bindmount source {}", bm.source.display());
                }
                let is_dir = bm.source.is_dir();
                try!(unistd::chroot(&app_rootfs));
                try!(unistd::chdir(path::Path::new("/")));
                match (bm.target.canonicalize(), is_dir) {
                    (Ok(p), _) => p,
                    (Err(_), true) => {
                        try!(fs::create_dir_all(&bm.target));
                        bm.target.canonicalize().unwrap_or(bm.target.clone())
                    },
                    (Err(_), false) => {
                        let parent = &bm.target.parent().unwrap_or(path::Path::new("/"));
                        try!(fs::create_dir_all(parent));
                        try!(fs::File::create(&bm.target));
                        bm.target.canonicalize().unwrap_or(bm.target.clone())
                    },
                }
            }
            rkt_stage1::AppMount::Mount(ref mut em) => {
                try!(unistd::chroot(&app_rootfs));
                try!(unistd::chdir(path::Path::new("/")));
                match em.target.canonicalize() {
                    Ok(p) => p,
                    Err(_) => {
                        try!(fs::create_dir_all(&em.target));
                        em.target.canonicalize().unwrap_or(em.target.clone())
                    }
                }
            }
        };
        // Chroot back to host rootfs (via its dirfd).
        if unsafe { libc::fchdir(hostfd) } != 0 {
            bail!("fchdir failed");
        };
        try!(unistd::close(hostfd));
        try!(unistd::chroot(path::Path::new(".")));
        let suffix = match canon.is_absolute() {
            true => canon.strip_prefix("/").unwrap(),
            false => canon.as_path(),
        };
        m.set_target(app_rootfs.join(&suffix));
        try!(m.mount());
        slog_debug!(logger, "volume mounted";
                    "target" => format!("{}", m.target().display()));
    }

    // Clear FD_CLOEXEC on pod-lock and prepare app execution.
    try!(fcntl::fcntl(pod_fd_lock,
                      fcntl::FcntlArg::F_SETFD(fcntl::FdFlag::empty())));
    try!(unistd::chroot(&app_rootfs));
    try!(unistd::chdir(path::Path::new("/")));
    slog_debug!(logger, "chrooting to app rootfs";
                "path" => format!("{}", app_rootfs.display()));
    let mut cmd = process::Command::new(exec_cmd);
    cmd.env_clear()
        .env(rkt_stage1::DEFAULT_PATH.0, rkt_stage1::DEFAULT_PATH.1)
        .args(exec_args)
        .uid(uid)
        .gid(gid)
        .current_dir(workdir);
    for kv in exec_env {
        cmd.env(&kv.name, &kv.value);
    }

    return Ok(Some(cmd));
}

fn parse_mounts(empty: BTreeMap<&str, ()>,
                host: BTreeMap<&str, rkt_stage1::BindTuple>,
                mounts: Option<&Vec<appc::AppMount>>)
                -> errors::Result<Vec<rkt_stage1::AppMount>> {
    let mut res = vec![];
    // Default mounts.
    let runtime = vec![("/proc".into(), "/proc".into(), true, 0u64),
                       ("/sys".into(), "/sys".into(), true, 0u64),
                       ("/dev".into(), "/dev".into(), true, 0u64)];
    for i in runtime {
        res.push(rkt_stage1::AppMount::from(i));
    }

    // Application mounts.
    if let Some(ref ms) = mounts {
        for m in ms.iter() {
            if let Some(ref v) = m.appVolume {
                match v.kind {
                    appc::VolumeKind::Empty => {
                        let emptymount = (path::PathBuf::from("tmpfs"),
                                          "tmpfs".to_string(),
                                          m.path.to_path_buf(),
                                          false,
                                          0u64,
                                          vec![]);
                        res.push(rkt_stage1::AppMount::from(emptymount));
                    }
                    appc::VolumeKind::Host => {
                        let source = v.source.clone().unwrap_or(path::PathBuf::from(""));
                        let recursive = v.recursive.clone().unwrap_or(false);
                        let bind = (source, m.path.to_path_buf(), recursive, 0u64);
                        res.push(rkt_stage1::AppMount::from(bind));
                    }
                };
            } else if let Some(ref _k) = empty.get(m.volume.as_str()) {
                let emptymount = (path::PathBuf::from("tmpfs"),
                                  "tmpfs".to_string(),
                                  m.path.to_path_buf(),
                                  false,
                                  0u64,
                                  vec![]);
                res.push(rkt_stage1::AppMount::from(emptymount));
            } else if let Some(k) = host.get(m.volume.as_str()) {
                let hostmount = (k.0.to_path_buf(), m.path.to_path_buf(), k.2, k.3);
                res.push(rkt_stage1::AppMount::from(hostmount));
            } else {
                bail!("missing volume {}", m.volume);
            };
        }
    };
    return Ok(res);
}
