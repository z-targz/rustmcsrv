use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::mob_base::TraitMobBase;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LootableBase<T> where T: TraitLootableBase {
    #[serde(skip)]
    phantom_data: PhantomData<T>,
    
    death_loot_table: String,
    death_loot_table_seed: i64,
}

pub trait TraitLootableBase: TraitMobBase {
    fn lootable_tags(&self) -> &LootableBase<Self>;
    fn lootable_tags_mut(&mut self) -> &mut LootableBase<Self>;
}

impl<T> LootableBase<T> where T: TraitLootableBase {
    pub fn tick(&mut self) {
        
    }
}