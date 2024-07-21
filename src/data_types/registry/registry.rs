use std::marker::PhantomData;


use std::collections::HashMap;

use enum_as_inner::EnumAsInner;

use quartz_nbt::io::Flavor;
use quartz_nbt::serde::serialize;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use server_macros::pack_registry_json_files;
use maplit::hashmap;
use super::biome;
use super::chat_type;
use super::dimension_type;
use super::wolf_variant;

use crate::data_types::text_component::Json;
use crate::data_types::TextComponent;
use crate::data_types::NBT;



use serde::Serialize;

//Compound Tags:

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct RegistryData {
    data: HashMap<String, Registry>,
}

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct Registry {
    r#type: String,
    value: Vec<RegistryEntry>,
}

#[derive(serde::Serialize, Deserialize, Debug, Clone)]
pub struct RegistryEntry {
    name: String,
    id: i32,
    element: Element,
}

impl RegistryEntry {
    pub fn get_element(&self) -> &Element {
        &self.element
    }
}

pub struct NBTifiedRegistryEntry {
    pub entry_identifier: String,
    id: i32,
    pub data: NBT,
}

impl NBTifiedRegistryEntry {
    pub fn get_id(&self) -> i32 {
        self.id
    }
}

#[derive(serde::Serialize, Deserialize, Debug, Clone)]
pub struct Empty {}



pub fn byte_from_bool<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    
    struct ByteOrBool<T>(PhantomData<fn() -> T>);
    impl<'de, T> Visitor<'de> for ByteOrBool<T>
    where 
        T:Deserialize<'de>
    {
        type Value = T;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("bool or i8")
        }
        fn visit_bool<E>(self, value: bool) -> Result<T, E> 
        where 
            E: de::Error
        {
            match value {
                true => Deserialize::deserialize(de::value::I8Deserializer::new(1)),
                false => Deserialize::deserialize(de::value::I8Deserializer::new(0)), 
            }
        }
        fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
        where
            E: de::Error
        {
            Deserialize::deserialize(de::value::I8Deserializer::new(value))    
        }
    }
    deserializer.deserialize_any(ByteOrBool(PhantomData))
}

#[derive(serde::Serialize, Deserialize, Debug, Clone, EnumAsInner)]
#[serde(untagged)]
pub enum Element {
    TrimPattern {
        asset_id: String,
        template_item: String,
        description: TextComponent<Json>,
        #[serde(deserialize_with = "byte_from_bool")]
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
        #[serde(deserialize_with = "byte_from_bool")]
        has_precipitation: i8,
        temperature: f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        temperature_modifier: Option<String>,
        downfall: f32,
        effects: biome::Effects,

        #[serde(skip_serializing)]
        carvers: Option<HashMap<String, biome::Carvers>>,

        #[serde(skip_serializing)]
        creature_spawn_probability: Option<f32>,

        #[serde(skip_serializing)]
        features: Option<Vec<Vec<String>>>,

        #[serde(skip_serializing)]
        spawn_costs: Option<Empty>,

        #[serde(skip_serializing)]
        spawners: Option<HashMap<String, Vec<biome::Spawn>>>,

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
        #[serde(flatten)]
        data: DimensionProperties,
    },

    BannerPattern {
        asset_id: String,
        translation_key: String,
    },

    WolfVariant {
        wild_texture: String,
        tame_texture: String,
        angry_texture: String,
        biomes: wolf_variant::Biomes,
    },

    PaintingVariant {
        asset_id: String,
        width: i32,
        height: i32,
    },
}

impl Element {
    pub fn get_dimension_properties(&self) -> Result<&DimensionProperties,()> {
        match self {
            Element::DimensionType { data } => Ok(data),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DimensionProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_time: Option<i64>,
    #[serde(deserialize_with = "byte_from_bool")]
    has_skylight: i8,//
    #[serde(deserialize_with = "byte_from_bool")]
    has_ceiling: i8,//
    #[serde(deserialize_with = "byte_from_bool")]
    ultrawarm: i8,//
    #[serde(deserialize_with = "byte_from_bool")]
    natural: i8,//
    coordinate_scale: f64,//
    #[serde(deserialize_with = "byte_from_bool")]
    bed_works: i8,//
    #[serde(deserialize_with = "byte_from_bool")]
    respawn_anchor_works: i8,//
    min_y: i32,//
    height: i32,//
    logical_height: i32,//
    infiniburn: String,//
    effects: String,//
    ambient_light: f32,//
    #[serde(deserialize_with = "byte_from_bool")]
    piglin_safe: i8,//
    #[serde(deserialize_with = "byte_from_bool")]
    has_raids: i8,//
    monster_spawn_light_level: dimension_type::LightLevel,//
    monster_spawn_block_light_limit: i32,//
}

impl DimensionProperties {
    pub fn has_skylight(&self) -> bool {
        self.has_skylight != 0
    }

    pub fn has_ceiling(&self) -> bool {
        self.has_ceiling != 0
    }

    pub fn is_ultrawarm(&self) -> bool {
        self.ultrawarm != 0
    }

    pub fn is_natural(&self) -> bool {
        self.natural != 0
    }

    pub fn bed_works(&self) -> bool {
        self.natural != 0
    }

    pub fn respawn_anchor_works(&self) -> bool {
        self.respawn_anchor_works != 0
    }

    pub fn is_piglin_safe(&self) -> bool {
        self.piglin_safe != 0
    }

    pub fn has_raids(&self) -> bool {
        self.has_raids != 0
    }

    pub fn get_fixed_time(&self) -> Option<i64> {
        self.fixed_time
    }

    pub fn get_coordinate_scale(&self) -> f64 {
        self.coordinate_scale
    }

    pub fn get_min_y(&self) -> i32 {
        self.min_y
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_logical_height(&self) -> i32 {
        self.logical_height
    }

    pub fn get_infiniburn(&self) -> &String {
        &self.infiniburn
    }

    pub fn get_effects(&self) -> &String {
        &self.effects
    }

    pub fn get_ambient_light(&self) -> f32 {
        self.ambient_light
    }

    pub fn get_monster_spawn_light_level(&self) -> &dimension_type::LightLevel {
        &self.monster_spawn_light_level
    }

    pub fn get_monster_spawn_block_light_limit(&self) -> i32 {
        self.monster_spawn_block_light_limit
    }

}


pub fn get_registry() -> 
    Result<HashMap<String, HashMap<String, RegistryEntry>>, serde_json::error::Error> {

    let map: HashMap<String, HashMap<String, String>> = pack_registry_json_files!();
    map.into_iter()
        .map(|(registry_name, registry_data)| {
            let mut i = 0;
            Ok((
                registry_name,
                registry_data.into_iter()
                    .map(|(reg_key, json_text)| {
                        let reg_entry = 
                            RegistryEntry {
                                name: reg_key.clone(),
                                id: i,
                                element: serde_json::from_str(json_text.as_str())?
                            };
                        i += 1;
                        Ok((reg_key, reg_entry))
                    })
                    .collect::<Result<HashMap<String, RegistryEntry>, serde_json::error::Error>>()?
            ))
        })
        .collect::<Result<HashMap<String, HashMap<String, RegistryEntry>>, serde_json::error::Error>>()
}

pub fn get_registry_nbt(mut registry_entries: Vec<RegistryEntry>) -> Result<Vec<NBTifiedRegistryEntry>, Box<dyn std::error::Error>> {
    let mut nbtified_entries = Vec::new();
    registry_entries.sort_by(|e1, e2| e1.name.cmp(&e2.name));
    for entry in registry_entries {
        match serialize(&entry.element, None, Flavor::Uncompressed) {
            Ok(mut x) => {
                x.drain(0..2);
                x[0] = 10u8;
                nbtified_entries.push(NBTifiedRegistryEntry {
                    entry_identifier: entry.name,
                    id: entry.id,
                    data: x,
                });
            },
            Err(e) => { 
                return Err(Box::new(e)); 
            },
        }
    }
    Ok(nbtified_entries)
    /*serialize(
        &serde_json::de::from_str::<HashMap<String, Registry>>(get_registry().unwrap().to_string().as_str()).unwrap(), 
        None, 
        Flavor::Uncompressed)*/
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};

    use itertools::Itertools;


    use super::*;
    #[test]
    fn construct_registry() {

        //let the_json = get_registry().unwrap().to_string();
        //let jd = &mut serde_json::Deserializer::from_str(the_json.as_str());


        let dirs = vec![
            "trim_pattern", 
            "trim_material", 
            "worldgen/biome",
            "chat_type",
            "damage_type",
            "dimension_type",
            "banner_pattern",
            "wolf_variant",
        ];
        let mut registries: HashMap<String, Registry> = HashMap::with_capacity(dirs.len());
        
        dirs.into_iter().for_each(|dir| {
            let mut i = 0;
            let mut registry_entries: Vec<RegistryEntry> = Vec::new();

            fs::read_dir(format!("generated/data/minecraft/{}", dir)).unwrap()
            .map(|f| f.unwrap().path())
            .filter(|f| f.is_file())
            .sorted_by(|p, p2| {
                p.cmp(p2)
            })
            .map(|p| (p.clone(), File::open(p).unwrap()))
            .for_each(|(p, f)| {
                println!("registry: {}",dir);
                println!("{}",fs::read_to_string(p.clone()).unwrap());
                registry_entries.push(RegistryEntry {
                    name: format!("minecraft:{}", p.file_stem().unwrap().to_str().unwrap()),
                    id: i,
                    element: serde_json::from_reader(f).unwrap(),
                });
                i += 1;
            });

            registries.insert(format!("minecraft:{}", dir), Registry { 
                r#type: format!("minecraft:{}",dir), value: registry_entries
            });
        });

        let registry_data: RegistryData = RegistryData {
            data: registries,
        };
        
        /*RegistryData{
            data: serde_json::de::from_str(get_registry().unwrap().to_string().as_str()).unwrap(),
        };*/

        
        
        println!("{:#?}", registry_data);
        
        /*let result: Result<nbt::Map<String, Registry>, _> = serde_path_to_error::deserialize(jd);
        match result {
            Ok(_) => panic!("expected a type error"),
            Err(err) => {
                panic!("{:?}", err);
            }
        }*/
    }
    //#[test]
    //fn construct_nbt() {
        //let registry_data: RegistryData = RegistryData{
        //    data: serde_json::de::from_str(get_registry_nbt("trim_pattern").unwrap().data.to_string().as_str()).unwrap(),
        //};




        //let mut serialized = quartz_nbt::serde::serialize(&registry_data.data, None, quartz_nbt::io::Flavor::Uncompressed).unwrap();

        //serialized.drain(0..2);
        //serialized[0] = 10u8;
        

        //println!("{:?}", serialized);
        //println!("{:?}", serialized);
        //println!();
    //}

    //use quartz_nbt::{snbt, NbtTag};

    //#[test]
    /*fn snbt() {
 
        let tag: NbtTag = serde_json::de::from_str(get_registry().unwrap().to_string().as_str()).unwrap();
        println!("nbt: {}", tag.to_snbt());
        println!();
    }*/
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
    /*#[bench]
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
    }*/
}
