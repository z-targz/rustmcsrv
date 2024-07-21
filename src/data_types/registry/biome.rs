use serde::{Serialize, Deserialize};
use super::registry;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effects {
    fog_color: i32,
    water_color: i32,
    water_fog_color: i32,
    sky_color: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    foliage_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grass_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grass_color_modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    particle: Option<Particle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ambient_sound: Option<String>,  //potential BUG: Implement Option<AmbientSound>,
                                    //and custom deser that either does string or struct.
                                    //currently MC only uses string, so we're fine for now.
    #[serde(skip_serializing_if = "Option::is_none")]
    mood_sound: Option<MoodSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additions_sound: Option<AdditionsSound>,   
    #[serde(skip_serializing_if = "Option::is_none")]
    music: Option<Music>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Particle {
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<ParticleOptions>,
    probability: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParticleOptions {
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ParticleData>,
}

//TODO: move this somewhere appropriate
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum ParticleData {
    ambient_entity_effect,
    angry_villager,
    block(i32)
    //...
    //TODO: fill this out https://wiki.vg/Particles
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmbientSound {
    sound_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoodSound {
    sound: String,
    tick_delay: i32,
    block_search_extent: i32,
    offset: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdditionsSound {
    sound: String,
    tick_chance: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Music {
    sound: String,
    min_delay: i32,
    max_delay: i32,
    #[serde(deserialize_with = "registry::byte_from_bool")]
    replace_current_music: i8,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spawn {
    r#type: String,
    #[serde(rename = "maxCount")]
    max_count: i32,
    #[serde(rename = "minCount")]
    min_count: i32,
    weight: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Carvers {
    VecOfStrings(Vec<String>),
    JustAString(String),
}