use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use crate::data_types::*;
use datapack::DataPackID;
use datapack::PackResponse;
use tag::TagRegistry;
use text_component::Json;
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
        /*let mut data = vec!

        [21, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 100, 97, 109, 97, 103, 101, 95, 116, 
        121, 112, 101, 47, 15, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 97, 114, 114, 111, 
        119, 0, 27, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 98, 97, 100, 95, 114, 101, 115, 
        112, 97, 119, 110, 95, 112, 111, 105, 110, 116, 0, 16, 109, 105, 110, 101, 99, 114, 97, 102, 
        116, 58, 99, 97, 99, 116, 117, 115, 0, 18, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 99, 
        97, 109, 112, 102, 105, 114, 101, 0, 18, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 99, 114, 
        97, 109, 109, 105, 110, 103, 0, 23, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 100, 114, 97, 
        103, 111, 110, 95, 98, 114, 101, 97, 116, 104, 0, 15, 109, 105, 110, 101, 99, 114, 97, 102, 116, 
        58, 100, 114, 111, 119, 110, 0, 17, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 100, 114, 121, 
        95, 111, 117, 116, 0, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 101, 120, 112, 108, 111, 
        115, 105, 111, 110, 0, 14, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 102, 97, 108, 108, 0, 23, 
        109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 102, 97, 108, 108, 105, 110, 103, 95, 97, 110, 118, 
        105, 108, 0, 23, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 102, 97, 108, 108, 105, 110, 103, 
        95, 98, 108, 111, 99, 107, 0, 28, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 102, 97, 108, 
        108, 105, 110, 103, 95, 115, 116, 97, 108, 97, 99, 116, 105, 116, 101, 0, 18, 109, 105, 110, 101, 
        99, 114, 97, 102, 116, 58, 102, 105, 114, 101, 98, 97, 108, 108, 0, 19, 109, 105, 110, 101, 99, 114, 
        97, 102, 116, 58, 102, 105, 114, 101, 119, 111, 114, 107, 115, 0, 23, 109, 105, 110, 101, 99, 114, 97, 
        102, 116, 58, 102, 108, 121, 95, 105, 110, 116, 111, 95, 119, 97, 108, 108, 0, 16, 109, 105, 110, 101, 
        99, 114, 97, 102, 116, 58, 102, 114, 101, 101, 122, 101, 0, 17, 109, 105, 110, 101, 99, 114, 97, 102, 
        116, 58, 103, 101, 110, 101, 114, 105, 99, 0, 22, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 103, 
        101, 110, 101, 114, 105, 99, 95, 107, 105, 108, 108, 0, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 
        58, 104, 111, 116, 95, 102, 108, 111, 111, 114, 0, 17, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 
        105, 110, 95, 102, 105, 114, 101, 0, 17, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 105, 110, 95, 
        119, 97, 108, 108, 0, 24, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 105, 110, 100, 105, 114, 101, 
        99, 116, 95, 109, 97, 103, 105, 99, 0, 14, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 108, 97, 118, 
        97, 0, 24, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 108, 105, 103, 104, 116, 110, 105, 110, 103, 95, 
        98, 111, 108, 116, 0, 15, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 109, 97, 103, 105, 99, 0, 20, 109, 
        105, 110, 101, 99, 114, 97, 102, 116, 58, 109, 111, 98, 95, 97, 116, 116, 97, 99, 107, 0, 29, 109, 105, 110, 
        101, 99, 114, 97, 102, 116, 58, 109, 111, 98, 95, 97, 116, 116, 97, 99, 107, 95, 110, 111, 95, 97, 103, 103, 
        114, 111, 0, 24, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 109, 111, 98, 95, 112, 114, 111, 106, 101, 
        99, 116, 105, 108, 101, 0, 17, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 110, 95, 102, 105, 114, 
        101, 0, 22, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 117, 116, 95, 111, 102, 95, 119, 111, 114, 
        108, 100, 0, 24, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 117, 116, 115, 105, 100, 101, 95, 98, 
        111, 114, 100, 101, 114, 0, 23, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 112, 108, 97, 121, 101, 114, 
        95, 97, 116, 116, 97, 99, 107, 0, 26, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 112, 108, 97, 121, 101, 
        114, 95, 101, 120, 112, 108, 111, 115, 105, 111, 110, 0, 20, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 
        115, 111, 110, 105, 99, 95, 98, 111, 111, 109, 0, 14, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 115, 
        112, 105, 116, 0, 20, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 115, 116, 97, 108, 97, 103, 109, 105, 
        116, 101, 0, 16, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 115, 116, 97, 114, 118, 101, 0, 15, 109, 105, 
        110, 101, 99, 114, 97, 102, 116, 58, 115, 116, 105, 110, 103, 0, 26, 109, 105, 110, 101, 99, 114, 97, 102, 116, 
        58, 115, 119, 101, 101, 116, 95, 98, 101, 114, 114, 121, 95, 98, 117, 115, 104, 0, 16, 109, 105, 110, 101, 99, 
        114, 97, 102, 116, 58, 116, 104, 111, 114, 110, 115, 0, 16, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 116, 
        104, 114, 111, 119, 110, 0, 17, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 116, 114, 105, 100, 101, 110, 116, 0, 
        31, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 117, 110, 97, 116, 116, 114, 105, 98, 117, 116, 101, 100, 95, 102, 
        105, 114, 101, 98, 97, 108, 108, 0, 21, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 119, 105, 110, 100, 95, 99, 
        104, 97, 114, 103, 101, 0, 16, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 119, 105, 116, 104, 101, 114, 0, 22, 
        109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 119, 105, 116, 104, 101, 114, 95, 115, 107, 117, 108, 108, 0];*/
    
    
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

