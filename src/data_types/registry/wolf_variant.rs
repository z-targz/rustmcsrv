use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Biomes {
    string(String),
    list(Vec<String>)
}