use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use server_util::ConnectionState;

use crate::data_types::*;

use uuid::Uuid;

use server_macros::CPacket;
use server_macros::SPacket;

#[derive(CPacket)]
#[state(Play)]
#[id(0x00)]
pub struct CBundleDelimiter {}

#[derive(CPacket)]
#[state(Play)]
#[id(0x01)]
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
#[id(0x02)]
pub struct CSpawnExperienceOrb {
    entity_id: VarInt,
    x: f64,
    y: f64,
    z: f64,
    count: i16,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x03)]
/// Animation ID:\
/// * `0` - Swing main arm
/// * `2` - Leave bed
/// * `3` - Swing offhand
/// * `4` - Critical effect
/// * `5` - Magic critical effect (Sharpness, Smite, BOA)
pub struct CEntityAnimation {
    entity_id: VarInt, //Player ID
    animation: u8,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x04)]
pub struct CAwardStatistics {
    statistics: StatisticArray,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x05)]
pub struct CAcknowledgeBlockChange {
    sequence_id: VarInt,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x06)]
pub struct CSetBlockDestroyStage {
    entity_id: VarInt,
    location: Position,
    destroy_stage: i8,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x07)]
pub struct CBlockEntityData {
    location: Position,
    r#type: VarInt,
    nbt_data: NBT,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x08)]
/// The packet uses a block ID from minecraft:block registry instead of a block state.\
/// This shouldn't matter, since the client does not yet use and may never use this value,
/// so it should just be set to 0.
pub struct CBlockAction {
    location: Position,
    action_id: u8,
    action_parameter: u8,
    block_type: VarInt,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x09)]
/// ## Block Update
/// ### **Warning**: Do not send block updates to unloaded chunks.
/// 
/// `block_id` uses a blockstate
pub struct CBlockUpdate {
    location: Position,
    block_id: VarInt,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x1b)]
#[allow(non_camel_case_types)]
/// ## Disconnect (Play)
pub struct CDisconnect_Play {
    reason: TextComponent,
}

#[derive(CPacket)]
#[state(Play)]
#[id(0x24)]
#[allow(non_camel_case_types)]
/// ## Clientbound Keep Alive (Play)
pub struct CKeepAlive_Play {
    keep_alive_id: i64,
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

#[derive(SPacket, Debug)]
#[state(Play)]
#[id(0x15)]
#[allow(non_camel_case_types)]
pub struct SKeepAlive_Play {
    keep_alive_id: i64,
}