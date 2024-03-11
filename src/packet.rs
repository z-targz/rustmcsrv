use std::error::Error;
use crate::data::read_var_int;
use crate::server::ConnectionState;
use server_util::take_forced;
use server_util::take_forced::CanTakeForced;

pub mod handshake;
pub mod status;

pub trait Packet: Sized { 
    fn get_id(&self) -> i32;
    fn get_associated_state(&self) -> ConnectionState;
    fn to_be_bytes(&self) -> Vec<u8>;
}

pub fn read_packet(iter: &mut impl Iterator<Item = u8>, state: ConnectionState) -> Result<Box<dyn Packet>, Box<dyn Error>> {
    let len = read_var_int(iter)? as usize;
    let packet_iter = iter.take_forced(len);

    todo!()
}