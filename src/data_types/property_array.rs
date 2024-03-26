use serde::{Deserialize, Serialize};
use server_util::error::ProtocolError;

use super::{FromProtocol, ToProtocol, VarInt};

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    name: String,
    value: String,
    signature: Option<String>,
}

impl Property {
    pub fn new(name: String, value: String, signature: Option<String>) -> Self {
        Property {
            name : name,
            value : value,
            signature : signature, 
        }
    }
}

impl ToProtocol for Property {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.append(&mut self.name.to_protocol_bytes());
        out.append(&mut self.value.to_protocol_bytes());
        match &self.signature {
            Some(sig_ref) => {
                out.append(&mut true.to_protocol_bytes());
                out.append(&mut sig_ref.to_protocol_bytes());
            },
            None => out.append(&mut false.to_protocol_bytes()),
        }
        out
    }
}

//PropertyArray

pub type PropertyArray = Vec<Property>;

impl FromProtocol for PropertyArray {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let len = VarInt::from_protocol_iter(iter)?.get();
            let mut out: Vec<Property> = Vec::with_capacity(len as usize);
            for _ in 0..len {
                out.push(Property {
                    name : String::from_protocol_iter(iter)?,
                    value: String::from_protocol_iter(iter)?,
                    signature : if bool::from_protocol_iter(iter)? {
                        Some(String::from_protocol_iter(iter)?)
                    } else {
                        None
                    },
                })
            }
            Ok(out)
    }
}

impl ToProtocol for PropertyArray {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let len = self.len();
        out.append(&mut VarInt::new(len as i32).to_protocol_bytes());
        for i in 0..len {
            out.append(&mut self.get(i).unwrap().to_protocol_bytes());
        }
        out
    }
}