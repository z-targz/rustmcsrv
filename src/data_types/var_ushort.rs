use super::ToProtocol;

#[derive(Debug, Copy, Clone)]
pub struct VarUShort(u16);

impl VarUShort {
    pub fn get(&self) -> u16 {
        self.0
    }
    pub fn new(us: u16) -> Self {
        VarUShort(us)
    }
}

impl std::fmt::Display for VarUShort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<VarUShort> for u16 {
    fn from(value: VarUShort) -> Self {
        value.0
    }
}

impl ToProtocol for VarUShort {
    #[inline]
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut value = self.get();
        let mut out: Vec<u8> = Vec::with_capacity(3);
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