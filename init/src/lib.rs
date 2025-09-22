use std::{collections::HashMap, path::PathBuf};

use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub struct Config {
    pub path: PathBuf,
    pub layers: HashMap<String, PathBuf>,
    pub binds: HashMap<PathBuf, PathBuf>,
    pub mount_proc: bool,
    pub mount_tmp: bool,
    pub mount_sys: bool,
    pub mount_dev: bool,
}
