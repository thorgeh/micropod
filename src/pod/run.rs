use std::{
    ffi::CString,
    io::{PipeReader, pipe},
    os::fd::AsRawFd,
};

use nix::{
    libc::SIGCHLD,
    sched::CloneFlags,
    sys::wait::waitpid,
    unistd::{getgid, getuid},
};

use crate::{Command, Pod};

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
            mount_proc: self.mounts.proc,
            mount_tmp: self.mounts.tmp,
            mount_sys: self.mounts.sys,
            mount_dev: self.mounts.dev,
            command: micropod_init::Command {
                cmd: cmd.get_cmd().to_string(),
                args: cmd.get_args().to_vec(),
                env: cmd
                    .get_env()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<Vec<_>>(),
            },
        };

        let config_read = create_config_pipe(&config)?;
        let (error_read, error_write) = pipe()?;

        let env = [
            CString::new(format!("CONFIG_PIPE={}", config_read.as_raw_fd())).unwrap(),
            CString::new(format!("ERROR_PIPE={}", error_write.as_raw_fd())).unwrap(),
        ];

        let init_fd = crate::init::get_init_fd()?;

        let child = Box::new(|| {
            let _ = std::fs::write("/proc/self/setgroups", "deny");
            let _ = std::fs::write("/proc/self/uid_map", &uid_map);
            let _ = std::fs::write("/proc/self/gid_map", &gid_map);

            let _ = nix::unistd::fexecve(&init_fd, &[c"init"], &env);
            0
        });

        let pid = unsafe { nix::sched::clone(child, &mut stack, flags, Some(SIGCHLD)) }
            .map_err(crate::Error::Clone)?;

        loop {
            match waitpid(pid, None).map_err(crate::Error::Wait)? {
                nix::sys::wait::WaitStatus::Exited(_, _) => break,
                nix::sys::wait::WaitStatus::Signaled(_, _, _) => break,
                _ => continue,
            }
        }

        Ok(())
    }
}

fn create_config_pipe(config: &micropod_init::Config) -> crate::Result<PipeReader> {
    let (config_read, mut config_write) = pipe()?;

    bincode::serde::encode_into_std_write(config, &mut config_write, bincode::config::standard())?;

    Ok(config_read)
}
