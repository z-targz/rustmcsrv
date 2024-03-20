use serde::{Serialize, Deserialize};

use crate::data::CJSONTextComponent;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrimMaterial {
    asset_name: String,
    ingredient: String,
    item_model_index: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_armor_materials: Option<nbt::Value>,
    description: CJSONTextComponent,
}