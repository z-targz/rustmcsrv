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

