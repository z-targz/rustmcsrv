use std::error::Error;
use crate::data::read_var_int;
use crate::server::ConnectionState;
use server_util::take_forced;
use server_util::take_forced::CanTakeForced;

pub mod handshake;

pub trait Packet { 
    fn get_id(&self) -> i32;
}

pub fn read_packet(iter: &mut impl Iterator<Item = u8>, state: ConnectionState) -> Result<Box<dyn Packet>, Box<dyn Error>> {
    let len = read_var_int(iter)? as usize;
    let packet_iter = iter.take_forced(len);

    todo!()
}