use std::{fs::File, path::Path};

use std::collections::HashMap;

use quartz_nbt::io::NbtIoError;
use quartz_nbt::io::Flavor;
use quartz_nbt::serde::serialize;
use serde::Deserialize;
use super::biome;
use super::chat_type;
use super::dimension_type;

use crate::data_types::text_component::Json;
use crate::data_types::TextComponent;
use crate::data_types::NBT;

use serde::Serialize;

//Compound Tags:

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct RegistryData {
    //data: Map<quartz_nbt::NbtCompound>,
    data: HashMap<String, Registry>,
}

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct Registry {
    r#type: String,
    value: Vec<RegistryEntry>,
}

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct RegistryEntry {
    name: String,
    id: i32,
    element: Element,
}

#[derive(serde::Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Element {

    TrimPattern {
        asset_id: String,
        template_item: String,
        description: TextComponent<Json>,
        decal: i8,
    },

    TrimMaterial {
        asset_name: String,
        ingredient: String,
        item_model_index: f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        override_armor_materials: Option<HashMap<String, String>>,
        description: TextComponent<Json>,
    },

    Biome {
        has_precipitation: i8,
        temperature: f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        temperature_modifier: Option<String>,
        downfall: f32,
        effects: biome::Effects,
    },

    ChatType {
        chat: chat_type::Decoration,
        narration: chat_type::Decoration,
    },

    DamageType {
        scaling: String,
        exhaustion: f32,
        message_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        effects: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        death_message_type: Option<String>,
    },

    DimensionType {
        #[serde(skip_serializing_if = "Option::is_none")]
        fixed_time: Option<i64>,
        has_skylight: i8,
        has_ceiling: i8,
        ultrawarm: i8,
        natural: i8,
        coordinate_scale: f64,
        bed_works: i8,
        respawn_anchor_works: i8,
        min_y: i32,
        height: i32,
        logical_height: i32,
        infiniburn: String,
        effects: String,
        ambient_light: f32,
        piglin_safe: i8,
        has_raids: i8,
        monster_spawn_light_level: dimension_type::LightLevel,
        monster_spawn_block_light_limit: i32,
    },
}


pub fn get_registry() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let file = File::open(Path::new("src/data_types/registry/registry.json"))?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}

pub fn get_registry_nbt() -> Result<NBT, NbtIoError> {
    serialize(
        &serde_json::de::from_str::<HashMap<String, Registry>>(get_registry().unwrap().to_string().as_str()).unwrap(), 
        None, 
        Flavor::Uncompressed)
}

#[cfg(test)]
mod tests {
    use serde::ser::Serialize;


    use test::Bencher;

    use super::*;
    #[test]
    fn construct_registry() {

        //let the_json = get_registry().unwrap().to_string();
        //let jd = &mut serde_json::Deserializer::from_str(the_json.as_str());
        let registry_data: RegistryData = RegistryData{
            data: serde_json::de::from_str(get_registry().unwrap().to_string().as_str()).unwrap(),
        };
        
        println!("{:#?}", registry_data);
        
        /*let result: Result<nbt::Map<String, Registry>, _> = serde_path_to_error::deserialize(jd);
        match result {
            Ok(_) => panic!("expected a type error"),
            Err(err) => {
                panic!("{:?}", err);
            }
        }*/
    }
    #[test]
    fn construct_nbt() {
        let registry_data: RegistryData = RegistryData{
            data: serde_json::de::from_str(get_registry().unwrap().to_string().as_str()).unwrap(),
        };




        let mut serialized = quartz_nbt::serde::serialize(&registry_data.data, None, quartz_nbt::io::Flavor::Uncompressed).unwrap();

        serialized.drain(0..2);
        serialized[0] = 10u8;
        

        //println!("{:?}", serialized);
        println!("{:?}", serialized);
        println!();
    }

    use quartz_nbt::{snbt, NbtTag};

    #[test]
    fn snbt() {
 
        let tag: NbtTag = serde_json::de::from_str(get_registry().unwrap().to_string().as_str()).unwrap();
        println!("nbt: {}", tag.to_snbt());
        println!();
    }
    //Significantly slower
    /*
    #[bench]
    fn bench_construct_nbt_valence(bencher: &mut Bencher) {
        match serde_json::de::from_str::<HashMap<String, Registry>>(get_registry().unwrap().to_string().as_str()) {
            Ok(deserialized) => {
                bencher.iter(|| {
                    let mut serialized = Vec::new();
                    let _ = valence_nbt::to_binary(&deserialized.serialize(valence_nbt::serde::CompoundSerializer).unwrap(), &mut serialized, "");
                    serialized.drain(0..2);
                    serialized[0] = 10u8;
                });
            },
            Err(_) => panic!("JSON was invalid!"),
        };
    }*/
    
    //Significantly faster
    #[bench]
    fn bench_construct_nbt_quartz(bencher: &mut Bencher) {
        match serde_json::de::from_str::<HashMap<String, Registry>>(get_registry().unwrap().to_string().as_str()) {
            Ok(deserialized) => {
                bencher.iter(|| {
                    let mut serialized = quartz_nbt::serde::serialize(&deserialized, None, quartz_nbt::io::Flavor::Uncompressed).unwrap();
                    serialized.drain(0..2);
                    serialized[0] = 10u8;
                });
            },
            Err(_) => panic!("JSON was invalid!"),
        };
    }
}
