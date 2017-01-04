use libc;
use slog;
use std::{fs, process};
use nix::{sys, unistd};
use rkt_stage1::cli;
use errors;

use std::io::Read;

pub fn volo_stop(logger: slog::Logger) -> errors::Result<Option<process::Command>> {
    // Parse all command-line flags.
    let matches = try!(cli::stop_flags().get_matches_safe());
    let signal = match matches.is_present("force") {
        false => sys::signal::Signal::SIGTERM,
        true => sys::signal::Signal::SIGKILL,
    };

    // Get pod pid (leader), and signal it.
    let pod_dir = {
        let d = try!(unistd::getcwd());
        try!(d.canonicalize())
    };
    let pod_pid = {
        let mut buf = String::new();
        let mut f = try!(fs::File::open(pod_dir.join("pid")));
        try!(f.read_to_string(&mut buf));
        try!(buf.parse::<libc::pid_t>())
    };

    slog_debug!(logger, "sending signal to process";
                "pid" => pod_pid,
                "signal" => format!("{:?}", signal));
    let pid_res = sys::signal::kill(pod_pid, signal);
    if pid_res.is_ok() {
        return Ok(None);
    }

    // If pod leader is missing, get pod pgid and signal it.
    let pod_pgid = {
        let mut buf = String::new();
        let mut f = try!(fs::File::open(pod_dir.join("pgid")));
        try!(f.read_to_string(&mut buf));
        try!(buf.parse::<libc::pid_t>())
    };
    slog_info!(logger, "unable to signal process, re-trying with group";
               "pid" => pod_pid,
               "pgid" => pod_pgid,
                "signal" => format!("{:?}", signal));
    let pgid_res = sys::signal::kill(pod_pgid.wrapping_neg(), signal);
    if pgid_res.is_ok() {
        return Ok(None);
    }

    bail!("unable to stop pod");
}
