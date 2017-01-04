extern crate clap;
use clap::Arg;

use errors;

// public types
include!(concat!(env!("OUT_DIR"), "/stage1_cli.rs"));

pub fn run_flags<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("run")
        .arg(Arg::with_name("uuid"))
        .arg(Arg::with_name("debug")
             .long("debug"))
        .arg(Arg::with_name("net")
             .long("net")
             .takes_value(true)
             .value_delimiter(","))
        .arg(Arg::with_name("mds-token")
             .long("mds-token")
             .takes_value(true))
        .arg(Arg::with_name("local-config")
             .long("local-config")
             .takes_value(true))
        .arg(Arg::with_name("private-users")
             .long("private-users")
             .takes_value(true))
        .arg(Arg::with_name("interactive")
             .long("interactive"))
    // from v2
        .arg(Arg::with_name("hostname")
             .long("hostname")
             .takes_value(true))
    // from v3
        .arg(Arg::with_name("disable-paths")
             .long("disable-paths"))
        .arg(Arg::with_name("disable-seccomp")
             .long("disable-seccomp"))
        .arg(Arg::with_name("disable-capabilities-restriction")
             .long("disable-capabilities-restriction"))
    // from v4
        .arg(Arg::with_name("dns-conf-mode")
             .long("dns-conf-mode")
             .takes_value(true))
    // from v5
        .arg(Arg::with_name("mutable")
             .long("mutable"))
}

pub fn enter_flags<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("enter")
        .setting(clap::AppSettings::TrailingVarArg)
        .arg(Arg::with_name("pid")
            .long("pid")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("appname")
            .long("appname")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("exec")
            .multiple(true)
            .required(true)
            .min_values(1))
}

pub fn gc_flags<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("gc")
        .arg(Arg::with_name("uuid"))
        .arg(Arg::with_name("debug")
             .long("debug"))
    // from v5
        .arg(Arg::with_name("local-config")
             .long("local-config")
             .takes_value(true))
}

pub fn stop_flags<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("stop")
        .arg(Arg::with_name("force").long("force"))
        .arg(Arg::with_name("uuid"))
}

pub enum DnsResolv {
    Default,
    Host,
    Stage0,
    None,
}

pub enum DnsHosts {
    Default,
    Host,
    Stage0,
}

pub struct RunDnsConf {
    pub resolv: DnsResolv,
    pub hosts: DnsHosts,
}

impl Default for RunDnsConf {
    fn default() -> Self {
        return RunDnsConf {
            resolv: DnsResolv::Default,
            hosts: DnsHosts::Default,
        };
    }
}

impl ::std::str::FromStr for RunDnsConf {
    type Err = errors::Error;
    fn from_str(s: &str) -> errors::Result<Self> {
        let parts: Vec<&str> = s.splitn(2, ",").collect();

        // Mode for /etc/resolv.conf
        let rkv: Vec<&str> = parts[0].splitn(2, "=").collect();
        let r = match rkv[1] {
            "host" => DnsResolv::Host,
            "stage0" => DnsResolv::Stage0,
            "none" => DnsResolv::None,
            _ => DnsResolv::Default,
        };

        // Mode for /etc/hosts
        let hkv: Vec<&str> = parts[1].splitn(2, "=").collect();
        let h = match hkv[1] {
            "host" => DnsHosts::Host,
            "stage0" => DnsHosts::Stage0,
            _ => DnsHosts::Default,
        };

        return Ok(RunDnsConf {
            resolv: r,
            hosts: h,
        });
    }
}
