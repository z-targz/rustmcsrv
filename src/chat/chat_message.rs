use std::rc::Weak;
use crate::player::Player;
use crate::data_types::TextComponent;

pub struct ChatMessage {
    sender: Weak<Player>,
    message: TextComponent,
}