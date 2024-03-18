extern crate nbt;

use std::{fs::File, path::Path};

use serde::{Serialize, Deserialize};



#[derive(Debug)]
pub struct RegistryData {
    data: nbt::Map<String, Registry>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Registry {
    r#type: String,
    value: Vec<RegistryEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistryEntry {
    name: String,
    id: i32,
    element: nbt::Map<String, nbt::Value>
}

pub fn get_registry() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let file = File::open(Path::new("src/data/registry.json"))?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_registry() {
        let mut registry_data = RegistryData {
            data: nbt::Map::new(),
        };
        registry_data.data = serde_json::de::from_str(get_registry().unwrap().to_string().as_str()).unwrap();
        
        println!("{:?}", registry_data.data);
    }
}
