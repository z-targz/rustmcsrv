use super::Packet;
use crate::server::ConnectionState;

pub struct SHandshake {
    id: i32,
    protocol_version: i32,
    server_address: u16,
    next_state: i32,
}

impl SHandshake {
    pub fn new(protocol_version: i32, server_address: u16, next_state: i32) -> Self {
        SHandshake { id : 0, protocol_version : protocol_version, server_address : server_address, next_state: next_state }
    }
}

impl Packet for SHandshake {
    fn get_id(&self) -> i32{
        self.id
    }
    fn get_associated_state(&self) -> ConnectionState {
        ConnectionState::Handshake
    }
    fn to_be_bytes(&self) -> Vec<u8> {
        protocol_version_var_int = create_var_int(self.protocol)
    }
}