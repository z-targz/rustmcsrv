use super::{Identifier, Optional, Position, ToProtocol};

#[derive(Debug, Clone)]
pub struct DeathLocation {
    death_dimension: Identifier,
    death_location: Position,
}

impl ToProtocol for DeathLocation {
    #[inline]
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.death_dimension.to_protocol_bytes()//todo finish
    }
}

impl Optional for DeathLocation {}