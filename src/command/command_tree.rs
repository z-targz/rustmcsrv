use std::{
    hash::Hash, 
    ptr::NonNull
};

use super::argument::{ArgType, Argument};



#[derive(Clone, Debug, Eq)]
pub struct CommandNode {
    last: bool,
    arg: Argument,
    children: Vec<CommandNode>,
}




impl PartialEq for CommandNode {
    fn eq(&self, other: &Self) -> bool {
        self.arg == other.arg && self.is_last() == other.is_last()
    }
}

impl Hash for CommandNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.arg.hash(state);
    }
}

impl FromIterator<Argument> for Option<CommandNode> {
    fn from_iter<I: IntoIterator<Item = Argument>>(iter: I) -> Self {
        
        struct CommandNodeList {
            head: Option<CommandNode>,
            tail: Option<NonNull<CommandNode>>,
        }

        impl CommandNodeList {
            fn new() -> Self {
                Self {
                    head: None, tail: None,
                }
            }

            fn root(self) -> Option<CommandNode> {
                self.head
            }

            fn extend(&mut self, mut node: CommandNode) {
                if let Some(mut tail) = self.tail {
                    unsafe {    
                        let node_ptr = &mut node as *mut CommandNode;
                        tail.as_mut().add_child(node);
                        self.tail = NonNull::new(node_ptr);
                    }
                } else {
                    let node_ptr = &mut node as *mut CommandNode;
                    self.head = Some(node);
                    self.tail = NonNull::new(node_ptr);
                }
            }
        }

        let mut into_iter = iter.into_iter().peekable();

        let mut list = CommandNodeList::new();

        while let Some(arg) = into_iter.next() {
            list.extend(CommandNode {
                last: into_iter.peek().is_none(),
                arg: arg,
                children: vec![]
            })
        }
        list.root()
    }
}

impl CommandNode {
    pub fn is_last(&self) -> bool {
        self.last
    }
    pub fn single_usage(label: &str, args: &[Argument]) -> Self {
        Self {
            last: args.len() > 0,
            arg: Argument::new(
                label.to_owned(), 
                ArgType::Command { label: label.to_string() }, 
                true
            ),
            children: 
                if let Some(x) = args.iter().cloned().collect() { 
                    vec![x]
                } else {
                    vec![]
                }
        }
    }

    pub fn get_argument(&self) -> &Argument {
        &self.arg
    }

    pub fn get_children(&self) -> &[CommandNode] {
        self.children.as_slice()
    }

    pub fn get_child(&self, child: &Argument) -> Option<&CommandNode> {
        self.children.iter().find(|ch| ch.get_argument() == child)
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

