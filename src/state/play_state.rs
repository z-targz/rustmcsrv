use std::sync::{Arc, Weak};
use std::time::{Duration, SystemTime};

use log::debug;
use server_util::ConnectionState;
use tokio::time;
use tokio::sync::mpsc;

use crate::data_types::text_component::Formatting;
use crate::data_types::{Identifier, TextComponent, VarInt};
use crate::player::Player;
use crate::packet::{SPacket, play::*};
use crate::THE_SERVER;

pub(in crate::state) async fn play_state(player_ref: Arc<Player>) {
    let mut lock = player_ref.get_connection().lock().await;
    lock.set_connection_state(ConnectionState::Play).await;
    drop(lock);
    debug!("Made it to the play state!");
    let tx = keep_alive(Arc::downgrade(&player_ref));

    let reason = 
        TextComponent::builder()
            .text("")
            .formatting(Formatting::builder().color_rgb(0xff00ff).build())
            .add_extra(
                TextComponent::builder()
                    .text("# ")
                    .formatting(Formatting::builder().color(0x4).obfuscated(true).build())
                    .build()
            ).add_extra(
                TextComponent::builder()
                    .text("Test ")
                    .reset_fmt()
                    .build()
            ).add_extra(
                TextComponent::builder()
                    .text(":3")
                    .formatting(Formatting::builder().bold(true).build())
                    .build()
            ).add_extra(
                TextComponent::builder()
                    .text(" #")
                    .reset_fmt()
                    .formatting(Formatting::builder().color(0x4).obfuscated(true).build())
                    .build()
            ).build();

    debug!("sending login play packet");
    match player_ref.send_packet(CLogin_Play::new(
        THE_SERVER.get_next_eid().await, 
        false,
        VarInt::new(3), 
        vec![
            Identifier::new("minecraft:overworld").unwrap(), 
            Identifier::new("minecraft:the_nether").unwrap(), 
            Identifier::new("minecraft:the_end").unwrap()
        ],
        VarInt::new(THE_SERVER.get_properties().get_max_players()),
        VarInt::new(THE_SERVER.get_properties().get_view_distance()),
        VarInt::new(THE_SERVER.get_properties().get_simulation_distance()),
        false,
        true,
        false,
        VarInt::new(0),
        Identifier::new("minecraft:overworld").unwrap(),
        4297447447117712613,
        1,
        1,
        false,
        true,
        None,
        VarInt::new(0),
        false
    )).await {
        Ok(_) => (),
        Err(e) => {
            player_ref.disconnect(e.to_string().as_str()).await;
            return;
        }
    }
    debug!("send login play complete");

    player_ref.disconnect_tc(reason).await;

    while player_ref.is_connected().await {
        match player_ref.read_next_packet().await {
            Ok(SPacket::SKeepAlive_Play(packet)) => {
                let _ = tx.send(packet.get_keep_alive_id()).await;
            }
            Ok(packet) => player_ref.queue_packet(packet).await,
            Err(_) => {
                player_ref.disconnect("Connection lost").await;
                return;
            }
        }   
    }
}

//TODO: this really needs reworking
fn keep_alive(weak: Weak<Player>) -> mpsc::Sender<i64>{
    let (tx, mut rx) = mpsc::channel::<i64>(1);
    let keep_alive = async move {
        let mut timer = time::interval(Duration::from_secs(5));
        loop {
            timer.tick().await;
            match weak.upgrade() {
                Some(player) => {
                    //potential BUG: Client might not immediately send the keep alive packet
                    let lock = player.get_connection().lock().await;
                    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
                    match time::timeout(crate::TIMEOUT, lock.send_packet(CKeepAlive_Play::new(time))).await {
                        Ok(_) => {
                            match rx.recv().await {
                                Some(long) => {
                                    if long == time {
                                        continue;
                                    }
                                }
                                None => ()
                            }
                        },
                        Err(_) => (),
                    }
                    player.disconnect("Timed out.").await;
                    break;
                },
                None => break
            };
        }
    };
    tokio::spawn(keep_alive);
    tx
}