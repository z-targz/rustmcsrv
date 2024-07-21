use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data_types::registry::byte_from_bool;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Overrides {
    String(String),
    List(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layer {
    height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    block: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum NoiseSettings {
    ID(String),
    Inlined {

    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AParam {
    min: f32,
    max: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Parameter {
    Float(f32),
    List(Vec<f32>),
    Compound(AParam),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameters {
    temperature: Parameter,
    humidity: Parameter,
    continentalness: Parameter,
    erosion: Parameter,
    weirdness: Parameter,
    depth: Parameter,
    offset: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParemeterPoint {
    biome: String,
    paremeters: Parameters
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MultiNoise {
    Preset {
        preset: String,
    },
    Biomes {
        biomes: Vec<ParemeterPoint>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BiomeSource {
    Checkerboard {
        r#type: String,
    },
    Fixed {
        r#type: String,

        biome: String,
    },
    MultiNoise {
        r#type: String,

        #[serde(flatten)]
        multi_noise: MultiNoise
    },
    TheEnd {
        r#type: String,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlatSettings {
    layers: Vec<Layer>,

        #[serde(skip_serializing_if = "Option::is_none")]
        biome: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        lakes: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        features: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        structure_overrides: Option<Overrides>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GeneratorSettings {
    Debug {
        r#type: String,
    },
    Flat {
        r#type: String,

        settings: FlatSettings,
    },
    Noise {
        r#type: String,

        settings: NoiseSettings,

        #[serde(skip_serializing_if = "Option::is_none")]
        biome_source: Option<BiomeSource>,
    }
}

impl Default for GeneratorSettings {
    fn default() -> Self {
        GeneratorSettings::Flat {
            r#type: "minecraft:flat".to_owned(),
            settings: FlatSettings {
                layers: vec![
                Layer {
                    height: 1,
                    block: Some("minecraft:bedrock".to_owned()),
                },
                Layer {
                    height: 2,
                    block: Some("minecraft:dirt".to_owned()),
                },
                Layer {
                    height: 1,
                    block: Some("minecraft:grass_block".to_owned()),
                },
            ],
            biome: None,
            lakes: None,
            features: None,
            structure_overrides: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dimension {
    r#type: String,

    generator: GeneratorSettings

}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldGenSettings {
    #[serde(deserialize_with = "byte_from_bool")]
    pub bonus_chest: i8,

    seed: i64,

    generate_features: i8,

    dimensions: HashMap<String, Dimension>,
}

impl Default for WorldGenSettings {
    fn default() -> Self {
        let mut the_dimensions = HashMap::new();
        the_dimensions.insert("minecraft:overworld".to_owned(), Dimension {
            r#type: "overworld".to_owned(),
            generator: GeneratorSettings::default()
        });
        Self { 
            bonus_chest: 0, 
            seed: 0, 
            generate_features: 0, 
            dimensions: the_dimensions
        }
    }
}
