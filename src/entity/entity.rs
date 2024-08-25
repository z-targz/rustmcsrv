
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

use crate::nbt::tags::entity::entity_base::{IsTickable, TraitEntityBase};

server_macros::generate_entity_id_enum!{}

server_macros::create_entity_enum!{}

pub trait TickableEntity: TraitEntityBase {
    fn tick(&mut self);
}

impl<T> IsTickable for T 
    where T: TickableEntity
{
    const IS_TICKABLE: bool = true;
}
