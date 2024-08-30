use log::{debug, info};

use crate::connection::Connection;
use crate::packet::SPacket;

use super::status_state::status_state;
use super::login_state::login_state;

/// ## Handshake State
/// 
/// Wait for a single packet `SHandshake` 
/// and transition to appropriate state `Status` or `Login`
/// 
pub(in crate) async fn handshake_state(mut connection: Connection) {
    let addr = connection.get_addr();
    if let Ok(s_packet) = connection.read_next_packet().await {
        info!("Connection established: {}", addr);
        if let SPacket::SHandshake(packet) = s_packet {
            debug!("{addr} > Handshake Successful!");
            debug!(
                "{addr} > Protocol Version: {}", 
                packet.get_protocol_version()
            );
            debug!(
                "{addr} > Hostname used to connect: {}", 
                packet.get_server_address()
            );
            debug!(
                "{addr} > Port used to connect: {}", 
                packet.get_server_port()
            );
            connection.set_hostname(packet.get_server_address());
            connection.set_port(packet.get_server_port());

            match packet.get_next_state().get()
            {
                1 => {
                    status_state(connection).await;
                }
                2 => {
                    
                    login_state(connection).await;
                }
                _ => {
                    // Invalid login state. 
                    // This will never happen with a vanilla client
                    // unless something goes terribly wrong.
                    connection.drop().await;
                    return;
                }
            }
        } else {
            connection.drop().await;
            return;
        }
    } else {
        connection.drop().await;
        return;
    }
}