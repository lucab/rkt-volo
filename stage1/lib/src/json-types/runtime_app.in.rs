extern crate libc;
extern crate nix;

use std::path::PathBuf;
use nix::mount::MsFlags;
use std::convert::Into;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BindMount {
    pub source: std::path::PathBuf,
    pub target: std::path::PathBuf,
    pub recursive: bool,
    pub other_flags: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Mount {
    pub source: std::path::PathBuf,
    pub fstype: String,
    pub target: std::path::PathBuf,
    pub propagation: Propagation,
    pub recursive: bool,
    pub other_flags: u64,
    pub data: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AppMount {
    Mount(Mount),
    BindMount(BindMount),
}

pub type BindTuple = (PathBuf, PathBuf, bool, u64);
impl From<BindTuple> for AppMount {
    fn from(v: BindTuple) -> Self {
        let b = BindMount {
            source: v.0,
            target: v.1,
            recursive: v.2,
            other_flags: v.3,
        };
        return AppMount::BindMount(b);
    }
}

pub type MountTuple = (PathBuf, String, PathBuf, Propagation, bool, u64, Vec<String>);
impl From<MountTuple> for AppMount {
    fn from(v: MountTuple) -> Self {
        let m = Mount {
            source: v.0,
            fstype: v.1,
            target: v.2,
            propagation: v.3,
            recursive: v.4,
            other_flags: v.5,
            data: v.6,
        };
        return AppMount::Mount(m);
    }
}
impl AppMount {
    pub fn mount(&self) -> nix::Result<&Self> {
        let r = match *self {
            AppMount::Mount(ref x) => {
                nix::mount::mount(Some(&x.source),
                                  &x.target,
                                  Some(x.fstype.as_str()),
                                  {
                                      let mut f = MsFlags::from_bits_truncate(x.other_flags);
                                      f.insert(x.propagation.clone().into());
                                      if x.recursive {
                                          f.insert(nix::mount::MS_REC);
                                      };
                                      f
                                  },
                                  Some(x.data.join(",").as_str()))
            }
            AppMount::BindMount(ref x) => {
                nix::mount::mount(Some(&x.source),
                                  &x.target,
                                  None::<&str>,
                                  {
                                      let mut f = MsFlags::from_bits_truncate(x.other_flags);
                                      f.insert(nix::mount::MS_BIND);
                                      if x.recursive {
                                          f.insert(nix::mount::MS_REC);
                                      };
                                      f
                                  },
                                  None::<&str>)
            }
        };
        return r.and(Ok(self));
    }
    pub fn set_propagation(&self) -> nix::Result<&Self> {
        return Ok(self);
    }
    pub fn target(&self) -> &PathBuf {
        return match *self {
            AppMount::Mount(ref x) => &x.target,
            AppMount::BindMount(ref x) => &x.target,
        };
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[repr(u64)]
pub enum Propagation {
    Shared = libc::MS_SHARED,
    Private = libc::MS_PRIVATE,
    Slave = libc::MS_SLAVE,
    Unbindable = libc::MS_UNBINDABLE,
}

impl Into<MsFlags> for Propagation {
    fn into(self) -> MsFlags {
        MsFlags::from_bits_truncate(self as u64)
    }
}

impl std::fmt::Display for Propagation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            Propagation::Private {} => "private",
            Propagation::Shared {} => "shared",
            Propagation::Slave {} => "shared",
            Propagation::Unbindable {} => "unbindable",
        };
        return write!(f, "{}", s);
    }
}

impl std::fmt::Debug for Propagation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            Propagation::Private {} => "MS_PRIVATE",
            Propagation::Shared {} => "MS_SHARED",
            Propagation::Slave {} => "MS_SLAVE",
            Propagation::Unbindable {} => "MS_UNBINDABLE",
        };
        return write!(f, "{}", s);
    }
}
