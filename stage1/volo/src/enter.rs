use slog;
use std::{path, process};
use nix::unistd;
use rkt_stage1::cli;
use errors;

pub fn volo_enter(logger: slog::Logger) -> errors::Result<Option<process::Command>> {

    // Parse all command-line flags.
    let matches = try!(cli::enter_flags().get_matches_safe());
    let app_name = try!(value_t!(matches, "appname", String));
    let exec = try!(values_t!(matches, "exec", String));
    let (exec_cmd, exec_args) = try!(exec.split_first().ok_or("empty exec entrypoint".to_string()));
    slog_debug!(logger, "stage1-volo enter entrypoint";
                "appname" => app_name,
                "exec cmd" => exec_cmd.to_string(),
                "exec args" => format!("{:?}", exec_args));

    // Prepare execution.
    let pod_dir = {
        let d = try!(unistd::getcwd());
        try!(d.canonicalize())
    };

    let app_rootfs = pod_dir.join("stage1")
        .join("rootfs")
        .join("opt")
        .join("stage2")
        .join(app_name)
        .join("rootfs");
    try!(unistd::chroot(&app_rootfs));
    try!(unistd::chdir(path::Path::new("/")));
    slog_debug!(logger, "chrooting to app rootfs";
                "path" => format!("{}", app_rootfs.display()));

    let mut cmd = process::Command::new(exec_cmd);
    cmd.env_clear()
        .args(&exec_args)
        .current_dir(path::Path::new("/"));
    return Ok(Some(cmd));
}
