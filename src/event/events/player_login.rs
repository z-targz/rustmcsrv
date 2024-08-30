use std::{net::SocketAddr, sync::Weak};

use crate::{data_types::text_component::{Nbt, TextComponent}, event::TraitEvent, player::Player};

#[derive(Debug, Clone)]
pub struct EventPlayerLogin {
    player: Weak<Player>,
    hostname: String,
    port: u16,
    address: SocketAddr,
    result: PlayerLoginResult,
    real_address: SocketAddr,
}

impl EventPlayerLogin {
    pub fn new(player: Weak<Player>, hostname: &str, port: u16, address: SocketAddr, real_address: SocketAddr) -> Self {
        Self {
            player: player.clone(),
            hostname: hostname.to_string(),
            port: port,
            address: address,
            result: PlayerLoginResult::Allowed,
            real_address: real_address,
        }
    }

    pub fn disallow(&mut self, result: PlayerLoginResult) {
        self.result = result;
    }

    pub fn get_player(&self) -> Weak<Player> {
        self.player.clone()
    }
    
    pub fn get_hostname(&self) -> &str {
        &self.hostname
    }
    
    pub fn get_port(&self) -> u16 {
        self.port
    }
    
    pub fn get_address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn get_real_address(&self) -> &SocketAddr {
        &self.real_address
    }
}

impl TraitEvent for EventPlayerLogin {}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PlayerLoginResult {
    Allowed,
    KickBanned { message: Option<TextComponent<Nbt>> },
    KickFull { message: Option<TextComponent<Nbt>> },
    KickOther { message: Option<TextComponent<Nbt>> },
    KickWhitelist { message: Option<TextComponent<Nbt>> },
}

impl PlayerLoginResult {
    //TODO: translations
    pub fn default_ban_message() -> TextComponent<Nbt> {
        TextComponent::builder().text("The Ban Hammer has spoken!").build()
    }

    pub fn default_kick_full_message() -> TextComponent<Nbt> {
        TextComponent::builder().text("Server is full!").build()
    }

    pub fn default_kick_whitelist_message() -> TextComponent<Nbt> {
        TextComponent::builder().text("You are not white-listed on this server").build()
    }

    pub fn default_kick_message() -> TextComponent<Nbt> {
        TextComponent::builder().text("Kicked by an operator").build()
    }
}