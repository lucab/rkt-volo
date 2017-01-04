use rkt_stage1;
use clap;
use nix;
use log;
use envy;
use serde_json;
error_chain!{
    links {
        Stage1(rkt_stage1::Error, rkt_stage1::ErrorKind);
    }

    foreign_links {
        Cli(clap::Error);
        Env(envy::Error);
        Io(::std::io::Error);
        Json(serde_json::Error);
        Logger(log::SetLoggerError);
        Posix(nix::Error);
        Int(::std::num::ParseIntError);
    }
}
