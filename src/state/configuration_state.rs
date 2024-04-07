use std::error::Error;
use std::sync::{Arc, Weak};
use std::time::{Duration, SystemTime};

use log::debug;
use server_util::ConnectionState;
use tokio::time;

use crate::packet::configuration::*;
use crate::player::Player;
use crate::state::play_state::play_state;
use crate::SPacket;


pub(in crate::state) async fn configuration_state(player_ref: Arc<Player>) {
    let mut lock = player_ref.get_connection().write().await;
    lock.set_connection_state(ConnectionState::Configuration).await;
    drop(lock);

    debug!("Made it to the configuration state!");
    //TODO: keep alive
    //TODO: Handle plugin message.
    //TODO: Handle Client Information.
    //TODO: Clientbound plugin message
    //TODO: Feature Flags

    debug!("sending registry data");

    match player_ref.send_packet(CRegistryData::new()).await {
        Ok(_) => (),
        Err(e) => {
            player_ref.disconnect(e.to_string().as_str()).await;
            return;
        },
    }
    debug!("Sent registry data");
    
    //TODO: Update Tags
    match player_ref.send_packet(CFinishConfig::new()).await {
        Ok(_) => (),
        Err(e) => {
            player_ref.disconnect(e.to_string().as_str()).await;
            return;
        }
    }

    //TODO: Handle these packets accordingly.
    match filter_packets_until_s_acknowledge_finish_config(player_ref.clone()).await {
        Ok(_) => (),
        Err(e) => {
            player_ref.disconnect(e.to_string().as_str()).await;
            return;
        }
    }
    
    debug!("Received SAcknowledgeFinishConfig!");
    debug!("Switching to play state...");
    play_state(player_ref).await;
}

async fn filter_packets_until_s_acknowledge_finish_config(player_ref: Arc<Player>) -> Result<(), Box<dyn Error + Send + Sync>> {
    for _ in 0..3 {
        match player_ref.read_next_packet().await {
            Ok(packet) => {
                debug!("Found packet: {:?}", packet);
                match packet {
                    SPacket::SPluginMessage_Config(_) => continue,
                    SPacket::SClientInformation_Config(_) => continue,
                    SPacket::SAcknowledgeFinishConfig(_) => return Ok(()),
                    _ => return Err(format!("Wrong packet: {:?}!", packet))?
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Err("Did not find SAcknowledgeFinishConfig!")?
}

//TODO: Fix this so it actually works, although it doesn't seem necessary in the configuration state
fn keep_alive(weak: Weak<Player>) {
    #[allow(non_snake_case)]
    let keep_alive__config = async move {
        let mut timer = time::interval(Duration::from_secs(5));
        loop {
            timer.tick().await;
            match weak.upgrade() {
                Some(player) => {
                    match player.get_connection_state().await {
                        ConnectionState::Configuration => {
                            //potential BUG: Client might not immediately send the keep alive packet
                            let lock = player.get_connection().write().await;
                            let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
                            match time::timeout(crate::TIMEOUT, lock.send_packet(CKeepAlive_Config::new(time))).await {
                                Ok(_) => {
                                    match lock.read_next_packet().await {
                                        Ok(s_packet) => match s_packet {
                                            SPacket::SKeepAlive_Config(packet) => {
                                                if packet.get_keep_alive_id() == time {
                                                    drop(lock);
                                                    continue;
                                                }
                                            },
                                            _ => ()
                                        },
                                        Err(_) => ()
                                    }
                                },
                                Err(_) => (),
                            }
                            player.disconnect("Timed out.").await;
                            break;
                        },
                        _ => break
                    }
                },
                None => break
            };
        }
    };
    tokio::spawn(keep_alive__config);
}