use std::collections::HashMap;

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

    pub fn get_cmd(&self) -> &str {
        &self.cmd
    }

    pub fn get_args(&self) -> &[String] {
        &self.args
    }

    pub fn get_env(&self) -> &HashMap<String, String> {
        &self.env
    }
}
