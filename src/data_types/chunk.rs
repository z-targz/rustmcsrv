use std::marker::PhantomData;

use crate::data_types::{ToProtocol, VarInt, VarUShort};

/// Represents a `Chunk`, or a ChunkColumn
/// 
pub struct ProtocolChunk {
    chunk_x: i32,
    chunk_z: i32,
    //TODO: Replace this with a custom data type to avoid wasting 8 bytes when all chunks in a ChunkCache will have the same length
    data: Vec<ProtocolChunkSection>,
    motion_blocking: [u16; 256],
    world_surface: [u16; 256],
}

/// Represents a 16^3 vertical section of a `Chunk`\
/// see [Chunk Section Structure](https://wiki.vg/Chunk_Format#Chunk_Section_structure)
/// ## Arguments:
/// * `block_count: i16` - Number of non-air blocks present in the chunk section.
///     "Non-air" is defined as any fluid and block other than air, cave air, and void air.
///     The client will keep count of the blocks as they are broken and placed, and, if the block count reaches 0, the whole chunk section is not rendered, even if it still has blocks. 
/// * `block_states: PalettedContainer` - Consists of 4096 entries, representing all the blocks in the chunk section. 
/// * `biomes: PalettedContainer` - Consists of 64 entries, representing 4×4×4 biome regions in the chunk section. 
pub struct ProtocolChunkSection {
    block_count: i16,
    block_states: PalettedContainer<BlockPalette>,
    biomes: PalettedContainer<BiomePalette>,
}

/// A Paletted Container is a palette-based storage of entries.\
/// Paletted Containers have an associated registry (either block states or biomes as of now), where values are mapped from.
pub struct PalettedContainer<T> 
    where T: PaletteType
{
    palette: Palette<T>,
    data_array: DataArray,
}

trait PaletteType {}

pub struct BlockPalette;
impl PaletteType for BlockPalette {}

pub struct BiomePalette;
impl PaletteType for BiomePalette {}


#[allow(private_bounds)]
pub enum Palette<T> 
    where T: PaletteType
{
    SingleValued {
        value: VarUShort,
    },
    Indirect {
        palette: VarUShortArray
    },
    Direct(PhantomData<T>),
}

impl ToProtocol for Palette<BiomePalette> {
    #[inline]
    fn to_protocol_bytes(&self) -> Vec<u8> {
        match self {
            Self::SingleValued { value: val } => {
                single_valued(val)
            },
            Self::Indirect { palette: pal } => {
                indirect(pal)
            },
            //TODO: Calculate this from the size of the block registry
            Self::Direct(PhantomData) => vec![6]
        }
    }
}

impl ToProtocol for Palette<BlockPalette> {
    #[inline]
    fn to_protocol_bytes(&self) -> Vec<u8> {
        match self {
            Self::SingleValued { value: val } => {
                single_valued(val)
            },
            Self::Indirect { palette: pal } => {
                indirect(pal)
            },
            //TODO: Calculate this from the size of the biome registry
            Self::Direct(PhantomData) => vec![15]
        }
    }
}

#[inline]
fn single_valued(val: &VarUShort) -> Vec<u8> {
    vec![0u8]
        .into_iter()
        .chain(
            val.to_protocol_bytes()
                .into_iter()
        ).collect()
}

#[inline]
fn indirect(pal: &VarUShortArray) -> Vec<u8> {
    vec![(pal.data.len() as u32).ilog2() as u8]
    .into_iter()
    .chain(
        pal.to_protocol_bytes()
            .into_iter()
    ).collect()
}

pub struct VarUShortArray {
    data: Vec<VarUShort>,
}

impl ToProtocol for VarUShortArray {
    #[inline]
    fn to_protocol_bytes(&self) -> Vec<u8> {
        VarInt::new(self.data.len() as i32)
            .to_protocol_bytes()
            .into_iter()
            .chain(
                self.data.iter()
                    .flat_map(|var_ushort| var_ushort.to_protocol_bytes())
            ).collect()
    }
}

pub struct DataArray {
    data: Vec<u64>,
}

impl ToProtocol for DataArray {
    #[inline]
    fn to_protocol_bytes(&self) -> Vec<u8> {
        VarInt::new(self.data.len() as i32)
            .to_protocol_bytes()
            .into_iter()
            .chain(
                self.data.iter()
                    .flat_map(|ulong| ulong.to_be_bytes().to_vec())
            ).collect()
    }
}