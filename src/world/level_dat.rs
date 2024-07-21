use std::collections::HashMap;

use quartz_nbt::io::NbtIoError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data_types::registry::byte_from_bool;

use super::{generator_settings::WorldGenSettings, world};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionInfo {
    #[serde(rename = "Id")]
    id: i32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Series")]
    series: String,

    #[serde(rename = "Snapshot")]
    #[serde(deserialize_with = "byte_from_bool")]
    snapshot: i8,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelDat {
    #[serde(rename = "allowCommands")]
    #[serde(deserialize_with = "byte_from_bool")]
    pub allow_commands: i8,

    #[serde(rename = "BorderCenterX")]
    pub border_center_x: f64,

    #[serde(rename = "BorderCenterZ")]
    pub border_center_z: f64,

    #[serde(rename = "BorderDamagePerBlock")]
    pub border_damage_per_block: f64,

    #[serde(rename = "BorderSize")]
    pub border_size: f64,

    #[serde(rename = "BorderSafeZone")]
    pub border_safe_zone: f64,

    #[serde(rename = "BorderSizeLerpTarget")]
    pub border_size_lerp_target: f64,

    #[serde(rename = "BorderSizeLerpTime")]
    pub border_size_lerp_time: i64,

    #[serde(rename = "BorderWarningBlocks")]
    pub border_warning_blocks: f64,

    #[serde(rename = "BorderWarningTime")]
    pub border_warning_time: f64,

    #[serde(rename = "clearWeatherTime")]
    pub clear_weather_time: i32,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,

    #[serde(rename = "DayTime")]
    pub day_time: i64,

    #[serde(rename = "Difficulty")]
    pub difficulty: i8,

    #[serde(rename = "DifficultyLocked")]
    pub difficulty_locked: i8,

    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_features: Option<Vec<String>>,

    #[serde(rename = "GameRules")]
    pub game_rules: HashMap<String, String>,

    #[serde(rename = "WorldGenSettings")]
    pub world_gen_settings: WorldGenSettings,

    #[serde(rename = "GameType")]
    pub game_type: i32,

    #[serde(deserialize_with = "byte_from_bool")]
    pub hardcore: i8,

    #[serde(deserialize_with = "byte_from_bool")]
    pub initialized: i8,

    #[serde(rename = "LastPlayed")]
    pub last_played: i64,

    #[serde(rename = "LevelName")]
    pub level_name: String,

    #[serde(rename = "MapFeatures")]
    #[serde(deserialize_with = "byte_from_bool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_features: Option<i8>,

    #[serde(deserialize_with = "byte_from_bool")]
    pub raining: i8,

    #[serde(rename = "rainTime")]
    pub rain_time: i32,

    #[serde(rename = "RandomSeed")]
    pub random_seed: i64,

    #[serde(rename = "SpawnX")]
    pub spawn_x: i32,

    #[serde(rename = "SpawnY")]
    pub spawn_y: i32,

    #[serde(rename = "SpawnZ")]
    pub spawn_z: i32,

    #[serde(deserialize_with = "byte_from_bool")]
    pub thundering: i8,

    #[serde(rename = "thunderTime")]
    pub thunder_time: i32,

    #[serde(rename = "Time")]
    pub time: i64,

    version: i32,

    #[serde(rename = "Version")]
    pub version_info: VersionInfo,

    #[serde(rename = "WanderingTraderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wandering_trader_id: Option<[i32; 4]>,

    #[serde(rename = "WanderingTraderSpawnChance")]
    pub wandering_trader_spawn_chance: i32,

    #[serde(rename = "WanderingTraderSpawnDelay")]
    pub wandering_trader_spawn_delay: i32,

    #[serde(rename = "WasModded")]
    #[serde(deserialize_with = "byte_from_bool")]
    was_moded: i8,

    #[serde(skip_serializing)]
    #[serde(rename = "DragonFight")]
    dragon_fight: Option<Value>,

    #[serde(skip_serializing)]
    #[serde(rename = "CustomBossEvents")]
    custom_boss_events: Option<Value>,  //We will use packets to manage the boss bars
}

impl LevelDat {
    pub fn new(
        allow_commands: i8,
        border_center_x: f64,
        border_center_z: f64,
        border_damage_per_block: f64,
        border_size: f64,
        border_safe_zone: f64,
        border_size_lerp_target: f64,
        border_size_lerp_time: i64,
        border_warning_blocks: f64,
        border_warning_time: f64,
        clear_weather_time: i32,
        data_version: i32,
        day_time: i64,
        difficulty: i8,
        difficulty_locked: i8,
        enabled_features: Option<Vec<String>>,
        game_rules: HashMap<String, String>,
        world_gen_settings: WorldGenSettings,
        game_type: i32,
        hardcore: i8,
        initialized: i8,
        last_played: i64,
        level_name: String,
        map_features: Option<i8>,
        raining: i8,
        rain_time: i32,
        random_seed: i64,
        spawn_x: i32,
        spawn_y: i32,
        spawn_z: i32,
        thundering: i8,
        thunder_time: i32,
        time: i64,
        version: i32,
        version_info: VersionInfo,
        wandering_trader_id: Option<[i32; 4]>,
        wandering_trader_spawn_chance: i32,
        wandering_trader_spawn_delay: i32,

    ) -> Self {
        Self {
            allow_commands,
            border_center_x,
            border_center_z,
            border_damage_per_block,
            border_size,
            border_safe_zone,
            border_size_lerp_target,
            border_size_lerp_time,
            border_warning_blocks,
            border_warning_time,
            clear_weather_time,
            data_version,
            day_time,
            difficulty,
            difficulty_locked,
            enabled_features,
            game_rules,
            world_gen_settings,
            game_type,
            hardcore,
            initialized,
            last_played,
            level_name,
            map_features,
            raining,
            rain_time,
            random_seed,
            spawn_x,
            spawn_y,
            spawn_z,
            thundering,
            thunder_time,
            time,
            version,
            version_info,
            wandering_trader_id,
            wandering_trader_spawn_chance,
            wandering_trader_spawn_delay,
            was_moded: 1,
            dragon_fight: None,
            custom_boss_events: None,
        }
    }

    pub fn to_nbt(&self) -> Result<Vec<u8>, NbtIoError> {
        #[derive(Serialize, Debug, Clone)]
        struct Data<'a> {
            #[serde(rename = "Data")]
            data: &'a LevelDat,
        }

        quartz_nbt::serde::serialize(&Data { data: &self }, None, quartz_nbt::io::Flavor::GzCompressed)
    }

    pub fn from_nbt(nbt: Vec<u8>) -> Result<(Self, String), NbtIoError> {
        quartz_nbt::serde::deserialize(nbt.as_slice(), quartz_nbt::io::Flavor::GzCompressed)
    }

}

impl Default for LevelDat {
    fn default() -> Self {
        Self {
            allow_commands: 1,
            border_center_x: 0.,
            border_center_z: 0.,
            border_damage_per_block: 0.2,
            border_size: 1024.,
            border_safe_zone: 5.,
            border_size_lerp_target: 1024.,
            border_size_lerp_time: 0,
            border_warning_blocks: 5.,
            border_warning_time: 15.,
            clear_weather_time: -1,
            data_version: -1,
            day_time: world::TIME_NOON,
            difficulty: Default::default(),
            difficulty_locked: Default::default(),
            game_rules: Default::default(),
            world_gen_settings: Default::default(),
            game_type: 1,
            version: 19133,
            enabled_features: None,
            hardcore: 0,
            initialized: 0,
            last_played: 0,
            level_name: "world".to_owned(),
            map_features: Some(0),
            raining: 0,
            rain_time: -1,
            random_seed: 0,
            spawn_x: 0,
            spawn_y: -60,
            spawn_z: 0,
            thundering: 0,
            thunder_time: -1,
            time: 0,
            version_info: VersionInfo {
                id: 3953,
                name: "1.21".to_owned(),
                series: "main".to_owned(),
                snapshot: 0,
            },
            wandering_trader_id: None,
            wandering_trader_spawn_chance: 0,
            wandering_trader_spawn_delay: -1,
            was_moded: 1,
            dragon_fight: None,
            custom_boss_events: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write, path::Path};

    use super::LevelDat;

    #[test]
    fn test_create_level_dat() {
        let level_data = LevelDat::default();
        let bytes = level_data.to_nbt().unwrap();
        let mut level_dat_file = std::fs::OpenOptions::new().read(true).write(true).create(true).open("level.dat").unwrap();
        level_dat_file.write_all(bytes.as_slice()).unwrap();
        println!("{}", Path::new("level.dat").display());
    }
}