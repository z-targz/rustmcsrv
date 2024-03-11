use super::Packet;

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
}

pub struct CHandshake {
    id: i32
}

impl CHandshake {
    pub fn new() -> Self {
        CHandshake { id : 0 }
    }
}