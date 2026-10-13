use std::ffi::OsString;
use std::process::Command;

pub enum ChildVariable {
    Removed(&'static str),
    Set { name: &'static str, value: OsString },
}

impl ChildVariable {
    pub(crate) fn apply(&self, command: &mut Command) {
        match self {
            Self::Removed(name) => command.env_remove(name),
            Self::Set { name, value } => command.env(name, value),
        };
    }
}
