use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use server_util::ConnectionState;

use crate::data::*;
use crate::data::angle::Angle;
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