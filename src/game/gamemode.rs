use std::error::Error;

pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator
}

#[derive(Debug)]
pub struct InvalidGamemodeError {}

impl Error for InvalidGamemodeError {}

impl std::fmt::Display for InvalidGamemodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not a valid gamemode.")
    }
}

impl Gamemode {
    pub fn from_str(the_str: &str) -> Result<Self, InvalidGamemodeError> {
        match the_str {
            "0" | "s"   | "survival"    | "Survival"    => Ok(Gamemode::Survival),
            "1" | "c"   | "creative"    | "Creative"    => Ok(Gamemode::Creative),
            "2" | "a"   | "adventure"   | "Adventure"   => Ok(Gamemode::Adventure),
            "3" | "sp"  | "spectator"   | "Spectator"   => Ok(Gamemode::Spectator),
            _ => Err(InvalidGamemodeError{})
        }
    }
}