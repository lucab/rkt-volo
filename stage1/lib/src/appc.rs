// appc types and helpers

use errors;
include!(concat!(env!("OUT_DIR"), "/appc_pod.rs"));

use std::{fs, io, path};
use nix::sys::stat;
use parsswd;

use std::io::BufRead;

pub fn resolve_uid(uid_field: &str, rootfs: &path::Path) -> errors::Result<u32> {
    let passwd = path::PathBuf::from(rootfs).join("etc").join("passwd");
    if let Ok(pfile) = fs::File::open(passwd) {
        for line in io::BufReader::new(pfile).lines() {
            let l = try!(line);
            if let Some(entry) = parsswd::PwEnt::from_str(&l) {
                if entry.name == uid_field {
                    return Ok(entry.uid);
                }
            }
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
    if let Ok(gfile) = fs::File::open(groups) {
        for line in io::BufReader::new(gfile).lines() {
            let l = try!(line);
            if let Some(entry) = parsswd::GrpEnt::from_str(&l) {
                if entry.name == gid_field {
                    return Ok(entry.gid);
                }
            }
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
        return Ok(filestat.st_gid);
    }

    bail!("unable to determine gid");
}
