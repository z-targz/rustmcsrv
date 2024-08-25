use serde::{Deserialize, Serialize};
use server_macros::{entity, Breedable, Entity, LivingEntity, Lootable, Mob, TickableEntity};




use crate::{
    entity::TickableEntity, 
    nbt::tags::entity::{
        breedable_base::BreedableBase, 
        entity_base::EntityBase, 
        living_base::{
            DefaultHealth, 
            LivingBase
        }, 
        lootable_base::LootableBase, 
        mob_base::MobBase
    }
};


#[entity]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Entity, LivingEntity, Mob, Lootable, Breedable, TickableEntity)]
pub struct EntityPig {
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
    #[tickable]
    #[breedable_base]
    breedable_base: BreedableBase<Self>,
    
    #[serde(flatten)]
    #[lootable_base]
    lootable_base: LootableBase<Self>,



    #[serde(rename = "Saddle")]
    has_saddle: bool,
}

impl DefaultHealth for EntityPig {
    fn get_default_health() -> f32 {
        10.0
    }
}
