use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{BufRead, BufReader, BufWriter, Write};

//use crate::game::gamemode::Gamemode;

pub struct ServerProperties {
    server_port: u16,
    motd: String,
    max_players: i32,
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
            //gamemode: Gamemode::Survival,
        }
    }

    pub fn write_to_file(&self, file: &mut impl Write) -> Result<(), std::io::Error> {
        writeln!(file, "#An analog of the vanilla server.properties.")?;
        writeln!(file, "#Most properties here are similar to the vanilla server.properties.")?;
        writeln!(file, "#Most vanilla properties, however, are not supported.")?;
        writeln!(file, "server-port={}", self.server_port)?;
        writeln!(file, "motd={}", self.motd)?;
        writeln!(file, "max-players={}", self.max_players)?;
        writeln!(file, "online-mode={}", self.online_mode)?;
        Ok(())
    }

    /// Loads the server properties from the file server.properties in the
    /// main directory.
    /// 
    /// TODO: replace this with a custom serde implementation
    /// 
    /// Desired functionality for the serde implementation:
    ///
    /// 1. creates the default ServerProperties
    /// 2. deserializes server.properties from field-name=value to field_name = value 
    ///    into the created ServerProperties
    /// 3. serializes the createdServerProperties to server.properties to populate 
    ///    missing properties
    pub fn load_server_properties() -> Result<ServerProperties, LoadPropertiesError> {
        if !Path::new("server.properties").exists()
        {
            let mut file = File::create("server.properties")?;
            match ServerProperties::default().write_to_file(&mut file) {
                Ok(_) => (),
                Err(e) => return Err(LoadPropertiesError::IOError(e.to_string())),
            }
        };

        let reader = BufReader::new(OpenOptions::new().read(true).open("server.properties")?);
        
        let mut server_properties = ServerProperties::default();

        let mut i = 0;
        for result in reader.lines() {
            i += 1;
            match result {
                Ok(line) => {
                    if line.starts_with("#") { continue; }

                    let pair = line.split("=").collect::<Vec<_>>();

                    if pair.len() != 2 {
                        return Err(std::io::Error::other("Malformed line"))?;
                    }

                    
                    //let mut gamemode = Gamemode::Survival;
                    let tuple: (&str, &str) = (pair.get(0).unwrap(), pair.get(1).unwrap());
                    match tuple.0 {
                        "server-port" => {
                            match tuple.1.parse::<u16>() {
                                Ok(short) => {
                                    server_properties.server_port = short;
                                },
                                Err(_) => {
                                    return Err(LoadPropertiesError::InvalidValueForProperty(tuple.0.to_string(), i));
                                }
                            }
                        },
                        "motd" => {
                            server_properties.motd = tuple.1.to_string();
                        }
                        "max-players" => {
                            match tuple.1.parse::<i32>() {
                                Ok(int) => {
                                    server_properties.max_players = int;
                                },
                                Err(_) => {
                                    return Err(LoadPropertiesError::InvalidValueForProperty(tuple.0.to_string(), i));
                                }
                            }
                        }
                        "online-mode" => {
                            match tuple.1.parse::<bool>() {
                                Ok(bool_val) => {
                                    server_properties.online_mode = bool_val;
                                },
                                Err(_) => {
                                    return Err(LoadPropertiesError::InvalidValueForProperty(tuple.0.to_string(), i));
                                }
                            }
                        }
                        _ => return Err(LoadPropertiesError::InvalidProperty(pair.get(0).unwrap().to_string(), i))
                    }
                },
                Err(e) => return Err(e)?,
            }
        }
        let mut writer = BufWriter::new(OpenOptions::new().read(true).write(true).truncate(true).open("server.properties")?);
        match server_properties.write_to_file(&mut writer) {
            Ok(_) => (),
            Err(e) => return Err(LoadPropertiesError::IOError(e.to_string())),
        }
        Ok(server_properties)
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


