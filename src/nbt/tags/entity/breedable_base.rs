//! Submissive and breedable
//! 

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::entity_base::TraitEntityBase;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BreedableBase<T: TraitBreedableBase> {
    phantom_data: PhantomData<T>,

    age: i32,

    forced_age: i32,

    in_love: i32,

    love_cause: [i32;4], //Uuid
}

pub trait TraitBreedableBase: TraitEntityBase {
    fn breedable_tags(&self) -> &BreedableBase<Self>;

    fn breedable_tags_mut(&mut self) -> &mut BreedableBase<Self>;
}

impl<T> BreedableBase<T> where T: TraitBreedableBase {
    pub fn tick(&mut self) {

    }
}


