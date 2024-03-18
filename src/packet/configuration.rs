use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use crate::data::*;
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
    reason: CJSONTextComponent,
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

//TODO: CRegistryData (id: 5)
// registry_codec: NBT
// https://wiki.vg/Registry_Data

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

#[derive(SPacket)]
#[state(Configuration)]
#[id(1)]
pub struct SLoginPluginRequest {
    identifier: String //TODO: implement custom type alias

}
