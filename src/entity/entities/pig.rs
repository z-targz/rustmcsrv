use macrophilia::inherit;
use serde::{Deserialize, Serialize};


use crate::entity::Entity;
use crate::data_types::{TextComponent, text_component::Json};
use crate::entity::EnumEntityType;


#[inherit(EntityBase)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityPig {

}