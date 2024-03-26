use server_util::error::ProtocolError;

use super::{FromProtocol, ToProtocol};

//NBT
pub type NBT = Vec<u8>;

impl FromProtocol for NBT {
    /// Reads NBT data from the iterator
    /// This is wrapped in an empty Result for compatibility with other functions and macros.
    /// 
    /// This is lazily evaluated, so an invalid NBT error needs to be handled 
    /// when the NBT is actually serialized into the appropriate data structure.
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            iter.next(); // Skip first element (0x0a)
            Ok(vec![10u8, 0u8, 0u8].into_iter().chain(iter).collect())
    }
}

impl ToProtocol for NBT {
    /// This function strips the root tag from the provided NBT so it can be sent
    /// over the network in versions 1.20.2+
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out = vec![10u8];
        out.extend(self.as_slice()[3..].iter());
        out
    }
}