// appc types and helpers

use errors;
include!(concat!(env!("OUT_DIR"), "/appc_pod.rs"));

use std::{fs, path};

pub fn resolve_uid(uid_field: &str, rootfs: &path::Path) -> errors::Result<u32> {
    let passwd = path::PathBuf::from(rootfs).join("etc").join("passwd");
    if let Ok(_f) = fs::File::open(passwd) {
        if uid_field == "root" {
            return Ok(0);
        }
    }

    let tf = path::PathBuf::from(uid_field);
    if tf.is_absolute() {
        bail!("unimplemented");
    }

    if let Ok(uid) = uid_field.parse::<u32>() {
        return Ok(uid);
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

    let tf = path::PathBuf::from(gid_field);
    if tf.is_absolute() {
        bail!("unimplemented");
    }

    if let Ok(gid) = gid_field.parse::<u32>() {
        return Ok(gid);
    }

    bail!("unable to determine gid");
}
