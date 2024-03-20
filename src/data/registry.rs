extern crate nbt;

use std::{fs::File, path::Path};

use serde::{Serialize, Deserialize};


pub mod trim_pattern;
use trim_pattern::TrimPattern;

pub mod trim_material;
use trim_material::TrimMaterial;

pub mod biome;
use biome::Biome;

pub mod chat_type;
use chat_type::ChatType;

pub mod damage_type;
use damage_type::DamageType;

pub mod dimension_type;
use dimension_type::DimensionType;

//Compound Tags:

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistryData {
    //data: Map<quartz_nbt::NbtCompound>,
    data: nbt::Map<String, Registry>,
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
    element: nbt::Map<String, Element>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Element {
    TrimPattern(TrimPattern),
    TrimMaterial(TrimMaterial),
    Biome(Biome),
    ChatType(ChatType),
    DamageType(DamageType),
    DimensionType(DimensionType),
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

        let the_json = &mut  serde_json::de::from_str(get_registry().unwrap().to_string().as_str()).unwrap();
        /*let registry_data: RegistryData = RegistryData{
            data: serde_json::de::from_str(get_registry().unwrap().to_string().as_str()).unwrap(),
        };
        
        println!("{:#?}", registry_data);*/
        let result: Result<nbt::Map<String, Registry>, _> = serde_path_to_error::deserialize::<&mut _, nbt::Map<String, Registry>>(the_json);
    match result {
        Ok(_) => panic!("expected a type error"),
        Err(err) => {
            let path = err.path().to_string();
            assert_eq!(path, "dependencies.serde.version");
        }
    }
    }
}
