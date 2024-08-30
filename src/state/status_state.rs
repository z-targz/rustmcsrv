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
    if let Ok(s_packet) =  connection.read_next_packet().await {
        if !matches!(s_packet, SPacket::SStatusRequest(_)) {
            debug!("{addr} > Incorrect packet...");
            connection.drop().await;
            return;
        }
    } else {
        connection.drop().await;
        return;
    }
    debug!("{addr} > Received SStatusRequest!");

    /*
        Send CStatusResponse
     */
    debug!("{addr} > Sending CStatusResponse...");
    if connection.send_packet(generate_status_response().await).await.is_err() {
        println!("{addr} > Error sending packet!");
        connection.drop().await;
        return;
    };
    debug!("{addr} > Sent CStatusResponse.");

    /*
        Listen for SPingRequest_Status
     */
    let payload: i64;
    if let Ok(s_packet) = connection.read_next_packet().await {
        if let SPacket::SPingRequest_Status(s_ping_request_status) = s_packet {
            payload = s_ping_request_status.get_payload();
        } else {
            debug!("{addr} > Incorrect packet");
            connection.drop().await;
            return;
        }
    } else {
        connection.drop().await;
        return;
    }
    debug!("{addr} > Received SPingRequest_Status!");

    /*
        Send SPingResponse_Status
    */
    if connection.send_packet(CPingResponse_Status::new(payload)).await.is_err() {
        debug!("{addr} > Unable to send packet SPingResponse_Status");
        connection.drop().await;
        return;
    };

    connection.drop().await;
    debug!("Connection Closed: {addr}.");
    return;
}

async fn generate_status_response() -> CStatusResponse {
    let player_count = THE_SERVER.get_num_players_async().await;
    let max_players = THE_SERVER.get_max_players();
    let motd = THE_SERVER.get_motd().to_string();

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