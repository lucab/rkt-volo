extern crate libc;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunEnvFlags {
    pub rkt_lock_fd : ::std::os::unix::io::RawFd,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrossingEnvFlags {
    pub rkt_stage1_entercmd : ::std::path::PathBuf,
    pub rkt_stage1_enterpid : libc::pid_t,
    pub rkt_stage1_enterapp : Option<String>,
}
