#![feature(proc_macro_span)]

extern crate proc_macro;

use std::sync::Mutex;
use std::collections::HashMap;

use proc_macro::TokenStream;

use quote::quote;

use quote::ToTokens;
use syn::MetaList;
use syn::Data;
use syn::DataStruct;
use syn::Fields;
use syn::parse_macro_input;
use syn::LitStr;

use server_util::ConnectionState;

use base64::prelude::*;

use lazy_static::lazy_static;

lazy_static!{
    static ref HANDSHAKE_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
    static ref STATUS_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
    static ref LOGIN_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
    static ref CONFIGURATION_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
    static ref PLAY_PACKETS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
}

#[proc_macro_derive(CPacket, attributes(state, id))]
pub fn cpacket_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_cpacket(&ast)
}

fn impl_cpacket(ast: &syn::DeriveInput) -> TokenStream {
    let (
            name, 
            id, 
            state, 
            fields
        ) = impl_packet(ast);
    /* 
     * impl Self, impl Clientbound
     */
    let mut fields_text: String = String::new();
    let mut assign: String = String::new();
    let mut writes: String = String::new();

    for field in &fields.named {
        let field_name = field.ident.to_token_stream().to_string();

        fields_text += field.to_token_stream().to_string().as_str();
        fields_text += ", ";
        
        assign += format!("{field_name} : {field_name}, ").as_str();


        let field_type = field.ty.to_token_stream().to_string();
        if field_type.starts_with("Option") {
            writes += format!("data.extend(create_option({}self.{field_name}));", "").as_str();
        } else {
            //writes += format!("data.extend({func}({}self.{field_name}));", borrow(field_type.as_str())).as_str();
            writes += format!("data.extend(self.{field_name}.to_protocol_bytes().iter());", /*borrow(field_type.as_str())*/).as_str();
        }
    }

    let assign: proc_macro2::TokenStream = assign.parse().unwrap();
    let fields_text: proc_macro2::TokenStream = fields_text.parse().unwrap();
    let writes: proc_macro2::TokenStream = writes.parse().unwrap();

    let gen = quote! {
        impl Packet for #name {
            fn get_id(&self) -> i32{
                #id
            }
            fn get_associated_state(&self) -> ConnectionState {
                #state
            }
        }

        impl Clientbound for #name {
            fn to_be_bytes(&self) -> Vec<u8> {
                let mut data: Vec<u8> = Vec::new();
                #writes
                let mut out: Vec<u8> = VarInt::new(data.len() as i32 + 1).to_protocol_bytes();
                out.push(#id as u8);
                out.append(&mut data);
                out
            }
        }

        impl #name {
            pub fn new(
                #fields_text
            ) -> Self {
                #name {
                    #assign
                }
            }
        }
    };
    gen.into()
}


#[proc_macro_derive(SPacket, attributes(state, id))]
pub fn spacket_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_spacket(&ast)
}

fn impl_spacket(ast: &syn::DeriveInput) -> TokenStream {
    let (
            name, 
            id, 
            state, 
            fields
        ) = impl_packet(ast);

    match state {
        ConnectionState::Handshake => {
            let mut lock = HANDSHAKE_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Status => {
            let mut lock = STATUS_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Login => {
            let mut lock = LOGIN_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Configuration => {
            let mut lock = CONFIGURATION_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        },
        ConnectionState::Play => {
            let mut lock = PLAY_PACKETS.lock().unwrap();
            lock.insert(id, name.to_string());
            drop(lock);
        }
    }

    /* 
     * impl Serverbound, Self
     */

    let mut field_names = String::new();
    let mut let_reads = String::new();
    let mut getters = String::new();

    //borrow variable-sized types
    fn borrow(str: &str) -> &str {
        match str {
            "String" => "&",
            "JSON" => "&",
            "CJSONTextComponent" => "&",
            "NBT" => "&",
            "PrefixedByteArray" => "&",
            "InferredByteArray" => "&",
            "PropertyArray" => "&",
            _ => "",
        }
    }

    for field in &fields.named {
        let field_name = field.ident.to_token_stream().to_string();
        field_names += format!("{field_name} : {field_name}, ").as_str();

        let field_type = field.ty.to_token_stream().to_string();

        if field_type.starts_with("Option") {
            let option_type = extract_T_from_option(&field_type);
            
            let b = borrow(option_type.as_str());
            getters += format!("pub fn get_{field_name}(&self) -> Option<{b}{option_type}> {{ self.{field_name}{} }}", if b == "&" {".as_ref()"} else {""}).as_str();

            let_reads += format!("let {field_name}: Option<{option_type}> = read_option(iter)?;").as_str();
        } else {
            let b = borrow(field_type.as_str());
            getters += format!("pub fn get_{field_name}(&self) -> {b}{field_type} {{ {b}self.{field_name} }}").as_str();

            let_reads += format!("let {field_name}: {field_type} = {field_type}::from_protocol_iter(iter)?;").as_str();
        }
    }

    let field_names: proc_macro2::TokenStream = field_names.parse().unwrap();
    let let_reads: proc_macro2::TokenStream = let_reads.parse().unwrap();
    let getters: proc_macro2::TokenStream = getters.parse().unwrap();

    let gen = quote! {
        impl Packet for #name {
            fn get_id(&self) -> i32{
                #id
            }
            fn get_associated_state(&self) -> ConnectionState {
                #state
            }
        }
        impl Serverbound for #name {
            fn parse(iter: &mut impl Iterator<Item = u8>) -> Result<Box<#name>, Box<dyn Error + Send + Sync>> {
                #let_reads
                Ok(Box::new(#name {
                    #field_names
                }))
            }
        }
        impl #name {
            #getters
        }
    };
    gen.into()
}

fn impl_packet(ast: &syn::DeriveInput) -> (&syn::Ident, i32, ConnectionState, &syn::FieldsNamed) {
    let name = &ast.ident;
    let attributes = &ast.attrs;
    if attributes.len() < 2 {
        panic!("Missing required attributes for Derive(Packet): id, ConnectionState.");
    }
    let mut id: i32 = 0;
    let mut state: ConnectionState = ConnectionState::Handshake;

    for attr in attributes {
        let meta_list: &MetaList = attr.meta.require_list().unwrap_or_else(|_| panic!("Missing arguments for {:?}", attr.path().get_ident()));

        if attr.path().is_ident("id") {
            let msg = "Argument to id must be a valid positive i32.";
            let arg: syn::LitInt = meta_list.parse_args().expect(msg);
            let arg_as_int = arg.base10_parse::<i32>().expect(msg);
            if arg_as_int.is_negative() {
                panic!("{msg}");
            }
            id = arg_as_int;
        } else if attr.path().is_ident("state") {
            let msg = "Argument to state must be a valid ConnectionState: (Handshake, Status, Login, Play, Configuration) }";
            
            let arg: proc_macro2::TokenStream = meta_list.parse_args().expect(msg);
            match arg.to_string().as_str() {
                "Handshake" => state = ConnectionState::Handshake,
                "Status" => state = ConnectionState::Status,
                "Login" => state = ConnectionState::Login,
                "Configuration" => state = ConnectionState::Configuration,
                "Play" => state = ConnectionState::Play,
                _ => panic!("{msg}"),
            }
        }

    }

    let fields = match &ast.data {
        Data::Struct(DataStruct{ fields: Fields::Named(it), struct_token : _, semi_token : _ }) => it,
        Data::Struct(_) => panic!("Expected a `struct` with named fields."),
        Data::Enum(_) | Data::Union(_) => panic!("#[Derive(CPacket)] is only implemented for `struct`s."),
    };
    
    (name, id, state, fields)
}

#[allow(non_snake_case)]
fn extract_T_from_option(string: &String) -> String {
    let s = remove_whitespace(string);
    s[7..s.len()-1].to_string()
}

fn remove_whitespace(string: &String) -> String {
    let mut s = string.clone();
    s.retain(|c| !c.is_whitespace());
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extract_option() {
        let string = "Option < Meme >".to_string();
        assert_eq!(extract_T_from_option(&string), "Meme".to_string());
    }
}

#[proc_macro]
pub fn base64_image(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let path = input.value();
    
    let data = std::fs::read(&path);
    match data {
        Ok(vec) => {
            let mut base64_string = String::new();
            BASE64_STANDARD.encode_string(vec, &mut base64_string);
            quote!{ Ok(#base64_string) }.into()
        }
        Err(_) => {
            quote!{ Err(std::io::Error::from(std::io::ErrorKind::NotFound))}.into()
        }
    }
}

#[proc_macro]
pub fn json_text_component(input: TokenStream) -> TokenStream {
    let input: LitStr = parse_macro_input!(input as LitStr);
    let text = input.value();
    let the_string = format!("{{\"text\":\"{text}\"}}");
    quote!{ #the_string }.into()
}

#[proc_macro]

pub fn register_packets(_: TokenStream) -> TokenStream {

    let mut packets = String::new();

    let handshake_lock = HANDSHAKE_PACKETS.lock().unwrap();
    for handshake_packet in handshake_lock.iter() {
        packets += format!("{}(Box<handshake::{}>),", handshake_packet.1, handshake_packet.1).as_str();
    }
    drop(handshake_lock);

    let status_lock = STATUS_PACKETS.lock().unwrap();
    for status_packet in status_lock.iter() {
        packets += format!("{}(Box<status::{}>),", status_packet.1, status_packet.1).as_str();
    }
    drop(status_lock);

    let login_lock = LOGIN_PACKETS.lock().unwrap();
    for login_packet in login_lock.iter() {
        packets += format!("{}(Box<login::{}>),", login_packet.1, login_packet.1).as_str();
    }
    drop(login_lock);

    let config_lock = CONFIGURATION_PACKETS.lock().unwrap();
    for config_packet in config_lock.iter() {
        packets += format!("{}(Box<configuration::{}>),", config_packet.1, config_packet.1).as_str();
    }
    drop(config_lock);

    let play_lock = PLAY_PACKETS.lock().unwrap();
    for play_packet in play_lock.iter() {
        packets += format!("{}(Box<play::{}>),", play_packet.1, play_packet.1).as_str();
    }
    drop(play_lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub enum SPacket {
            #packets
        }
    }.into()
}

#[proc_macro]
pub fn create_handshake_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = HANDSHAKE_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(handshake::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let handshake_packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #handshake_packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}

#[proc_macro]
pub fn create_status_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = STATUS_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(status::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}

#[proc_macro]
pub fn create_login_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = LOGIN_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(login::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}

#[proc_macro]
pub fn create_config_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = CONFIGURATION_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(configuration::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}

#[proc_macro]
pub fn create_play_packets(_: TokenStream) -> TokenStream {
    let mut packets = String::new();
    let lock = PLAY_PACKETS.lock().unwrap();
    for packet in lock.iter() {
        packets += format!("{} => Ok(SPacket::{}(play::{}::parse(iter)?)),", packet.0, packet.1, packet.1).as_str();
    }
    drop(lock);

    let packets: proc_macro2::TokenStream = packets.parse().unwrap();
    quote! {
        match id {
            #packets
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }.into()
}