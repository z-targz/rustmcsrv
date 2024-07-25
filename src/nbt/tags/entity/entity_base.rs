
use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Weak;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data_types::{text_component::Json, TextComponent};
use crate::entity::{EnumEntityType, TickableEntity};
use crate::entity::entity::Entity;



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="PascalCase")]
pub struct EntityBase<T> 
    where T: TraitEntityBase
{
    #[serde(skip)]
    phantom_data: PhantomData<T>,

    air: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_name: Option<TextComponent<Json>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_name_visible: Option<bool>,

    fall_distance: f32,

    fire: i16,

    #[serde(skip_serializing_if = "Option::is_none")]
    glowing: Option<bool>,

    has_visual_fire: bool,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    id: Option<EnumEntityType>,

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

impl<T> EntityBase<T> where T: TraitEntityBase {
    pub fn tick(&mut self) {

    }
    pub fn get_uuid(&self) -> Uuid {
        Uuid::from_u128(
            (self.u_u_i_d[0] as u128) << 96 |
            (self.u_u_i_d[1] as u128) << 64 |
            (self.u_u_i_d[2] as u128) << 32 |
            (self.u_u_i_d[3] as u128)
        )    
    }
    pub fn set_uuid(&mut self, uuid: Uuid) {
        let uuid_128 = uuid.as_u128();
        self.u_u_i_d[0] = (uuid_128 >> 96) as i32;
        self.u_u_i_d[1] = (uuid_128 >> 64) as i32;
        self.u_u_i_d[2] = (uuid_128 >> 32) as i32;
        self.u_u_i_d[3] = (uuid_128) as i32;
    }
}

pub trait TraitEntityBase: 
    Debug + Clone + Serialize + for <'a> Deserialize<'a> + Sized
{
    fn base_entity_tags(&self) -> &EntityBase<Self>;

    fn base_entity_tags_mut(&mut self) -> &mut EntityBase<Self>;
}




