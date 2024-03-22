use std::sync::Arc;

use server_util::ConnectionState;

use crate::player::Player;

pub(in crate::state) async fn play_state(player_ref: Arc<Player>) {
    let mut lock = player_ref.get_connection().lock().await;
    lock.set_connection_state(ConnectionState::Play).await;
    drop(lock);
    println!("Made it to the play state!");
}