use std::sync::{Arc, Weak};
use std::time::{Duration, SystemTime};

use server_util::ConnectionState;
use tokio::time;

use crate::packet::configuration::CKeepAlive_Config;
use crate::player::Player;

pub(in crate::state) async fn configuration_state(player_ref: Arc<Player>) {
    println!("Made it to the configuration state!");
    player_ref.disconnect("Not implemented yet ;)").await;
    keep_alive(Arc::downgrade(&player_ref));
    
}

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
                            let mut lock = player.get_connection().lock().await;
                            let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
                            match lock.send_packet(CKeepAlive_Config::new(time)).await {
                                Ok(_) => (),
                                Err(_) => {
                                    player.disconnect("Timed out.").await;
                                    break;
                                },
                            }
                            drop(lock);
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