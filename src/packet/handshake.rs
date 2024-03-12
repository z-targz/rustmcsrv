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
/*
impl SHandshake {
    fn new(
        protocol_version: VarInt, 
        server_address: String, 
        server_port: u16, 
        next_state: VarInt,
    ) -> Self {
        SHandshake { 
            protocol_version : protocol_version, 
            server_address : server_address, 
            server_port : server_port, 
            next_state: next_state 
        }
    }
}
*/
/*
impl Packet for SHandshake {
    fn get_id(&self) -> i32{
        0
    }
    fn get_associated_state(&self) -> ConnectionState {
        ConnectionState::Handshake
    }
}
*/
/*
impl Serverbound for SHandshake {
    fn parse(iter: &mut impl Iterator<Item = u8>) -> Result<Box<SHandshake>, Box<dyn Error>> {
        let protocol_version: VarInt = read_var_int(iter)?;
        let server_address: String = read_string(iter)?;
        let server_port: u16 = read_ushort(iter)?;
        let next_state: VarInt = read_var_int(iter)?;

        Ok(Box::new(
            SHandshake {
                protocol_version, 
                server_address, 
                server_port, 
                next_state
            }))
    }
}*/