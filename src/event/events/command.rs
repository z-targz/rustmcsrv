use crate::{command::{Command, CommandSender}, event::TraitEvent};

#[derive(Debug, Clone)]
pub struct CommandEvent {
    sender: CommandSender,
    command: Command,
    label: String,
    args: Vec<String>,
}

impl CommandEvent {
    pub fn new(sender: CommandSender, command: Command, label: String, args: Vec<String>) -> Self {
        Self { sender, command, label, args }
    }
    
    pub fn get_sender(&self) -> &CommandSender {
        &self.sender
    }
    
    pub fn get_command(&self) -> &Command {
        &self.command
    }
    
    pub fn get_label(&self) -> &str {
        &self.label
    }
    
    pub fn get_args(&self) -> &[String] {
        &self.args
    }
}

impl TraitEvent for CommandEvent {}