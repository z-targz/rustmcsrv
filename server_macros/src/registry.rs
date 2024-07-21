use std::{collections::HashMap, fs::File, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistriesJSON {
    registries: HashMap<String, TagRegistry>
}

impl RegistriesJSON {
    pub fn new() -> Self {
        let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let path = Path::new(cargo_manifest_dir.as_str())
            .join("generated")
            .join("reports")
            .join("registries.json");

        RegistriesJSON {
            registries: serde_json::de::from_reader(
                File::open(path).unwrap()
            ).unwrap()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagRegistry {
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    entries: HashMap<String, Entry>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    protocol_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mapping {
    mappings: HashMap<String, i32>,
}

impl Mapping {
    pub fn from_tag_registry(tag_registry: TagRegistry) -> Self {
        Mapping {
            mappings: tag_registry.entries
                .into_iter()
                .map(|(entry_name, entry)| {
                    (entry_name, entry.protocol_id)
                }).collect::<HashMap<String, i32>>(),
        }
    }

    pub fn get_mappings(&self) -> &HashMap<String, i32> {
        &self.mappings
    }
}

pub fn read_registry_json() -> HashMap<String, Mapping> {
    RegistriesJSON::new().registries.into_iter().map(|(entry_name, entry_data)| {
        (entry_name, Mapping::from_tag_registry(entry_data))
    }).collect::<HashMap<String, Mapping>>()
}