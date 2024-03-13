use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use crate::data::*;

use server_util::ConnectionState;

use server_macros::CPacket;
use server_macros::SPacket;

#[derive(CPacket)]
#[state(Status)]
#[id(0)]
pub struct CStatusResponse {
    json_response: String,
}

#[derive(CPacket)]
#[state(Status)]
#[id(1)]
pub struct CPingResponse_Status {
    payload: i64,
}

#[derive(SPacket)]
#[state(Status)]
#[id(0)]
pub struct SStatusRequest { }

#[derive(SPacket)]
#[state(Status)]
#[id(1)]
pub struct SPingRequest_Status {
    payload: i64,
}

