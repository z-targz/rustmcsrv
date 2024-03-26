use server_util::error::ProtocolError;

use super::{FromProtocol, ToProtocol, VarInt};

#[derive(Debug)]
///A byte array prefixed by its length as a VarInt
pub struct PrefixedByteArray {
    bytes: Vec<u8>,
}

impl PrefixedByteArray {
    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl FromProtocol for PrefixedByteArray {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let len = VarInt::from_protocol_iter(iter)?.get() as usize;
            let raw = iter.take(len).collect::<Vec<u8>>();
            if raw.len() < len {
                return Err(ProtocolError::IterEndError);
            }
            Ok(PrefixedByteArray{ bytes : raw })
    }
}

impl ToProtocol for PrefixedByteArray {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        VarInt::new(self.get_bytes().len() as i32).to_protocol_bytes().into_iter().chain(self.get_bytes().clone().into_iter()).collect()
    }
}