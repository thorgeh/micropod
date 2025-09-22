use std::{
    fs::{File, OpenOptions},
    io::{PipeReader, pipe},
    os::fd::OwnedFd,
    path::Path,
};

use nix::{
    sched::CloneFlags,
    unistd::{getgid, getuid},
};

use crate::{Command, Mounts, Pod};

impl Pod {
    /// runs the given command in the pod
    ///
    /// # Errors
    /// TODO
    #[expect(unused_variables)]
    pub fn run(&self, cmd: &Command) -> crate::Result<()> {
        std::fs::create_dir_all(&self.path)?;

        let mut stack = vec![0; 1024 * 1024];

        let flags = CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWCGROUP
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWUSER
            | CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNET;

        let uid_map = format!("0 {} 1", getuid());
        let gid_map = format!("0 {} 1", getgid());

        let config = micropod_init::Config {
            path: self.path.clone(),
            layers: self.layers.clone(),
            binds: self.binds.clone(),
            mount_proc: self.mounts.contains(Mounts::Proc),
            mount_tmp: self.mounts.contains(Mounts::Tmp),
            mount_sys: self.mounts.contains(Mounts::Sys),
            mount_dev: self.mounts.contains(Mounts::Dev),
        };

        let config_read = create_config_pipe(&config)?;

        Ok(())
    }
}

fn create_config_pipe(config: &micropod_init::Config) -> crate::Result<PipeReader> {
    let (config_read, mut config_write) = pipe()?;

    bincode::encode_into_std_write(config, &mut config_write, bincode::config::standard())?;

    Ok(config_read)
}
