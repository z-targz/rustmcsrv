use std::collections::HashMap;

use serde::{Deserialize, Serialize};



pub struct RegionFile {
    header: RegionFileHeader,
    chunks: Vec<ChunkFormat>,
}

pub struct RegionFileHeader {
    locations: [LocationEntry;1024],
    timestamps: [u32;1024],
}

pub struct LocationEntry {
    offset: Offset,
    sector_count: u8,
}

pub struct Offset {
    data: [u8;3]
}

pub fn chunk_index(x: i32, z: i32) -> usize {
    (4 * x + 32 * z) as usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkFormat {
    #[serde(rename = "DataVersion")]
    data_version: i32,

    #[serde(rename = "xPos")]
    x_pos: i32,

    #[serde(rename = "zPos")]
    z_pos: i32,

    #[serde(rename = "yPos")]
    y_pos: i32,

    #[serde(rename = "Status")]
    status: String,

    #[serde(rename = "LastUpdate")]
    last_update: i64,

    sections: Vec<Section>,

    block_entities: Vec<BlockEntity>,

    #[serde(skip_serializing)]
    #[serde(rename = "CarvingMasks")]
    carving_masks: CarvingMasks,


}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    #[serde(rename = "Y")]
    y: i8,

    block_states: BlockStates,

    biomes: Biomes,

    #[serde(rename = "BlockLight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    
    block_light: Option<Vec<u8>>, // 2048 bytes

    #[serde(rename = "SkyLight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sky_light: Option<Vec<u8>>, // 2048 bytes
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockStates {
    palette: Vec<Block>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Properties")]
    properties: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Biomes {
    palette: Vec<String>,
    
    data: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockEntity {

}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CarvingMasks {
    air: Vec<u8>,
    liquid: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct HeightMaps {
    motion_blocking: Vec<i64>,
    motion_blocking_no_leaves: Vec<i64>,
    ocean_floor: Vec<i64>,
    ocean_floor_wg: Vec<i64>,
    world_surface: Vec<i64>,
    world_surface_wg: Vec<i64>,
}