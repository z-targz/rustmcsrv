use serde::{Deserialize, Serialize};

use super::{Identifier, Optional, BlockPos, ToProtocol};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeathLocation {
    dimension: Identifier,
    pos: BlockPos,
}

impl ToProtocol for DeathLocation {
    #[inline]
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.dimension.to_protocol_bytes()//todo finish
    }
}

impl Optional for DeathLocation {}