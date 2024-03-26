use crate::connection::Connection;
use crate::packet::SPacket;

use super::status_state::status_state;
use super::login_state::login_state;

/// ## Handshake State
/// 
/// Wait for a single packet `SHandshake` and transition to appropriate state `Status` or `Login`
/// 
pub(in crate) async fn handshake_state(mut connection: Connection) {
    let addr = connection.get_addr();
    let result = connection.read_next_packet().await;
    println!("Connection established: {}", addr);
    match result {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SHandshake(packet) => {
                    

                    println!("{addr} > Handshake Successful!");
                    println!("{addr} > Protocol Version: {}", packet.get_protocol_version());
                    println!("{addr} > Hostname used to connect: {}", packet.get_server_address());
                    println!("{addr} > Port used to connect: {}", packet.get_server_port());
                    match packet.get_next_state().get()
                    {
                        1 => {
                            status_state(connection).await;
                        }
                        2 => {
                            
                            login_state(connection).await;
                        }
                        _ => {
                            //Invalid login state. 
                            //This will never happen with a vanilla client unless something goes terribly wrong.
                            connection.drop();
                            return;
                        }
                    }
                }
                _ => {
                    //Incorrect packet
                    connection.drop();
                    return;
                }
            }
        },
        Err(_) => {
            //Error reading packet
            connection.drop();
            return;
        }
    }
}