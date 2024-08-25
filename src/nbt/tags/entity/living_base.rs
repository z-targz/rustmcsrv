use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::entity_base::TraitEntityBase;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LivingBase<T> where T: TraitLivingBase {
    #[serde(skip)]
    phantom_data: PhantomData<T>,

    health: f32,

    hurt_by_timestamp: i32,

    hurt_time: i16,
}

impl<T> Default for LivingBase<T> 
    where T: TraitLivingBase
{
    fn default() -> Self {
        Self { 
            phantom_data: Default::default(), 
            health: T::get_default_health(), 
            hurt_by_timestamp: Default::default(), 
            hurt_time: Default::default() 
        }
    }
}

impl<T> LivingBase<T> where T: TraitLivingBase {
    pub fn tick(&mut self) {

    }
}

pub trait TraitLivingBase : TraitEntityBase + DefaultHealth {
    fn living_tags(&self) -> &LivingBase<Self>;
    fn living_tags_mut(&mut self) -> &mut LivingBase<Self>;
}

pub trait DefaultHealth {
    fn get_default_health() -> f32;
}