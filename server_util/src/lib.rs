extern crate quote;
extern crate proc_macro2;

use proc_macro2::TokenStream;

use quote::quote;
use quote::ToTokens;

pub mod error;

#[derive(Clone, Copy)]
pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play
}

impl ToTokens for ConnectionState {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let stream = match self {
            Self::Handshake => quote!{ConnectionState::Handshake},
            Self::Status => quote!{ConnectionState::Status},
            Self::Login => quote!{ConnectionState::Login},
            Self::Configuration => quote!{ConnectionState::Configuration},
            Self::Play => quote!{ConnectionState::Play},
        };
        tokens.clone_from(&stream);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PropertyType {
    Bool {optional: bool},
    Int {optional: bool},
    UShort {optional: bool},
    String {optional: bool},
}

