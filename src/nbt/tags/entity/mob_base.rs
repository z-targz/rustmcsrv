use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{
    entity::{
        attribute::Attribute, 
        potion_effect::PotionEffect
    }, 
    item::Item, 
    nbt::tags::item_base::{
        ItemBase, 
        TraitItemBase
    }
};

use super::living_base::TraitLivingBase;



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MobBase<T> where T: TraitMobBase {
    #[serde(skip)]
    phantom_data: PhantomData<T>,

    absorption_amount: f32, //Move to LivingEntity

    #[serde(rename = "active_effects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(default)]
    active_effects: Option<Vec<PotionEffect>>, //Move to LivingEntity

    #[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(default)]
    armor_drop_chances: Option<Vec<f32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    armor_items: Option<[Option<Item>;4]>,

    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(default)]
    attributes: Option<Vec<Attribute>>,

    #[serde(rename = "body_armor_drop_chance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(default)]
    body_armor_drop_chance: Option<f32>,

    #[serde(rename = "body_armor_item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    //#[serde(default)]
    body_armor_item: Option<Item>,

    #[serde(skip_serializing_if = "Option::is_none")]
    can_pick_up_loot: Option<bool>,

    death_time: i16,

    fall_flying: bool,

    

    #[serde(skip_serializing_if = "Option::is_none")]
    hand_drop_chances: Option<[f32;2]>, //main hand, offhand

    #[serde(skip_serializing_if = "Option::is_none")]
    hand_items: Option<[Item;2]>, //main hand, offhand

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "leash")]
    leash: Option<LeashInfo>, //

    left_handed: bool,

    no_ai: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    persistance_required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sleeping_x: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sleeping_y: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sleeping_z: Option<i32>,
}

impl<T> Default for MobBase<T> where T: TraitMobBase {
    fn default() -> Self {
        Self { 
            phantom_data: Default::default(), 
            absorption_amount: Default::default(), 
            active_effects: Default::default(), 
            armor_drop_chances: Default::default(), 
            armor_items: Default::default(), 
            attributes: Default::default(), 
            body_armor_drop_chance: Default::default(), 
            body_armor_item: Default::default(), 
            can_pick_up_loot: Default::default(), 
            death_time: Default::default(), 
            fall_flying: Default::default(), 
            
            hand_drop_chances: Default::default(), 
            hand_items: Default::default(), 
            leash: Default::default(), 
            left_handed: Default::default(), 
            no_ai: Default::default(), 
            persistance_required: Default::default(), 
            sleeping_x: Default::default(), 
            sleeping_y: Default::default(), 
            sleeping_z: Default::default() 
        }
    }
}

impl<T> MobBase<T> where T: TraitMobBase {
    pub fn tick(&mut self) {

    }
}

pub trait TraitMobBase : TraitLivingBase {
    fn mob_tags(&self) -> &MobBase<Self>;
    fn mob_tags_mut(&mut self) -> &mut MobBase<Self>;
    
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LeashInfo {
    Int(i32, i32, i32),
    UUID {
        #[serde(rename = "UUID")]
        uuid: [i32;4],
    }
}
