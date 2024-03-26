pub mod packet;
pub mod handshake;
pub mod status;
pub mod login;
pub mod configuration;
pub mod play;

pub use packet::*;

use server_macros::register_packets;
//leaving this here to avoid breaking macro
register_packets!{}

use server_macros::create_handshake_packets;
use server_macros::create_status_packets;
use server_macros::create_login_packets;
use server_macros::create_config_packets;
use server_macros::create_play_packets;
use server_util::ConnectionState;

//leaving this here to avoid breaking macros
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