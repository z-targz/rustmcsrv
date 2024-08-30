use std::{
    collections::HashSet,
    hash::Hash
};

use super::argument::Argument;

#[derive(Clone, Debug)]
pub struct CommandUsage {
    args: HashSet<CommandNode>,
}


impl CommandUsage {
    pub fn new() -> Self {
        Self {
            args: HashSet::new()
        }
    }

    pub fn add_usage(&mut self, usage: &[Argument]) {
        for arg in usage {
            if self.args.contains(arg) {

            }
        }
    }
}

#[derive(Clone, Debug, Eq)]
pub struct CommandNode {
    arg: Argument,
    children: HashSet<CommandNode>,
}

impl PartialEq for CommandNode {
    fn eq(&self, other: &Self) -> bool {
        self.arg == other.arg
    }
}

impl Hash for CommandNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.arg.hash(state);
    }
}

impl CommandNode {

}