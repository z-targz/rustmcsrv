use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use crate::data_types::*;
use datapack::DataPackID;
use tag::TagRegistry;
use text_component::Nbt;
use uuid::Uuid;

use server_util::ConnectionState;

use server_macros::CPacket;
use server_macros::SPacket;

#[derive(CPacket)]
#[state(Configuration)]
#[id(1)]
#[allow(non_camel_case_types)]
pub struct CPluginMessage_Config {
    channel: Identifier, //TODO: replace with Identifier
    data: InferredByteArray,
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(2)]
#[allow(non_camel_case_types)]
pub struct CDisconnect_Config {
    reason: TextComponent<Nbt>, //TODO: text component
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(3)]
pub struct CFinishConfig {}

#[derive(CPacket)]
#[state(Configuration)]
#[id(4)]
#[allow(non_camel_case_types)]
pub struct CKeepAlive_Config {
    keep_alive_id: i64,
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(6)]
pub struct CResetChat {}

//id = 7
const CREGISTRYDATA_ID: i32 = 7;
pub struct CRegistryData {
    registry_name: String,
}

impl Packet for CRegistryData {
    fn get_id(&self) -> i32 where Self: Sized {
        CREGISTRYDATA_ID
    }

    fn get_associated_state(&self) -> ConnectionState {
        ConnectionState::Configuration
    }
}

impl Clientbound for CRegistryData {
    fn to_be_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = format!("minecraft:{}",self.registry_name).to_protocol_bytes();
        let nbtified_entries = crate::REGISTRY_NBT.get(&self.registry_name).unwrap();
        data.append(&mut VarInt::new(nbtified_entries.len() as i32).to_protocol_bytes());
        data.append(
            &mut nbtified_entries.into_iter().map(|nbtified_entry| {
                let mut entry: Vec<u8> = nbtified_entry.entry_identifier.to_protocol_bytes();
                entry.push(0u8); //Has Data
                //entry.extend(nbtified_entry.data.iter());
                entry
            })
                .flatten()
                .collect()
        );
    
    
        let mut out: Vec<u8> = VarInt::new(data.len() as i32 + 1).to_protocol_bytes();
        out.push(CREGISTRYDATA_ID as u8);
        out.append(&mut data);
        out
    }
}

impl CRegistryData {
    pub fn new(registry_name: &str) -> Self {
        CRegistryData{
            registry_name: registry_name.to_owned()
        }
    }
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(8)]
#[allow(non_camel_case_types)]
pub struct CRemoveResourcePack_Config {
    uuid: Option<Uuid>, //The resource pack's UUID
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(9)]
#[allow(non_camel_case_types)]
pub struct CAddResourcePack_Config {
    uuid: Uuid, //UUID of the resource pack
    url: String,
    hash: String, //40 character hex string of SHA-1 of resource pack file
    forced: bool,
    //prompt_message: Option<TextComponent>, TODO: implement NBT TextComponent
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(0x0c)]
pub struct CFeatureFlags {
    features: IdentifierArray
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(0x0d)]
pub struct CUpdateTags {
    tag_registries: Vec<TagRegistry>
}

#[derive(CPacket)]
#[state(Configuration)]
#[id(0x0e)]
pub struct CKnownPacks {
    known_packs: Vec<DataPackID>
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
#[id(2)]
#[allow(non_camel_case_types)]
pub struct SPluginMessage_Config {
    identifier: String,
    payload: InferredByteArray,
}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Configuration)]
#[id(3)]
pub struct SAcknowledgeFinishConfig {}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Configuration)]
#[id(4)]
#[allow(non_camel_case_types)]
pub struct SKeepAlive_Config {
    keep_alive_id: i64,
}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Configuration)]
#[id(7)]
pub struct SKnownPacks {
    known_packs: Vec<DataPackID>
}

