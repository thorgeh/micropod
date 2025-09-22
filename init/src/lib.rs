use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: PathBuf,
    pub layers: Vec<(String, PathBuf)>,
    pub binds: Vec<(PathBuf, PathBuf)>,
    pub mount_proc: bool,
    pub mount_tmp: bool,
    pub mount_sys: bool,
    pub mount_dev: bool,
    pub command: Command,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
}
