
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{BufRead, BufReader, BufWriter, Write};


use serde::{Serialize, Deserialize};

use server_macros::ServerPropertiesDerive;



//use crate::game::gamemode::Gamemode;





#[derive(Serialize, Deserialize, Debug, ServerPropertiesDerive)]
pub struct ServerProperties {
    #[serde(rename = "server-port")]
    server_port: u16,
    motd: String,
    #[serde(rename = "max-players")]
    max_players: i32,
    #[serde(rename = "online-mode")]
    online_mode: bool,
    //gamemode: Gamemode,
}

impl ServerProperties {
    pub fn get_server_port(&self) -> u16 {
        self.server_port
    }

    pub fn get_motd(&self) -> &String {
        &self.motd
    }

    pub fn get_max_players(&self) -> i32 {
        self.max_players
    }

    pub fn is_online_mode(&self) -> bool {
        self.online_mode
    }

    /// Generates the default server_properties.json
    pub fn default() -> Self {
        ServerProperties { 
            server_port: 25565, 
            motd: "A Minecraft Server (§cMade with Rust!§r)".to_string(), 
            max_players: 20, 
            online_mode: false,
        }
    }

    pub fn write_to_file(&self, file: &mut impl Write) -> Result<(), WritePropertiesError> {
        let output = super::to_string(self)?;
        write!(file, "{output}")?;
        Ok(())
    }

    
    
}

#[derive(Debug)]
pub enum LoadPropertiesError {
    IOError(String),
    MalformedLine(i32),
    InvalidProperty(String, i32),
    PropertyCannotBeNone(String, i32),
    InvalidValueForProperty(String, i32),
    Custom(String, i32, String),
}

impl Error for LoadPropertiesError {}

impl std::fmt::Display for LoadPropertiesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_txt = match self {
            LoadPropertiesError::IOError(err) => err.clone(),
            LoadPropertiesError::MalformedLine(line_num) => {
                format!("Malformed line (line {line_num})")
            },
            LoadPropertiesError::InvalidProperty(prop, line_num) => {
                format!("Invalid property \"{prop}\" (line {line_num})")
            },
            LoadPropertiesError::PropertyCannotBeNone(prop, line_num) => {
                format!("No value specified for property \"{prop}\" (line {line_num}")
            }
            LoadPropertiesError::InvalidValueForProperty(prop, line_num) => {
                format!("Invalid value for property \"{prop}\" (line {line_num})")
            }
            LoadPropertiesError::Custom(prop, line_num, msg) => {
                format!("{msg}. property: \"{prop}\" (line {line_num})")
            }
        };
        write!(f, "Error reading server.properties: {err_txt}.")
    }
}

impl From<std::io::Error> for LoadPropertiesError {
    fn from(value: std::io::Error) -> Self {
        LoadPropertiesError::IOError(value.to_string())
    }
}

#[derive(Debug)]
pub enum WritePropertiesError {
    IOError(String),
    SerializationError(super::error::Error),
}

impl From<std::io::Error> for WritePropertiesError {
    fn from(value: std::io::Error) -> Self {
        WritePropertiesError::IOError(value.to_string())
    }
}

impl From<super::error::Error> for WritePropertiesError {
    fn from(value: super::error::Error) -> Self {
        Self::SerializationError(value)
    }
}

impl std::error::Error for WritePropertiesError {}

impl std::fmt::Display for WritePropertiesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            Self::IOError(err) => format!("I/O Error: {err}"),
            Self::SerializationError(err) => format!("Serde Error: {err}"),
        };
        write!(f, "Error writing server.properties: {err_msg}")
    }
}
