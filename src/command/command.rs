use std::error::Error;
use std::fmt::Display;
use std::sync::Weak;

use std::collections::HashMap;


use crate::{event::{self, events::command::CommandEvent, EventResult, TraitEvent}, player::Player, CONSOLE, RUNTIME, THE_SERVER};

#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    description: Option<String>,
    usage: String,
    aliases: Vec<String>,
    permission: Option<String>,
    func: fn(CommandEvent) -> EventResult,
}

impl Command {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn get_usage(&self) -> &str {
        &self.usage
    }

    pub fn get_aliases(&self) -> &[String] {
        &self.aliases
    }

    pub fn get_permission(&self) -> Option<&String> {
        self.permission.as_ref()
    }

    pub fn get_func(&self) -> fn(CommandEvent) -> EventResult {
        self.func
    }
}

impl Command {
    pub fn new(
        name: String, 
        description: Option<String>, 
        usage: String, 
        permission: Option<String>, 
        aliases: Vec<String>, 
        func: fn(CommandEvent) -> EventResult,
    ) -> Self {
        Self {
            name: name,
            description: description,
            usage: usage,
            aliases: aliases,
            permission: permission,
            func: func,
        }
    }

    pub fn execute(e: CommandEvent) {
        crate::RUNTIME.spawn(run_command(e));
    }
}

pub trait TraitCommand {
    fn execute(&self, event: CommandEvent) -> bool;
}

#[derive(Debug, Clone)]
pub enum CommandSender {
    Console,
    Player {
        player: Weak<Player>,
    },
}

impl CommandSender {
    pub fn get_name(&self) -> Option<String> {
        match self {
            CommandSender::Console => Some("@".to_owned()),
            CommandSender::Player { player } => 
            match player.upgrade() {
                Some(arc) => Some(arc.get_name().clone()),
                None => None,
            }
        }
    }

    pub fn send_message(&self, message: String) {
        match self {
            CommandSender::Console => {RUNTIME.spawn(CONSOLE.get().unwrap().println(message));},
            CommandSender::Player { player } => {
                match player.upgrade() {
                    Some(arc) => {RUNTIME.spawn(async move { arc.send_message(message).await });},
                    None => (),
                };
                
            },
        }
    }
}



pub struct CommandMapBuilder {
    commands: Vec<Command>,
}

impl CommandMapBuilder {
    pub fn new() -> Self {
        Self {
            commands: vec![
                Command::new(
                    "stop".to_string(), 
                    Some("Stops the server.".to_string()), 
                    "stop".to_string(), 
                    Some("stop".to_string()), 
                    vec![],
                    command_stop
                ),
            ],
        }
    }

    pub fn register_command(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    pub fn build(self) -> CommandMap {
        let mut command_hashmap = HashMap::with_capacity(self.commands.len());
        let mut aliases_hashmap = HashMap::new();
        for command in self.commands {
            for alias in command.aliases.iter() {
                aliases_hashmap.insert(alias.clone(), command.name.clone());
            }
            command_hashmap.insert(command.name.clone(), command);
        }
        CommandMap {
            commands: command_hashmap,
            aliases: aliases_hashmap
        }
    }
}

async fn run_command(e: CommandEvent) -> EventResult {

    async fn run(evt: CommandEvent) -> EventResult {
        match event::listen(THE_SERVER.get_event_manager(), &evt).await {
            EventResult::Deny => EventResult::Deny,
            _ => (evt.get_command().func)(evt)
        }
    }

    match e.get_sender() {
        CommandSender::Console => {
            run(e).await
        },
        CommandSender::Player { player } => {
            match player.upgrade() {
                Some(arc) => {
                    match &e.get_command().permission {
                        Some(permission) => {
                            if arc.has_permission(permission.as_str()) {
                                run(e).await
                            } else {
                                return EventResult::Deny;
                            }
                        },
                        None => run(e).await
                    }
                    
                },
                None => EventResult::Deny,
            }
        },
    }
}

fn command_stop(_: CommandEvent) -> EventResult {
    let _ = crate::STOP_SIGNAL.get().unwrap().send(true);
    EventResult::Default
}

fn command_reload(_: CommandEvent) -> EventResult {
    let _ = crate::STOP_SIGNAL.get().unwrap().send(false);
    EventResult::Default
}

pub struct CommandMap {
    commands: HashMap<String, Command>,
    aliases: HashMap<String, String>,
}

impl CommandMap {
    pub fn get_command(&self, label: &str) -> Option<&Command> {
        match self.commands.get(label) {
            Some(cmd) => Some(cmd),
            None => {
                match self.aliases.get(label) {
                    Some(cmd) => self.commands.get(cmd),
                    None => None,
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum CommandError {
    EventCancelled,
    PermissionError,
    InvalidUsage { command: Command },
    Other { message: String },
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::EventCancelled => f.write_fmt(format_args!("CommandEvent cancelled")),
            CommandError::PermissionError => f.write_fmt(format_args!("You do not have permission to execute this command.")),
            CommandError::InvalidUsage { command } => f.write_fmt(format_args!("Incorrect arguments for {}. Usage: /{}", command.name, command.usage)),
            CommandError::Other { message } => f.write_fmt(format_args!("{message}")),
        }
        
    }
}