use server_util::error::ProtocolError;

use super::{FromProtocol, Optional, ToProtocol};

#[derive(Debug, Copy, Clone)]
pub struct VarInt(i32);

impl VarInt {
    pub fn get(&self) -> i32 {
        self.0
    }
    pub fn new(i: i32) -> Self {
        VarInt(i)
    }
}

impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        value.0
    }
}

impl FromProtocol for VarInt {
    /// Reads a [VarInt](https://wiki.vg/Protocol#Type:VarInt) from a `u8` iterator, returning an `i32`.
    /// 
    /// The bytes will be consumed from the iterator.
    /// 
    /// See [https://wiki.vg/Protocol#VarInt_and_VarLong](https://wiki.vg/Protocol#VarInt_and_VarLong) for more details
    /// 
    /// # Arguments:
    /// * `iter:&mut impl Iterator<Item = u8>` - the iterator to read the bytes from
    /// 
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized
    {
        let mut out: i32 = 0;
        for i in 0..4 {
            let Some(val) = iter.next() else { return Err(ProtocolError::IterEndError) };
            out += i32::from(val & 0x7f) << 7*i;
            if val & 0x80 == 0 {
                return Ok(VarInt(out));
            }
        }
        let Some(val) = iter.next() else { return Err(ProtocolError::IterEndError) };
        if (val) & 0x80 != 0 {
            return Err(ProtocolError::VarIntError)
        }
        out += i32::from(val & 0x7f) << 7*4;
        Ok(VarInt(out))
    }
}

impl ToProtocol for VarInt {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut value: u32 = self.get().to_le() as u32;
        let mut out: Vec<u8> = Vec::with_capacity(5);
        loop {
            if value & !0x7f == 0 {
                out.push(value.to_le_bytes()[0]);
                break;
            }
            out.push(value.to_le_bytes()[0] | 0x80);
            value >>= 7;
        }
        out.shrink_to_fit();
        out
    }
}

impl Optional for VarInt {}