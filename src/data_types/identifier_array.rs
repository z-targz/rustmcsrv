use server_util::error::ProtocolError;

use super::{FromProtocol, Identifier, ToProtocol, VarInt};


pub struct IdentifierArray {
    data: Vec<Identifier>,
}

impl IdentifierArray {
    pub fn new(d: Vec<Identifier>) -> Self {
        IdentifierArray {
            data: d
        }
    }
}


impl ToProtocol for IdentifierArray {
    #[inline]
    fn to_protocol_bytes(&self) -> Vec<u8> {
        VarInt::new(self.data.len() as i32)
            .to_protocol_bytes()
            .into_iter()
            .chain(
                self.data.iter()
                    .map(|identifier| identifier.to_protocol_bytes())
                    .flatten()
            ).collect()
    }
}