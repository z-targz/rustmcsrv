use serde::{Serialize, Deserialize};

use crate::data::CJSONTextComponent;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrimPattern {
    asset_id: String,
    template_item: String,
    description: CJSONTextComponent,
    decal: i8,
}