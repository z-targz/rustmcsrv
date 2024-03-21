use std::error::Error;
use server_macros::create_handshake_packets;
use server_macros::create_status_packets;
use server_macros::create_login_packets;
use server_macros::create_config_packets;
use server_macros::create_play_packets;


use server_util::ConnectionState;

use server_macros::register_packets;

pub mod handshake;
pub mod status;
pub mod login;
pub mod configuration;

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

/*
#[allow(non_camel_case_types)]
pub enum CPacket {
    CStatusResponse(Box<status::CStatusResponse>),
    CPingResponse_Status(Box<status::CPingResponse_Status>),
}*/

register_packets!{}

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


pub fn create_packet(id: i32, state: ConnectionState, iter: &mut impl Iterator<Item = u8>) -> Result<SPacket, CreatePacketError> {
    match state {
        ConnectionState::Handshake => {
            create_handshake_packets!()
        },
        ConnectionState::Status => {
            create_status_packets!()
        },
        ConnectionState::Login => {
            create_login_packets!()
        }
        ConnectionState::Configuration => {
            create_config_packets!()
        }
        ConnectionState::Play => {
            create_play_packets!()
        }
    }
}

