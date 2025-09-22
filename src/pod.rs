mod command;
mod run;

use std::path::PathBuf;

pub use command::Command;

pub struct Pod {
    path: PathBuf,
    layers: Vec<(String, PathBuf)>,
    binds: Vec<(PathBuf, PathBuf)>,
    mounts: Mounts,
}

#[derive(Default)]
pub(crate) struct Mounts {
    tmp: bool,
    proc: bool,
    sys: bool,
    dev: bool,
}

impl Pod {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            layers: Vec::new(),
            binds: Vec::new(),
            mounts: Mounts::default(),
        }
    }

    #[must_use]
    pub fn layer(mut self, name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        self.layers.push((name.into(), path.into()));
        self
    }

    #[must_use]
    pub fn mount_proc(mut self) -> Self {
        self.mounts.proc = true;
        self
    }

    #[must_use]
    pub fn mount_tmp(mut self) -> Self {
        self.mounts.tmp = true;
        self
    }

    #[must_use]
    pub fn mount_sys(mut self) -> Self {
        self.mounts.sys = true;
        self
    }

    #[must_use]
    pub fn mount_dev(mut self) -> Self {
        self.mounts.dev = true;
        self
    }
}
