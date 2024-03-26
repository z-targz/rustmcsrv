use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use server_util::ConnectionState;

use crate::data_types::*;

use uuid::Uuid;

use server_macros::CPacket;
use server_macros::SPacket;

#[derive(CPacket)]
#[state(Play)]
#[id(0)]
pub struct CBundleDelimiter {}

#[derive(CPacket)]
#[state(Play)]
#[id(1)]
pub struct CSpawnEntity {
    entity_id: VarInt,
    entity_uuid: Uuid,
    r#type: VarInt,
    x: f64,
    y: f64,
    z: f64,
    pitch: Angle,
    yaw: Angle,
    head_yaw: Angle,
    data: VarInt,
    velocity_x: i16,
    velocity_y: i16,
    velocity_z: i16,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x1b)]
#[allow(non_camel_case_types)]
pub struct CDisconnect_Play {
    reason: TextComponent,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x29)]
#[allow(non_camel_case_types)]
pub struct CLogin_Play {
    entity_id: i32,
    is_hardcore: bool,
    dimension_count: VarInt,
    dimension_names: Vec<Identifier>,
    max_players: VarInt, //Unused, just send 0
    view_distance: VarInt,
    simulation_distance: VarInt,
    reduced_debug_info: bool,
    enable_respawn_screen: bool, //Set to false when the doImmediateRespawn gamerule is true.
    do_limited_crafting: bool, //Unused, just send false
    dimension_type: Identifier,
    dimension_name: Identifier,
    hashed_seed: i64,
    game_mode: u8,
    previous_game_mode: i8, //probably -1 for this by default
    is_debug: bool, //debug world, this is a special world type, set this to false.
    is_flat: bool, //superflat world yes or no
    death_location: Option<DeathLocation>,
    portal_cooldown: VarInt, //num ticks before player can use portal again. should start at 0

    //and many more
}
/// Flags (If the value of the byte is masked, it's a relative offset, otherwise it's absolute):
/// `0x01` - X
/// `0x02` - Y
/// `0x04` - Z
/// `0x08` - Y_ROT (Pitch)
/// `0x10` - X_ROT (Yaw)
#[derive(CPacket)]
#[state(Play)]
#[id(0x3e)]
pub struct CSynchronizePlayerPosition {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    flags: u8,
    teleport_id: VarInt,
}