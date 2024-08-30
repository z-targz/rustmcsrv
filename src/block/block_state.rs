use serde::{Deserialize, Serialize};

use crate::data_types::Identifier;


pub struct BlockState {
    name: Identifier,
    value: BlockStateValue,
}

pub enum BlockStateValue {
    Byte(u8),
    Facing(Facing),
}

pub enum Facing {
    North,
    South,
    East,
    West,
    Up,
    Down,
}