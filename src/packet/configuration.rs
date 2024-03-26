use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use crate::data_types::*;
use uuid::Uuid;

use server_util::ConnectionState;

use server_macros::CPacket;
use server_macros::SPacket;

#[derive(CPacket)]
#[state(Configuration)]
#[id(0)]
#[allow(non_camel_case_types)]
pub struct CPluginMessage_Config {
    channel: String, //TODO: replace with Identifier
    data: InferredByteArray,
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(1)]
#[allow(non_camel_case_types)]
pub struct CDisconnect_Config {
    reason: String,
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(2)]
pub struct CFinishConfig {}

#[derive(CPacket)]
#[state(Configuration)]
#[id(3)]
#[allow(non_camel_case_types)]
pub struct CKeepAlive_Config {
    keep_alive_id: i64,
}

//id = 5
pub struct CRegistryData {}

impl Packet for CRegistryData {
    fn get_id(&self) -> i32 where Self: Sized {
        5
    }

    fn get_associated_state(&self) -> ConnectionState {
        ConnectionState::Configuration
    }
}

impl Clientbound for CRegistryData {
    fn to_be_bytes(&self) -> Vec<u8> {
        
        let mut data: Vec<u8> = crate::REGISTRY_NBT.to_protocol_bytes();
        let mut out: Vec<u8> = VarInt::new(data.len() as i32 + 1).to_protocol_bytes();
        out.push(5 as u8);
        out.append(&mut data);
        out
    }
}

impl CRegistryData {
    pub fn new() -> Self {
        CRegistryData{}
    }
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(6)]
#[allow(non_camel_case_types)]
pub struct CRemoveResourcePack_Config {
    uuid: Option<Uuid>, //The resource pack's UUID
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(7)]
#[allow(non_camel_case_types)]
pub struct CAddResourcePack_Config {
    uuid: Uuid, //UUID of the resource pack
    url: String,
    hash: String, //40 character hex string of SHA-1 of resource pack file
    forced: bool,
    //prompt_message: Option<TextComponent>, TODO: implement NBT TextComponent
}

//TODO: ID 8: https://wiki.vg/Protocol#Feature_Flags

#[derive(Debug)]
#[derive(SPacket)]
#[state(Configuration)]
#[id(0)]
#[allow(non_camel_case_types)]
pub struct SClientInformation_Config {
    locale: String,
    view_distance: u8, //Is supposed to be an i8 but I'm lazy and it also doesn't make sense how can someone have negative render distance
    chat_mode: VarInt,
    chat_colors: bool,
    displayed_skin_parts: u8,
    main_hand: VarInt,
    enable_text_filtering: bool,
    allow_server_listings: bool,
}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Configuration)]
#[id(1)]
#[allow(non_camel_case_types)]
pub struct SPluginMessage_Config {
    identifier: String //TODO: implement custom type alias

}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Configuration)]
#[id(2)]
pub struct SAcknowledgeFinishConfig {}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Configuration)]
#[id(3)]
#[allow(non_camel_case_types)]
pub struct SKeepAlive_Config {
    keep_alive_id: i64,
}
