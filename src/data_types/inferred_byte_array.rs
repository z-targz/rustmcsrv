use server_util::error::ProtocolError;

use super::{FromProtocol, Optional, ToProtocol};

#[derive(Debug)]
///A byte array inferred from packet length. This is always at the end of the packet, so we just collect the iterator and return it
pub struct InferredByteArray {
    bytes: Vec<u8>,
}

impl InferredByteArray {
    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl FromProtocol for InferredByteArray {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            Ok(InferredByteArray{bytes : iter.collect()})
    }
}

impl ToProtocol for InferredByteArray {
    #[inline]
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.get_bytes().clone()
    }
}

impl Optional for InferredByteArray {}