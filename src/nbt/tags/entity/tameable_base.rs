use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::entity_base::TraitEntityBase;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TameableBase<T: TraitTameableBase> {
    phantom_data: PhantomData<T>,

    owner: [i32;4],

    sitting: bool,

}

pub trait TraitTameableBase: TraitEntityBase {
    fn tameable_tags(&self) -> &TameableBase<Self>;

    fn tameable_tags_mut(&mut self) -> &mut TameableBase<Self>;
}

impl<T> TameableBase<T> where T: TraitTameableBase {

}


