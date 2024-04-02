use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use crate::data_types::*;

use server_util::ConnectionState;

use server_macros::CPacket;
use server_macros::SPacket;


#[derive(CPacket)]
#[state(Status)]
#[id(0)]
pub struct CStatusResponse {
    json_response: JSONString,
}

#[derive(CPacket)]
#[state(Status)]
#[id(1)]
#[allow(non_camel_case_types)]
pub struct CPingResponse_Status {
    payload: i64,
}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Status)]
#[id(0)]
pub struct SStatusRequest { }

#[derive(Debug)]
#[derive(SPacket)]
#[state(Status)]
#[id(1)]
#[allow(non_camel_case_types)]
pub struct SPingRequest_Status {
    payload: i64,
}

