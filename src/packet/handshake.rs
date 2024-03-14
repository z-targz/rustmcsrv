use std::error::Error;

use super::Packet;
use server_util::ConnectionState;
use crate::packet::Serverbound;
use crate::packet::Clientbound;

use crate::data::*;

use server_macros::SPacket;
use server_macros::CPacket;

#[derive(SPacket)]
#[state(Handshake)]
#[id(0)]
pub struct SHandshake {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    next_state: VarInt,
}
