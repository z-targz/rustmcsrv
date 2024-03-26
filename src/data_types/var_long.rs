use server_util::error::ProtocolError;

use super::{FromProtocol, ToProtocol};

#[derive(Debug, Copy, Clone)]
pub struct VarLong(i64);

impl VarLong {
    pub fn get(&self) -> i64 {
        self.0
    }
    pub fn new(l: i64) -> Self {
        VarLong(l)
    }
}

impl From<VarLong> for i64 {
    fn from(value: VarLong) -> Self {
        value.0
    }
}

impl std::fmt::Display for VarLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromProtocol for VarLong {
    /// reads a [VarLong](https://wiki.vg/Protocol#Type:VarLong) from a `u8` iterator, returning an `i64`.
    /// 
    /// the bytes will be consumed from the iterator.
    /// 
    /// see [https://wiki.vg/Protocol#VarInt_and_VarLong](https://wiki.vg/Protocol#VarInt_and_VarLong) for more details
    /// 
    /// # Arguments:
    /// * `iter:&mut impl Iterator<Item = u8>` - the iterator to read the bytes from
    ///
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let mut out: i64 = 0;
            for i in 0..9 {
                let Some(val) = iter.next() else { return Err(ProtocolError::IterEndError) };
                out += i64::from(val & 0x7f) << 7*i;
                if val & 0x80 == 0 {
                    return Ok(VarLong(out));
                }
            }
            let Some(val) = iter.next() else { return Err(ProtocolError::IterEndError) };
            if (val) & 0x80 != 0 {
                return Err(ProtocolError::VarLongError)
            }
            out += i64::from(val & 0x7f) << 7*9;
            Ok(VarLong(out))
    }
}

impl ToProtocol for VarLong {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut value:u64 = self.get().to_le() as u64;
    let mut out: Vec<u8> = Vec::with_capacity(9);
    
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