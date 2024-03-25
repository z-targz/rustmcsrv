use std::{array::TryFromSliceError, string::FromUtf8Error};

#[derive(Debug, Clone)]
pub struct IterEndError;

impl IterEndError {
    pub fn new() -> Self {
        IterEndError { }
    }
}

impl std::fmt::Display for IterEndError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("IterEndError: Iterator ended unexpectedly."))
    }
}

impl std::error::Error for IterEndError {}

#[derive(Debug, Clone)]
pub enum ProtocolError {
    IterEndError,
    VarIntError,
    VarLongError,
    NotBoolean,
    InvalidUtf8,
    TryFromSlice,
}

impl std::error::Error for ProtocolError {}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_txt = match *self {
            ProtocolError::IterEndError => "Iterator ended unexpectedly",
            ProtocolError::VarIntError => "VarInt too large",
            ProtocolError::VarLongError => "VarLong too large",
            ProtocolError::NotBoolean => "Not a Boolean value",
            ProtocolError::InvalidUtf8 => "String is not valid UTF-8",
            ProtocolError::TryFromSlice => "Unable to convert from bytes",
        };
        write!(f, "Protocol Error: {err_txt}.")
    }
}

impl From<FromUtf8Error> for ProtocolError {
    fn from(_: FromUtf8Error) -> Self {
        ProtocolError::InvalidUtf8
    }
}

impl From<TryFromSliceError> for ProtocolError {
    fn from(_: TryFromSliceError) -> Self {
        ProtocolError::TryFromSlice
    }
}