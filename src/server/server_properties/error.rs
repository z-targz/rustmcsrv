use serde::{ser, de};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError(String),
    MalformedLine(i32),
    InvalidProperty(String, i32),
    PropertyCannotBeNone(String, i32),
    InvalidValueForProperty(String, i32),
    Custom(String, i32, String),

    Message(String),

    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`. These
    // are specific to the format, in this case JSON.
    Eof,
    Syntax,
    ExpectedBoolean,
    ExpectedInteger,
    ExpectedUShort,
    ValueCannotBeNegative,
    Empty,
    UnsupportedType,
    TrailingCharacters,
}

impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            Error::Message(msg) => msg.clone(),
            Error::Eof => "Unexpected EOF".to_string(),
            Error::Syntax => "Syntax Error".to_string(),
            Error::ExpectedBoolean => "Expected Boolean".to_string(),
            Error::ExpectedInteger => "Expected Integer".to_string(),
            Error::ExpectedUShort => "Value must be between 0 and 65535".to_string(),
            Error::ValueCannotBeNegative => "Value cannot be negative".to_string(),
            Error::Empty => "Value cannot be empty".to_string(),
            Error::UnsupportedType => "Unsupported Type".to_string(),
            Error::TrailingCharacters => "Trailing Characters".to_string(),

            Error::IOError(err) => err.clone(),
            Error::MalformedLine(line_num) => {
                format!("Malformed line (line {line_num})")
            },
            Error::InvalidProperty(prop, line_num) => {
                format!("Invalid property \"{prop}\" (line {line_num})")
            },
            Error::PropertyCannotBeNone(prop, line_num) => {
                format!("No value specified for property \"{prop}\" (line {line_num}")
            }
            Error::InvalidValueForProperty(prop, line_num) => {
                format!("Invalid value for property \"{prop}\" (line {line_num})")
            }
            Error::Custom(prop, line_num, msg) => {
                format!("{msg}. property: \"{prop}\" (line {line_num})")
            }
            
        };
        write!(f, "Error in server.properties: {err_msg}.")
    }
}

impl std::error::Error for Error {}