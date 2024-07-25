use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::{entity_base::TraitEntityBase, mob_base::TraitMobBase};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Brain<T> 
    where T: TraitEntityBase
{
    #[serde(skip)]
    phantom_data: PhantomData<T>,
    
    memories: Memories,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Memories {
    Piglin {},
    Warden {},
    Camel {},
    IronGolem {},
    Axolotl {},
    Villager {},
    Frog {},
    Allay {},
    Goat {},
    Sniffer {},
    
}

pub trait TraitHasBrain: TraitMobBase {
    fn get_brain(&self) -> &Brain<Self>;
    fn get_brain_mut(&mut self) -> &mut Brain<Self>;
}