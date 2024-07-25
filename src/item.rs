use serde::{Deserialize, Serialize};

use crate::nbt::tags::item_base::{ItemBase, TraitItemBase};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    #[serde(flatten)]
    item_base: ItemBase<Self>
}

impl TraitItemBase for Item {
    fn get_item_tags(&self) -> &ItemBase<Self> {
        &self.item_base
    }

    fn get_item_tags_mut(&mut self) -> &mut ItemBase<Self> {
        &mut self.item_base
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryItem {
    #[serde(flatten)]
    item_base: ItemBase<Self>,

    #[serde(rename = "Slot")]
    slot: i8,
}

impl TraitItemBase for InventoryItem {
    fn get_item_tags(&self) -> &ItemBase<Self> {
        &self.item_base
    }

    fn get_item_tags_mut(&mut self) -> &mut ItemBase<Self> {
        &mut self.item_base
    }
}