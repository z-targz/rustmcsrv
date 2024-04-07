use std::error::Error;



use server_util::ConnectionState;

use super::SPacket;

pub trait Packet: Sized { 
    fn get_id(&self) -> i32 where Self: Sized;
    fn get_associated_state(&self) -> ConnectionState;
}

pub trait Clientbound: Packet {
    fn to_be_bytes(&self) -> Vec<u8> where Self: Sized;
}

pub trait Serverbound: Packet {
    fn parse(iter: &mut impl Iterator<Item = u8>) -> Result<Box<Self>, Box<dyn Error + Send + Sync>> where Self: Sized;
}

#[derive(Debug)]
pub enum CreatePacketError {
    InvalidPacketIDError,
    PacketCreateError(String),
}

impl Error for CreatePacketError {}

impl std::fmt::Display for CreatePacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str = match self {
            CreatePacketError::InvalidPacketIDError => "Invalid Packet ID".to_string(),
            CreatePacketError::PacketCreateError(s) => format!("{}",s),
        };
        write!(f, "CreatePacketError: {err_str}")
    }
}

impl From<Box<dyn Error + Send + Sync>> for CreatePacketError {
    fn from(value: Box<dyn Error + Send + Sync>) -> Self {
        CreatePacketError::PacketCreateError(value.to_string())
    }
}




