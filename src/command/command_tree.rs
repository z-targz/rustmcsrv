use std::{
    collections::HashSet,
    hash::Hash
};

use super::argument::Argument;



#[derive(Clone, Debug, Eq)]
pub struct CommandNode {
    arg: Argument,
    children: Vec<CommandNode>,
}




impl PartialEq for CommandNode {
    fn eq(&self, other: &Self) -> bool {
        self.arg == other.arg && self.arg.is_last() == other.arg.is_last()
    }
}

impl Hash for CommandNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.arg.hash(state);
    }
}

impl CommandNode {
    pub fn single_usage(label: &str, args: &[&Argument]) -> Self {

    }

    pub fn get_argument(&self) -> &Argument {
        &self.arg
    }

    pub fn get_children(&self) -> &[CommandNode] {
        self.children.as_slice()
    }

    pub fn get_expecting(&self) -> Vec<&Argument> {
        self.children.iter()
            .map(|child| child.get_argument()).collect()
    }

    pub fn add_child(&mut self, child: CommandNode) -> bool {
        match self.children.iter().position(|p| p == &child) {
            Some(idx) => {
                self.children.get_mut(idx).unwrap().add_child(child);
                true
            },
            None => {
                self.children.push(child);
                false
            },
        }
    }
}

