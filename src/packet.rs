use std::error::Error;
use crate::data::read_var_int;
use crate::server::ConnectionState;
use server_util::take_forced;
use server_util::take_forced::CanTakeForced;
use server_util::error::IterEndError;

pub mod handshake;
//pub mod status;

pub trait Packet: Sized { 
    fn get_id(&self) -> i32;
    fn get_associated_state(&self) -> ConnectionState;
    
}

pub trait Clientbound {
    fn to_be_bytes(&self) -> Vec<u8>;
}

pub trait Serverbound {
    fn read_packet(iter: &mut impl Iterator<Item = u8>) -> Result<Box<Self>, Box<dyn Error>>;
}