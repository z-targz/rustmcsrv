use std::{net::SocketAddr, sync::{Arc, Weak}};

use crate::{event::TraitEvent, player::Player};

#[derive(Debug, Clone)]
pub struct EventPlayerLogin {
    player: Weak<Player>,
    hostname: String,
    address: SocketAddr
}

pub enum PlayerLoginResult {
    Allowed,
    KickBanned,
    KickFull,
    KickOther,
    KickWhitelist,
}