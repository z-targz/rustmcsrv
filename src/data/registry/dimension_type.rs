use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DimensionType {
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
    monster_spawn_light_level: LightLevel
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LightLevel {
    fixed(i32),
    distribution(IntegerDistribution)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegerDistribution {
    r#type: String,
    value: Uniform, //Only uniform is ever used, this is a potential BUG in future versions
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DistributionInfo {
    constant(i32),
    uniform(Uniform),

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uniform {
    min_inclusive: i32,
    max_inclusive: i32,
}