use std::{collections::HashMap, fs::File, path::{Path, PathBuf}};

use chashmap::CHashMap;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct RegistriesJSON {
    registries: HashMap<String, TagRegistry>
}

impl RegistriesJSON {
    pub fn new() -> Self {
        RegistriesJSON {
            registries: serde_json::de::from_reader(File::open(Path::new("generated/reports/registries.json")).unwrap()).unwrap()
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
}

pub fn read_registry_json() -> HashMap<String, Mapping> {
    RegistriesJSON::new().registries.into_iter().map(|(entry_name, entry_data)| {
        (entry_name, Mapping::from_tag_registry(entry_data))
    }).collect::<HashMap<String, Mapping>>()
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagJSON {
    values: Vec<String>,
}

impl TagJSON {
    pub fn from_file(path: PathBuf) -> Self {
        //TODO: Error handling here
        serde_json::de::from_reader(File::open(path.as_path()).unwrap()).unwrap()
    }
}

pub fn parse_directory(path: &str) -> HashMap<String, Vec<i32>> {
    let mappings = read_registry_json();
    let the_path = Path::new(path).to_path_buf();
    let mut tags: HashMap<_,_> = std::fs::read_dir(path).unwrap()
        .into_iter()
        .map(|result| result.unwrap().path())
        .map(|p| {
            match p.is_dir() {
                true => {
                    std::fs::read_dir(p.clone()).unwrap()
                        .into_iter()
                        .map(|result2| result2.unwrap().path())
                        .map(|p2| {
                            (
                                format!(
                                    "minecraft:{}/{}", 
                                    p.file_name().unwrap().to_str().unwrap(), 
                                    p2.file_stem().unwrap().to_str().unwrap()
                                ), 
                                TagJSON::from_file(p2)
                            )
                        }).collect()
                },
                false => {
                    vec![(format!("minecraft:{}",p.file_stem().unwrap().to_str().unwrap()), TagJSON::from_file(p))]
                },
            }
        })
        .flatten()
        .collect();

        
    tags.iter()
        .map(|(tag_name, tag_json)| {
            (
                tag_name, 
                tag_json.values
                    .iter()
                    .cloned()
                    .map(|unresolved_tag| {
                        resolve_tag(unresolved_tag, &tags)
                    })
                    .flatten()
                    .map(|tag_name| {
                        mappings.get(
                            &format!("minecraft:{}", the_path.file_name().unwrap().to_str().unwrap())
                        ).unwrap().mappings.get(&tag_name).unwrap()
                    }).cloned().collect()
            )
        }).map(|(x, y)| {
            (x.clone(), y)
        }).collect()
}


pub fn resolve_tag(tag: String, map: &HashMap<String, TagJSON>) -> Vec<String>{
    if tag.starts_with('#') {
        map.get(&tag.trim_start_matches('#').to_owned()).unwrap().values
            .iter()
            .cloned()
            .map(|unresolved_tag| {
                resolve_tag(unresolved_tag, map)
            }).flatten().collect()
    } else {
        vec![tag]
    }
}

mod tests {
    use std::{collections::HashMap, path::{Path, PathBuf}};

    use crate::data_types::tags::tags::{read_registry_json, RegistriesJSON, TagJSON};

    use super::{parse_directory, Mapping};


    #[test]
    fn test_read_registry_json() {
        //println!("{:#?}", RegistriesJSON::new());
        let mappings = read_registry_json();

        println!("{:?}", mappings.get("minecraft:block"));

        println!("{:?}", mappings.get("minecraft:item"));

        println!("{:?}", mappings.get("minecraft:fluid"));

        println!("{:?}", mappings.get("minecraft:entity_type"));

        println!("{:?}", mappings.get("minecraft:game_event"));
    }

    #[test]
    fn create_tag_structure() {
        let block_tags = parse_directory("generated/data/minecraft/tags/block");
        println!("{:?}", block_tags);
    }
    
    
    

}