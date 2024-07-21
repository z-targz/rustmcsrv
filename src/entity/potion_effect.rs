use serde::{Deserialize, Serialize};
use server_macros::generate_potion_effect_id_enum;

generate_potion_effect_id_enum!{}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PotionEffect {
    ambient: bool,
    amplifier: i8,
    duration: i32,
    hidden_effect: Option<Box<PotionEffect>>,
    id: EnumMobEffect,
    show_icon: bool,
    show_particles: bool,
}
