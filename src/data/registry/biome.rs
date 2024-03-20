use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Biome {
    has_precipitation: i8,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_modifier: Option<String>,
    downfall: f32,
    effects: Effects,
}

#[derive(Serialize, Deserialize, Debug)]
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
    ambient_sound: Option<AmbientSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mood_sound: Option<MoodSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additions_sound: Option<AdditionsSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    music: Option<Music>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Particle {
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<ParticleOptions>,
    probability: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticleOptions {
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ParticleData>,
}

//TODO: move this somewhere appropriate
#[derive(Serialize, Deserialize, Debug)]
pub enum ParticleData {
    ambient_entity_effect,
    angry_villager,
    block(i32),
    //...
    //TODO: fill this out https://wiki.vg/Particles
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmbientSound {
    sound_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoodSound {
    sound: String,
    tick_delay: i32,
    block_search_extent: i32,
    offset: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionsSound {
    sound: String,
    tick_chance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Music {
    sound: String,
    min_delay: i32,
    max_delay: i32,
    replace_current_music: i8,
}

