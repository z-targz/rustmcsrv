use std::error::Error;
use std::sync::Arc;

use tokio::time::timeout;
use uuid::Uuid;
use serde::Deserialize;

use crate::data::Property;
use crate::data::PropertyArray;
use crate::player::Player;
use crate::state::configuration_state::configuration_state;
use crate::RUNTIME;
use crate::THE_SERVER;
use crate::ONLINE_MODE;
use crate::connection::Connection;
use crate::packet::SPacket;
use crate::packet::login::*;
use crate::TIMEOUT;

/// ## Login Sequence:
/// 
/// __C -> S__ &nbsp; : &nbsp; SLoginStart
/// 
/// __S -> C__ &nbsp; : &nbsp; CEncryptionRequest //TODO: Not required for offline mode
/// 
/// __C -> S__ &nbsp; : &nbsp; SEncryptionResponse //Only if we sent the above packet
/// 
/// __Server auth step__ //TODO:
/// 
/// __S -> C__ CSetCompression //Optional
/// 
/// __S -> C__ CLoginSuccess
/// 
/// __C -> S__ SLoginAcknowledged
/// 
pub(in crate::state) async fn login_state(connection: Connection) {
    let addr = connection.get_addr();
    /*
        Listen for SLoginStart
    */
    println!("Listening for SLoginStart...");

    let player_ref: Arc<Player>;
    match connection.read_next_packet().await {
        Ok(s_packet) => {
            match s_packet {
                SPacket::SLoginStart(packet) => {
                    let player_name = packet.get_name().clone();

                    let player_uuid;
                    match get_player_uuid(&player_name).await {
                        Ok(uuid) => player_uuid = uuid,
                        Err(_) => {
                            RUNTIME.spawn(async move {
                                //TODO: move the timeout into the send packet function
                                let _ = connection.send_packet(CDisconnect::new("Mojang API appears to be down :(".to_string())).await;
                            });
                            return;
                        }
                    };

                    match THE_SERVER.get_player_by_name(&player_name) {
                        //TODO: If this returns an error somehow, temp ipban for 30s
                        Some(p) => p.upgrade().unwrap().disconnect(&"Logged in from another location.".to_string()).await,
                        None => (),
                    }

                    println!("Player {player_name} ({player_uuid}) logged in from {addr}.");

                    let player = Player::new(player_name, player_uuid, connection);
                    println!("Registering player...");    
                    
                    player_ref = THE_SERVER.register_player(player).await;

                    println!("Registered player!");              
                },
                _ => {
                    println!("Incorrect packet.");
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
    //TODO: Everything in between

    match player_ref.send_packet(CLoginSuccess::new(
        player_ref.get_uuid(), 
        player_ref.get_name().clone(), 
        get_player_property_array(player_ref.get_uuid()).await
    )).await {
        Ok(_) => (),
        Err(_) => {
            player_ref.disconnect(&"Connection closed.".to_string()).await;
            return;
        }
    }

    match player_ref.read_next_packet().await {
        Ok(s_packet) => match s_packet {
            SPacket::SLoginAcknowledged(_) => (),
            _ => {
                player_ref.disconnect(&"Incorrect packet.".to_string()).await;
                return;
            }
        },
        Err(_) => {
            player_ref.disconnect(&"Invalid packet.".to_string()).await;
            return;
        }
    };
    configuration_state(player_ref);
}

#[derive(Deserialize)]
#[allow(unused)]
struct APISessionResponse {
    id: String,
    name: String,
    properties: Vec<Property>,
    //profile_actions: Vec<()>, //if we can ignore this, perfect
}

#[derive(Deserialize)]
#[allow(unused)]
struct APIProfileResponse {
    id: String,
    name: String,
}

async fn get_player_property_array(player_uuid: Uuid) -> PropertyArray {
    match timeout(TIMEOUT, reqwest::get(format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", player_uuid.to_string()))).await {
        Ok(result) => match result {
            Ok(response) => match response.text().await {
                Ok(text) => match serde_json::from_str::<APISessionResponse>(text.as_str()) {
                    Ok(api_response) => {
                        return api_response.properties;
                    },
                    Err(_) => ()
                }
                Err(_) => (),
            },
            Err(_) => ()
        },
        Err(_) => ()
    }
    vec![] //default value
}

async fn get_player_uuid(player_name: &String) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
    if ONLINE_MODE {
        match reqwest::get(format!("https://api.mojang.com/users/profiles/minecraft/{}", player_name)).await {
            Ok(response) => match response.text().await {
                Ok(text) => match serde_json::from_str::<APIProfileResponse>(text.as_str()) {
                    Ok(api_response) => {
                        match Uuid::parse_str(api_response.id.as_str()) {
                            Ok(uuid) => Ok(uuid),
                            Err(_) => Err("Malformed API response")?
                        }
                    },
                    Err(_) => {
                        return Err("Malformed API response")?
                    }
                }
                Err(_) => Err("Malformed API response")?,
            },
            Err(e) => Err(e)?
        }
    } else {
        Ok(uuid::Builder::from_md5_bytes(md5::compute(format!("OfflinePlayer:{player_name}").as_bytes()).0).into_uuid())
    }
}