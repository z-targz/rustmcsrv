use std::error::Error;

use super::{Packet, Serverbound, Clientbound};

use crate::data_types::*;
use uuid::Uuid;

use server_util::ConnectionState;

use server_macros::CPacket;
use server_macros::SPacket;

#[derive(CPacket, Debug)]
#[state(Login)]
#[id(0)]
#[allow(non_camel_case_types)]
pub struct CDisconnect_Login {
    reason: String,
}

#[derive(CPacket, Debug)]
#[state(Login)]
#[id(1)]
pub struct CEncryptionRequest {
    server_id: String, //Leave empty
    pub_key: PrefixedByteArray,
    verify_token: PrefixedByteArray,
}

#[derive(CPacket, Debug)]
#[state(Login)]
#[id(2)]
pub struct CLoginSuccess {
    uuid: Uuid,
    username: String,
    properties: PropertyArray,
    strict_error_handling: bool,
}

#[derive(CPacket, Debug)]
#[state(Login)]
#[id(3)]
pub struct CSetCompression {
    threshold: VarInt,
}

#[derive(CPacket, Debug)]
#[state(Login)]
#[id(4)]
#[allow(non_camel_case_types)]
pub struct CPluginRequest_Login {
    message_id: VarInt,
    identifier: String, //TODO: Implement custom type alias
    data: InferredByteArray,
}

#[derive(Debug,)]
#[derive(SPacket)]
#[state(Login)]
#[id(0)]
pub struct SLoginStart {
    name: String,
    uuid: Uuid, //Unused
}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Login)]
#[id(1)]
#[allow(unused)]
pub struct SEncryptionResponse {
    shared_secret: PrefixedByteArray,
    verify_token: PrefixedByteArray,
}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Login)]
#[id(2)]
#[allow(unused)]
pub struct SLoginPluginResponse {
    message_id: VarInt,
    data: Option<InferredByteArray>,
}

#[derive(Debug)]
#[derive(SPacket)]
#[state(Login)]
#[id(3)]
pub struct SLoginAcknowledged { }