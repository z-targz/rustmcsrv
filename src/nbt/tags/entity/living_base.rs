use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::entity_base::TraitEntityBase;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LivingBase<T> where T: TraitLivingBase {
    #[serde(skip)]
    phantom_data: PhantomData<T>,
}

impl<T> LivingBase<T> where T: TraitLivingBase {
    pub fn tick(&mut self) {

    }
}

pub trait TraitLivingBase : TraitEntityBase {
    fn living_tags(&self) -> &LivingBase<Self>;
    fn living_tags_mut(&mut self) -> &mut LivingBase<Self>;
}