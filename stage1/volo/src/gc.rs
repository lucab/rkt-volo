use errors;
use rkt_stage1::cli;
use slog;
use std::process;
use uuid;

pub fn volo_gc(logger: slog::Logger) -> errors::Result<Option<process::Command>> {

    // Parse all command-line flags.
    let matches = try!(cli::gc_flags().get_matches_safe());
    let pod_uuid = try!(value_t!(matches, "uuid", uuid::Uuid));
    slog_debug!(logger, "no gc actions to perform";
                "pod UUID" => pod_uuid.hyphenated().to_string());

    return Ok(None);
}
