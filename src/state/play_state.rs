use std::sync::{Arc, Weak};
use std::time::{Duration, SystemTime};

use log::debug;
use server_util::ConnectionState;
use tokio::time;

use crate::player::Player;
use crate::packet::{SPacket, play::*};

pub(in crate::state) async fn play_state(player_ref: Arc<Player>) {
    let mut lock = player_ref.get_connection().write().await;
    lock.set_connection_state(ConnectionState::Play).await;
    drop(lock);
    debug!("Made it to the play state!");
    keep_alive(Arc::downgrade(&player_ref));

    while player_ref.is_connected().await {
        match player_ref.read_next_packet().await {
            Ok(packet) => player_ref.queue_packet(packet).await,
            Err(_) => {
                player_ref.disconnect("Connection lost").await;
                return;
            }
        }   
    }
}

fn keep_alive(weak: Weak<Player>) {
    #[allow(non_snake_case)]
    let keep_alive__config = async move {
        let mut timer = time::interval(Duration::from_secs(5));
        loop {
            timer.tick().await;
            match weak.upgrade() {
                Some(player) => {
                    //potential BUG: Client might not immediately send the keep alive packet
                    let lock = player.get_connection().write().await;
                    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
                    match time::timeout(crate::TIMEOUT, lock.send_packet(CKeepAlive_Play::new(time))).await {
                        Ok(_) => {
                            match lock.read_next_packet().await {
                                Ok(s_packet) => match s_packet {
                                    SPacket::SKeepAlive_Play(packet) => {
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
                None => break
            };
        }
    };
    tokio::spawn(keep_alive__config);
}