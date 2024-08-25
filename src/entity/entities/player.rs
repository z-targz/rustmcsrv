use std::{default, sync::Weak};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use server_macros::{entity, Entity, LivingEntity, Mob, TickableEntity};

use crate::{
    data_types::{
        DeathLocation, 
        Pos
    }, 
    entity::{
        Entity, 
        TickableEntity
    }, 
    item::{
        InventoryItem, 
        Item
    }, 
    nbt::tags::entity::{
        entity_base::EntityBase, 
        living_base::{LivingBase, DefaultHealth},
        mob_base::MobBase
    }
};

use super::parrot::EntityParrot;



#[entity]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Entity, LivingEntity, Mob, TickableEntity)]
#[serde(rename_all = "PascalCase")]
#[tick = "Self::player_tick_function"]
pub struct EntityPlayer {
    //Tags
    #[serde(flatten)]
    #[tickable]
    #[entity_base]
    entity_base: EntityBase<Self>,

    #[serde(flatten)]
    #[tickable]
    #[living_base]
    living_base: LivingBase<Self>,

    #[serde(flatten)]
    #[tickable]
    #[mob_base]
    mob_base: MobBase<Self>,

    #[serde(rename = "abilities")]
    abilities: PlayerAbilities,

    data_version: i32,

    dimension: String, //TODO: replace with DimensionType

    #[serde(skip_serializing_if = "Option::is_none")]
    ender_items: Option<Vec<InventoryItem>>,

    #[serde(rename = "enteredNetherPosition")]
    entered_nether_position: Option<Pos>,

    #[serde(rename = "foodExhaustionLevel")]
    food_exhaustion_level: f32,

    #[serde(rename = "foodLevel")]
    food_level: i32,

    #[serde(rename = "foodSaturationLevel")]
    food_saturation_level: f32,

    #[serde(rename = "foodTickTimer")]
    food_tick_timer: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    inventory: Option<Vec<InventoryItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_death_location: Option<DeathLocation>,

    #[serde(rename = "playerGameType")]
    player_game_type: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previousPlayerGameType")]
    previous_player_game_type: Option<i32>,

    #[serde(rename = "recipeBook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    recipe_book: Option<RecipeBook>,

    #[serde(skip_deserializing)]
    // #[serde(deserialize_with = "deserialize_weak")]
    //TODO: Deserialize by finding entity by UUID
    root_vehicle: Option<RootVehicle>,

    score: i32,
    
    #[serde(rename = "seenCredits")]
    seen_credits: bool,

    #[serde(skip)]
    selected_item: Option<Item>,

    selected_item_slot: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    shoulder_entity_left: Option<EntityParrot>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shouler_entity_right: Option<EntityParrot>,

    sleep_timer: i16,

    #[serde(skip_serializing_if = "Option::is_none")]
    spawn_forced: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spawn_x: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spawn_y: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spawn_z: Option<i32>,

    xp_level: i32,

    xp_p: f32,

    xp_seed: i32,

    xp_total: i32,
}

impl DefaultHealth for EntityPlayer {
    fn get_default_health() -> f32 {
        20.0
    }
}

impl Default for EntityPlayer {
    fn default() -> Self {
        Self { 
            entity_base: Default::default(), 
            living_base: Default::default(), 
            mob_base: Default::default(), 
            abilities: Default::default(), 
            data_version: 3953, 
            dimension: "minecraft:overworld".to_owned(), 
            ender_items: None, 
            entered_nether_position: None, 
            food_exhaustion_level: 0.0f32, 
            food_level: 20, 
            food_saturation_level: 5.0, 
            food_tick_timer: 0, 
            inventory: Some(vec![]), 
            last_death_location: Default::default(), 
            player_game_type: Default::default(), 
            previous_player_game_type: Default::default(), 
            recipe_book: None, 
            root_vehicle: None, 
            score: 0, 
            seen_credits: false, 
            selected_item: None, 
            selected_item_slot: 1, 
            shoulder_entity_left: None, 
            shouler_entity_right: None, 
            sleep_timer: Default::default(), 
            spawn_forced: Default::default(), 
            spawn_x: None, 
            spawn_y: None, 
            spawn_z: None, 
            xp_level: 0, 
            xp_p: 0.0f32, 
            xp_seed: 0, 
            xp_total: 0 
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RootVehicle {
    attach: [i32;4], //UUID
    #[serde(skip_serializing_if = "Weak::is_strong_count_zero")]
    #[serde(serialize_with = "Weak::serialize_weak")]
    #[serde(skip_deserializing)]
    entity: Weak<Entity>,
}

impl EntityPlayer {
    fn player_tick_function(&mut self) {

    }
}

pub trait WeakEntity {
    fn is_strong_count_zero(&self) -> bool;
    fn serialize_weak<S>(data: &Self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer;
    fn deserialize_weak<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>, Self: Sized;
}

impl WeakEntity for Weak<Entity> {
    fn is_strong_count_zero(&self) -> bool {
        self.strong_count() > 0   
    }

    fn serialize_weak<S>(data: &Self, serializer: S) -> Result<S::Ok, S::Error> 
        where S: Serializer
    {
        if !data.is_strong_count_zero() {
            data.upgrade().unwrap().serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }
    
    fn deserialize_weak<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>, Self: Sized
    {
        let entity = Entity::deserialize(deserializer)?;
        todo!()
    }
}





#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct PlayerAbilities {
    flying: bool,
    fly_speed: f32,
    instabuild: bool,
    invulnerable: bool,
    may_build: bool,
    may_fly: bool,
    walk_speed: f32, //defaults to 0.1
}

impl Default for PlayerAbilities {
    fn default() -> Self {
        Self { 
            flying: Default::default(), 
            fly_speed: 0.05, 
            instabuild: Default::default(), 
            invulnerable: Default::default(), 
            may_build: true, 
            may_fly: Default::default(), 
            walk_speed: 0.1 
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecipeBook {
    recipes: Vec<String>,
    to_be_displayed: Vec<String>,
    is_filtering_craftable: bool,
    is_gui_open: bool,
    is_furnace_filtering_craftable: bool,
    is_furnace_gui_open: bool,
    is_blacting_furnace_filtering_craftable: bool,
    is_blasting_furnace_gui_open: bool,
    is_smoker_filtering_craftable: bool,
    is_smoker_gui_open: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WardenSpawnTracker {
    warning_level: i32,
    cooldown_ticks: i32,
    ticks_since_last_warning: i32,
}

impl Default for WardenSpawnTracker {
    fn default() -> Self {
        Self {
            warning_level: 0, 
            cooldown_ticks: 200, 
            ticks_since_last_warning: 0 
        }
    }
}

pub trait TraitPlayer {}

impl TraitPlayer for EntityPlayer {}