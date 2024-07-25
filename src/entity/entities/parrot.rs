use serde::{Deserialize, Serialize};
use server_macros::{entity, Breedable, Entity, LivingEntity, Lootable, Mob, Tameable, TickableEntity};




use crate::{
    entity::TickableEntity, 
    nbt::tags::entity::{
        tameable_base::TameableBase, 
        entity_base::EntityBase, 
        living_base::LivingBase, 
        lootable_base::LootableBase, 
        mob_base::MobBase,
    }
};


#[entity]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Entity, LivingEntity, Mob, Lootable, Tameable, TickableEntity)]
pub struct EntityParrot {
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

    #[serde(flatten)]
    #[tameable_base]
    tameable_base: TameableBase<Self>,
    
    #[serde(flatten)]
    #[lootable_base]
    lootable_base: LootableBase<Self>,

    #[serde(rename = "Variant")]
    variant: i32,
}

