use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    id: String,
    base: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    modifiers: Option<Vec<Modifier>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modifier {
    amount: f64,
    id: String,
    operation: ModifierOperation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum ModifierOperation {
    AddValue,
    AddMultipliedBase,
    AddMultipliedTotal,
}