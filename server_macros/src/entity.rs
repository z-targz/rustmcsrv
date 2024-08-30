use std::fmt::Debug;
pub (in super) enum EntityMacroError {
    IOError(std::io::Error),
    SynError(syn::Error),
    Other(String),
}

impl std::error::Error for EntityMacroError {}

impl std::fmt::Display for EntityMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(io_error) => std::fmt::Display::fmt(io_error, f),
            Self::SynError(syn_error) => std::fmt::Display::fmt(syn_error, f),
            Self::Other(error_msg) => f.write_str(error_msg.as_str()),
        }
    }
}

impl std::fmt::Debug for EntityMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(io_error) => Debug::fmt(io_error, f),
            Self::SynError(syn_error) => Debug::fmt(syn_error, f),
            Self::Other(error_msg) => f.write_str(error_msg.as_str()),
        }
    }
}

impl From<std::io::Error> for EntityMacroError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<syn::Error> for EntityMacroError {
    fn from(value: syn::Error) -> Self {
        Self::SynError(value)
    }
}

impl From<String> for EntityMacroError {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}