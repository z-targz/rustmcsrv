use server_macros::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::entity::potion_effect::PotionEffect;

use super::Item;



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Mob {
    absorption_amount: f32,
    #[serde(rename = "active_effects")]
    active_effects: Option<Vec<PotionEffect>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    armor_drop_chances: Option<Vec<f32>>,
    armor_items: Vec<Item>,
}