#![warn(clippy::pedantic)]
#![allow(dead_code)] // while developing

mod error;
mod run;

use std::{collections::HashMap, path::PathBuf};

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Pod {
    path: PathBuf,
    layers: HashMap<String, PathBuf>,
    binds: HashMap<PathBuf, PathBuf>,
    mounts: Mounts,
}

impl Pod {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            layers: HashMap::new(),
            binds: HashMap::new(),
            mounts: Mounts::empty(),
        }
    }

    #[must_use]
    pub fn layer(mut self, name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        self.layers.insert(name.into(), path.into());
        self
    }

    #[must_use]
    pub fn mount_proc(mut self) -> Self {
        self.mounts |= Mounts::Proc;
        self
    }

    #[must_use]
    pub fn mount_tmp(mut self) -> Self {
        self.mounts |= Mounts::Tmp;
        self
    }

    #[must_use]
    pub fn mount_sys(mut self) -> Self {
        self.mounts |= Mounts::Sys;
        self
    }

    #[must_use]
    pub fn mount_dev(mut self) -> Self {
        self.mounts |= Mounts::Dev;
        self
    }
}

bitflags::bitflags! {
    pub struct Mounts: u8 {
        const Proc = 1;
        const Tmp = 1 << 1;
        const Sys = 1 << 2;
        const Dev = 1 << 3;
    }
}

pub struct Command {
    cmd: String,
    args: Vec<String>,
    env: HashMap<String, String>,
}

impl Command {
    pub fn new(cmd: impl Into<String>) -> Self {
        Self {
            cmd: cmd.into(),
            args: Vec::new(),
            env: HashMap::new(),
        }
    }

    #[must_use]
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    #[must_use]
    pub fn env(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(name.into(), value.into());
        self
    }
}
