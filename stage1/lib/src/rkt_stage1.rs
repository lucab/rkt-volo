// rkt_stage1

#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate parsswd;
extern crate semver;
extern crate sha2;

pub use clap::App;

pub mod cli;
pub mod errors;
pub mod appc;

pub use errors::*;

pub const DEFAULT_PATH: (&'static str, &'static str) =
    ("PATH", "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin");

include!(concat!(env!("OUT_DIR"), "/runtime_pod.rs"));
include!(concat!(env!("OUT_DIR"), "/runtime_app.rs"));
