use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LevelDat {
    #[serde(rename = "BorderCenterX")]
    border_center_x: f64,

    #[serde(rename = "BorderCenterZ")]
    border_center_z: f64,

    #[serde(rename = "BorderDamagePerBlock")]
    border_damage_per_block: f64,

    #[serde(rename = "BorderSize")]
    border_size: f64,

    #[serde(rename = "BorderSafeZone")]
    border_safe_zone: f64,

    #[serde(rename = "BorderSizeLerpTarget")]
    border_size_lerp_target: f64,

    #[serde(rename = "BorderSizeLerpTime")]
    border_size_lerp_time: i64,

    #[serde(rename = "BorderWarningBlocks")]
    border_warning_blocks: f64,

    #[serde(rename = "BorderWarningTime")]
    border_warning_time: f64,

    #[serde(rename = "clearWeatherTime")]
    clear_weather_time: i32,

    //...
}

