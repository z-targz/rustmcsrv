
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use crate::data_types::{text_component::Json, TextComponent};
use crate::entity::{Entity, EnumEntityType};

pub trait EntityTrait {
    
}


#[macrophilia::mark_parent]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="PascalCase")]
pub struct EntityBase {
    air: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_name: Option<TextComponent<Json>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_name_visible: Option<bool>,

    fall_distance: f32,

    fire: i16,

    glowing: bool,

    has_visual_fire: bool,

    #[serde(rename = "id")]
    id: EnumEntityType,

    invulnerable: bool,

    //dX, dY, dZ in blocks/tick
    motion: [f32;3], 

    no_gravity: bool,

    on_ground: bool,

    passengers: Vec<Entity>,

    portal_cooldown: i32,

    // Order: x, y, z
    pos: [f32;3], 

    // Angles, rotation of entity from -180 to 180. 
    // May want to modify the Angle struct to use for this. 
    // Yaw first, then pitch.
    rotation: [f32;2], 

    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,

    //Scoreboard tags of entity, for this implementation for now leave this as a list of size 0.
    tags: Vec<String>, 

    //Only used by mobs that are not `freeze_imune_entity_types`
    #[serde(skip_serializing_if = "Option::is_none")]
    ticks_frozen: Option<i32>, 
    
    u_u_i_d: [i32;4], 
}





