#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use(log,error,debug)]
extern crate log;
#[macro_use(slog_log,slog_debug,slog_info)]
extern crate slog;
extern crate envy;
extern crate libc;
extern crate nix;
extern crate serde_json;
extern crate slog_stdlog;
extern crate slog_term;
extern crate uuid;
extern crate rkt_stage1;

mod gc;
mod errors;
mod enter;
mod run;
mod stop;

use slog::{DrainExt, Logger};
use std::os::unix::process::CommandExt;
use std::process;

fn main() {
    // Call exec() in main, so destructors can run.
    fn runtime_exec(cmd: Option<process::Command>) -> errors::Result<()> {
        return match cmd {
            Some(mut c) => Err(errors::Error::from(c.exec())),
            None => Ok(()),
        };
    };

    // Single exit-point and error printing.
    let exitcode = match volo_main().and_then(runtime_exec) {
        Ok(_) => 0,
        Err(e) => {
            error!("stage1-volo: {}", e);
            254
        }
    };

    std::process::exit(exitcode);
}

fn volo_main() -> errors::Result<Option<process::Command>> {

    // Initialize logger.
    let loglevel = match std::env::args().any(|a| a == "--debug") {
        false => slog::Level::Info,
        true => slog::Level::Debug,
    };
    let stderr = slog::LevelFilter::new(slog_term::streamer().stderr().build().fuse(), loglevel);
    let logger = Logger::root(stderr, None);
    try!(slog_stdlog::set_logger(logger.clone()));
    slog_debug!(logger, "stage1-volo";
                "version" => crate_version!());

    // Dispatch to sub-action based on executable name
    let exe_path = {
        let p = try!(std::env::args().nth(0).ok_or("missing exec path"));
        std::path::PathBuf::from(p)
    };
    let exe_name = try!(exe_path.file_name().ok_or("missing exec name"));
    slog_debug!(logger, "stage1-volo entrypoint";
                "exe path" => format!("{}", exe_path.display()));

    return match exe_name.to_str() {
        Some("enter") => enter::volo_enter(logger),
        Some("gc") => gc::volo_gc(logger),
        Some("run") => run::volo_run(logger),
        Some("stop") => stop::volo_stop(logger),
        Some(s) => bail!("unknown entrypoint {}", s),
        None => bail!("missing entrypoint name"),
    };
}
