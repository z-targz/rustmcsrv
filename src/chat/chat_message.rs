use std::rc::Weak;
use crate::data_types::text_component::Nbt;
use crate::player::Player;
use crate::data_types::TextComponent;

pub struct ChatMessage {
    sender: Weak<Player>,
    message: TextComponent<Nbt>,
}