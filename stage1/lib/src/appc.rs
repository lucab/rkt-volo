// appc types and helpers

use errors;
include!(concat!(env!("OUT_DIR"), "/appc_pod.rs"));

use std::{fs, path};
use nix::sys::stat;

pub fn resolve_uid(uid_field: &str, rootfs: &path::Path) -> errors::Result<u32> {
    let passwd = path::PathBuf::from(rootfs).join("etc").join("passwd");
    if let Ok(_f) = fs::File::open(passwd) {
        if uid_field == "root" {
            return Ok(0);
        }
    }

    if let Ok(uid) = uid_field.parse::<u32>() {
        return Ok(uid);
    }

    let tf = path::PathBuf::from(uid_field);
    if tf.is_absolute() {
        let rel_path = try!(tf.strip_prefix("/"));
        let abs_path = path::PathBuf::from(rootfs).join(rel_path);
        let filestat = try!(stat::lstat(&abs_path));
        return Ok(filestat.st_uid);
    }

    bail!("unable to determine uid");
}

pub fn resolve_gid(gid_field: &str, rootfs: &path::Path) -> errors::Result<u32> {
    let groups = path::PathBuf::from(rootfs).join("etc").join("group");
    if let Ok(_f) = fs::File::open(groups) {
        if gid_field == "root" {
            return Ok(0);
        }
    }

    if let Ok(gid) = gid_field.parse::<u32>() {
        return Ok(gid);
    }

    let tf = path::PathBuf::from(gid_field);
    if tf.is_absolute() {
        let rel_path = try!(tf.strip_prefix("/"));
        let abs_path = path::PathBuf::from(rootfs).join(rel_path);
        let filestat = try!(stat::lstat(&abs_path));
        return Ok(filestat.st_uid);
    }

    bail!("unable to determine gid");
}
