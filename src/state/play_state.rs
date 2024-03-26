use std::sync::Arc;

use server_util::ConnectionState;

use crate::{data_types::TextComponent, player::Player};

pub(in crate::state) async fn play_state(player_ref: Arc<Player>) {
    let mut lock = player_ref.get_connection().lock().await;
    lock.set_connection_state(ConnectionState::Play).await;
    drop(lock);
    println!("Made it to the play state!");

    let disconnect_reason = TextComponent::new("#")
    .obfuscated().color(4u8)
    .add_extra(&TextComponent::new(" Test :3 ")
        .reset_fmt()
        .color_hex(0xdd33dd))
    .add_extra(&TextComponent::new("#").obfuscated().color(0x4u8));
    println!("tc: {:?}", disconnect_reason);
    player_ref.disconnect_tc(disconnect_reason).await;
}