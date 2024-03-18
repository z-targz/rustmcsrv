use std::error::Error;
use server_util::ConnectionState;

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

#[allow(non_camel_case_types)]
pub enum SPacket {
    SHandshake(Box<handshake::SHandshake>),
    SStatusRequest(Box<status::SStatusRequest>),
    SPingRequest_Status(Box<status::SPingRequest_Status>),
    SLoginStart(Box<login::SLoginStart>),
    SLoginAcknowledged(Box<login::SLoginAcknowledged>),
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


pub fn create_packet(id: i32, state: ConnectionState, iter: &mut impl Iterator<Item = u8>) -> Result<SPacket, CreatePacketError> {
    //TODO: Replace this with a macro!
    match state {
        ConnectionState::Handshake => match id {
            0 => Ok(SPacket::SHandshake(handshake::SHandshake::parse(iter)?)),
            _ => Err(CreatePacketError::InvalidPacketIDError),
        },
        ConnectionState::Status => match id {
            0 => Ok(SPacket::SStatusRequest(status::SStatusRequest::parse(iter)?)),
            1 => Ok(SPacket::SPingRequest_Status(status::SPingRequest_Status::parse(iter)?)),
            _ => Err(CreatePacketError::InvalidPacketIDError),
        },
        ConnectionState::Login => match id {
            0 => Ok(SPacket::SLoginStart(login::SLoginStart::parse(iter)?)),
            3 => Ok(SPacket::SLoginAcknowledged(login::SLoginAcknowledged::parse(iter)?)),
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
        ConnectionState::Configuration => match id {
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
        ConnectionState::Play => match id {
            _ => Err(CreatePacketError::InvalidPacketIDError),
        }
    }
}

