use std::error::Error;
use crate::data::read_var_int;
use server_util::ConnectionState;
use server_util::error::IterEndError;

pub mod handshake;
pub mod status;

pub trait Packet: Sized { 
    fn get_id(&self) -> i32 where Self: Sized;
    fn get_associated_state(&self) -> ConnectionState;
}

pub trait Clientbound: Packet {
    fn to_be_bytes(&self) -> Vec<u8> where Self: Sized;
}

pub trait Serverbound: Packet {
    fn parse(iter: &mut impl Iterator<Item = u8>) -> Result<Box<Self>, Box<dyn Error>> where Self: Sized;
}

pub enum SPacket {
    SHandshake(handshake::SHandshake),
    SStatusRequest(status::SStatusRequest),
    SPingRequest_Status(status::SPingRequest_Status),
}

