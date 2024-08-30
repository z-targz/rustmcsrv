use std::str::{FromStr, ParseBoolError};
use std::{error::Error, fmt::Debug};
use std::fmt::{Arguments, Display};
use std::sync::Weak;

use std::collections::{HashMap, HashSet, VecDeque};


use itertools::Itertools;
use yaml_rust::scanner::Token;

use crate::COMMAND_MAP;
use crate::{data_types::{Angle, Identifier}, event::{self, events::command::CommandEvent, EventResult}, player::Player, CONSOLE, RUNTIME, THE_SERVER};

use super::argument::Argument;
use super::command_tree::{CommandNode};

#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    description: Option<String>,
    usages: CommandNode,
    aliases: Vec<String>,
    permission: Option<String>,
    func: fn(&mut CommandEvent) -> EventResult,
}









// impl Display for CommandUsage {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mandatory = self.iter()
//             .map(|args| {
//                 args.iter().map(|a| a.get_name()).join("|") 
//             }).map(|arg| format!("<{arg}>")).join(" ");
//         let optional = self.optional_args.iter()
//         .map(|args| {
//             args.iter().map(|a| a.get_name()).join("|") 
//         }).map(|arg| format!("[{arg}]")).join(" ");
//         f.write_fmt(format_args!("/{} {} {}", self.name, mandatory, optional))
//     }
// }

impl Command {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn get_usages(&self) -> Vec<Argument> {
        todo!()
    }

    pub fn get_aliases(&self) -> &[String] {
        &self.aliases
    }

    pub fn get_permission(&self) -> Option<&String> {
        self.permission.as_ref()
    }

    pub fn get_func(&self) -> fn(&mut CommandEvent) -> EventResult {
        self.func
    }
}

impl Command {
    pub fn new(
        name: &str, 
        description: Option<&str>, 
        usages: CommandUsage, 
        permission: Option<&str>, 
        aliases: &[String], 
        func: fn(&mut CommandEvent) -> EventResult,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.map(str::to_string),
            usages: usage,
            aliases: aliases.to_vec(),
            permission: permission.map(str::to_string),
            func: func,
        }
    }

    pub fn execute(e: &mut CommandEvent) {
        crate::RUNTIME.block_on(run_command(e));
    }

    pub fn parse_input(
        input: &mut String, 
        usage: &CommandUsage
    ) -> Result<(String, Vec<CommandArg>), ParseError> {

        let chars = input.chars();

        let label = input
            .chars()
            .take_while_ref(|c| !c.is_whitespace())
            .collect::<String>();

        let mut expecting_mandatory: VecDeque<_> = 
            usage.mandatory_args.into();

        let mut expecting_optional: VecDeque<_> = 
            usage.optional_args.into();

        let mut args: Vec<CommandArg> = vec![];
        let mut remaining: String = chars
            .skip_while(|c| c.is_whitespace())
            .collect();

        while !remaining.is_empty() {

            if expecting_mandatory.is_empty() {
                if expecting_optional.is_empty() {
                    return Err(ParseError::TooManyArguments);
                } else {
                    break;
                }
            }

            let expecting = expecting_mandatory
                .pop_front()
                .unwrap();

            args.push(parse_next_arg(
                &mut remaining, 
                &expecting
            )?);
        }

        if !expecting_mandatory.is_empty() {
            return Err(ParseError::MissingArguments { 
                message: expecting_mandatory.into_iter()
                    .map(|args| {
                        args.iter()
                            .map(|a| a.to_string())
                            .join("|")
                    }).join(", ")
            });
        }
        
        
        Ok((label, args))
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    UnbalancedExpression { message: String },
    UnpairedQuote { message: String },
    MissingArguments { message: String },
    TooManyArguments,
    IncorrectArguments { message: String },
    Other { message: String }
}



impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnbalancedExpression { 
                message 
            } => write!(f, "Unbalanced Expression: {message}."),
            ParseError::UnpairedQuote {
                message
            } => write!(f, "Unpaired Quote: {message}."),
            ParseError::Other { 
                message 
            } => write!(f, "Other: {message}."),
            ParseError::MissingArguments { 
                message 
            } => write!(f, "Missing arguments: {message}"),
            ParseError::TooManyArguments => write!(f, "Too many arguments"),
            ParseError::IncorrectArguments { 
                message 
            } => write!(f, "Incorrect Arguments: {message}"),
        }
    }
}

fn parse_next_arg(
    remaining: &mut String, 
    expecting: &[Argument],
) -> Result<CommandArg, ParseError> {

    fn eat_token(remaining: &mut String) -> String {
        let mut chars = remaining.chars();
        let token: String = chars
            .take_while_ref(|c| !c.is_whitespace())
            .collect();
        *remaining = chars.collect();
        token
    }   
    
    

    for arg in expecting {
        match arg.get_type() {
            ArgType::Subcommand { usage } => {
                if let Ok((n, u)) = Command::parse_input(remaining, usage) {
                    return Ok(CommandArg::Subcommand(n, u));
                };
            },
            ArgType::Bool => {
                if let val = CommandArg::Bool(eat_token(remaining).parse()?) {
                    return val
                }
            },
            ArgType::Float { properties } => todo!(),
            ArgType::Double { properties } => todo!(),
            ArgType::Int { properties } => todo!(),
            ArgType::Long { properties } => todo!(),
            ArgType::String { properties } => todo!(),
            ArgType::Angle => todo!(),
            ArgType::Player => todo!(),
            ArgType::Identifier => todo!(),
        }
    }
    return 
}

pub trait TraitCommand {
    fn execute(&self, event: &mut CommandEvent) -> bool;
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
                Some(arc) => Some(arc.get_name().to_string()),
                None => None,
            }
        }
    }

    pub fn send_message(&self, message: String) {
        match self {
            CommandSender::Console => {
                RUNTIME.spawn(
                    CONSOLE.get().unwrap().println(message)
                );
            },
            CommandSender::Player { player } => {
                match player.upgrade() {
                    Some(arc) => {
                        RUNTIME.spawn(async move { 
                            arc.send_message(message).await 
                        });
                    },
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
                    "stop", 
                    Some("Stops the server."), 
                    CommandUsage::new("stop", &[], &[]), 
                    Some("stop"), 
                    &[],
                    command_stop
                ),
            ],
        }
    }

    pub fn register_command(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    pub fn build(self) -> CommandMap {
        let mut command_hashmap = 
            HashMap::with_capacity(self.commands.len());
        let mut aliases_hashmap = HashMap::new();
        for command in self.commands {
            for alias in command.aliases.iter() {
                aliases_hashmap.insert(
                    alias.clone(), 
                    command.name.clone()
                );
            }
            command_hashmap.insert(command.name.clone(), command);
        }
        CommandMap {
            commands: command_hashmap,
            aliases: aliases_hashmap
        }
    }
}

async fn run_command(e: &mut CommandEvent) -> EventResult {

    async fn run(evt: &mut CommandEvent) -> EventResult {
        match event::listen(THE_SERVER.get_event_manager(), evt).await {
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

fn command_stop(_: &mut CommandEvent) -> EventResult {
    let _ = crate::STOP_SIGNAL.get().unwrap().send(true);
    EventResult::Default
}

fn command_reload(_: &mut CommandEvent) -> EventResult {
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
    ParseError { error: ParseError },
    PermissionError,
    InvalidUsage { command: Command },
    Other { message: String },
    NotFound,
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::EventCancelled => {
                f.write_fmt(format_args!("CommandEvent cancelled."))
            },
            CommandError::PermissionError => {
                f.write_fmt(format_args!(
                    "You do not have permission to execute this command."
                ))
            }
            CommandError::InvalidUsage { command } => {
                f.write_fmt(format_args!(
                    "Incorrect arguments for {}. Usage: /{}.", 
                    command.name, command.usage
                ))
            }
            CommandError::Other { message } => {
                f.write_fmt(format_args!("{message}"))
            }
            CommandError::ParseError { error } => {
                write!(f, "Error while parsing command: {error}.")
            }
            CommandError::NotFound => {
                write!(f, "Command not found.")
            },
        }
        
    }
}



pub enum CommandArg {
    Subcommand(String, Vec<CommandArg>),
    Bool(bool),
    Float(f32),
    Double(f64),
    Int(i32),
    Long(i64),
    String(String),
    Angle(Angle),
    BlockPos(f64, f64, f64),
    BlockPredicate {
        block_id: Identifier, 
        block_states: Vec<(String, String)>,
        data_tags: Vec<(String, String)>,     
    },
    Item(Identifier),
    Structure(Identifier),

    Vec3(f64, f64, f64),
}



