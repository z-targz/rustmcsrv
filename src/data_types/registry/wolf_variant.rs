use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum Biomes {
    string(String),
    list(Vec<String>)
}