use std::error::Error;

use server_util::error::IterEndError;

use super::Packet;
use crate::server::ConnectionState;
use crate::packet::Serverbound;
use crate::data::{read_var_int, read_string, read_ushort};


pub struct SHandshake {
    id: i32,
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: i32,
}

impl SHandshake {
    fn new(protocol_version: i32, server_address: String, server_port: u16, next_state: i32) -> Self {
        SHandshake { id : 0, protocol_version : protocol_version, server_address : server_address, server_port : server_port, next_state: next_state }
    }
}

impl Packet for SHandshake {
    fn get_id(&self) -> i32{
        self.id
    }
    fn get_associated_state(&self) -> ConnectionState {
        ConnectionState::Handshake
    }
}

impl Serverbound for SHandshake {
    fn read_packet(iter: &mut impl Iterator<Item = u8>) -> Result<Box<SHandshake>, Box<dyn Error>> {
        let protocol_version: i32 = read_var_int(iter)?;
        let server_address: String = read_string(iter)?;
        let server_port: u16 = read_ushort(iter)?;
        let next_state: i32 = read_var_int(iter)?;
        Ok(Box::new(SHandshake::new(protocol_version, server_address, server_port, next_state)))
    }
}