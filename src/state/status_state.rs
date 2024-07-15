use std::path::Path;

use base64::prelude::*;
use log::debug;
use server_util::ConnectionState;

use crate::connection::Connection;
use crate::SPacket;
use crate::packet::status::*;
use crate::THE_SERVER;

/// ## Status ping sequence:
/// 
/// __C -> S__ &nbsp; : &nbsp; SStatusRequest
/// 
/// __S -> C__ &nbsp; : &nbsp; CStatusResponse
/// 
/// __C -> S__ &nbsp; : &nbsp; SPingRequest_Status
/// 
/// __S -> C__ &nbsp; : &nbsp; SPingResponse_Status
/// 
pub(in crate::state) async fn status_state(mut connection: Connection) {
    connection.set_connection_state(ConnectionState::Status).await;
    let addr = connection.get_addr();
    debug!("{addr} > Next State: Status(1)");
    /*
        Listen for SStatusRequest
    */
    debug!("{addr} > Listening for SStatusRequest...");
    match connection.read_next_packet().await {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SStatusRequest(_) => (),
                _ => {
                    debug!("{addr} > Incorrect packet...");
                    connection.drop().await;
                    return;
                }
            }
        },
        Err(_) => {
            connection.drop().await;
            return;
        }
    }
    debug!("{addr} > Received SStatusRequest!");

    /*
        Send CStatusResponse
     */
    debug!("{addr} > Sending CStatusResponse...");
    match connection.send_packet(generate_status_response()).await {
        Ok(_) => (),
        Err(_) => {
            println!("{addr} > Error sending packet!");
            connection.drop().await;
            return;
        }
    };
    debug!("{addr} > Sent CStatusResponse.");

    /*
        Listen for SPingRequest_Status
     */
    let payload: i64;
    match connection.read_next_packet().await {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SPingRequest_Status(s_ping_request_status) => {
                    payload = s_ping_request_status.get_payload();
                },
                _ => {
                    //Incorrect packet
                    debug!("{addr} > Incorrect packet");
                    connection.drop().await;
                    return;
                }
            }
        },
        Err(_) => {
            connection.drop().await;
            return;
        }
    }
    debug!("{addr} > Received SPingRequest_Status!");

    /*
        Send SPingResponse_Status
    */
    match connection.send_packet(CPingResponse_Status::new(payload)).await {
        Ok(_) => (),
        Err(_) => {
            debug!("{addr} > Unable to send packet SPingResponse_Status");
            connection.drop().await;
            return;
        }
    };

    connection.drop().await;
    debug!("Connection Closed: {addr}.");
    return;
}

fn generate_status_response() -> CStatusResponse {
    let player_count = THE_SERVER.get_players_iter().count();
    let max_players = THE_SERVER.get_max_players();
    let motd = THE_SERVER.get_motd().clone();

    let mut result = String::new();
    let data = std::fs::read(Path::new("server-icon.png"));
    match data {
        Ok(vec) => {
            BASE64_STANDARD.encode_string(vec, &mut result);
        }
        Err(_) => {
            let r: Result<&str, std::io::Error> = server_macros::base64_image!("favicon.png");
            result = r.unwrap().to_string();
        }
    }
    let favicon_str = format!("data:image/png;base64,{}", result);
    CStatusResponse::new(json!({
        "version": {
            "name": "1.21",
            "protocol": 767
        },
        "players": {
            "max": max_players as i32,
            "online": player_count,
            "sample": [] //TODO: make the sample
        },
        "description": {
            "text": motd
        },
        "favicon": favicon_str,
        "enforceSecureChat": false,
        //"previewsChat": false
    }).to_string())
}