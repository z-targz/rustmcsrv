use server_util::error::ProtocolError;
use uuid::Uuid;

use super::{Optional, VarInt};

pub trait ToProtocol {
    fn to_protocol_bytes(&self) -> Vec<u8>;
}

pub trait FromProtocol {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized;
}

//String

impl FromProtocol for String {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let len = VarInt::from_protocol_iter(iter)?.get() as usize;
            let raw = iter.take(len).collect::<Vec<u8>>();
            if raw.len() < len {
                Err(ProtocolError::IterEndError)?
            }
            Ok(String::from_utf8(raw)?)
    }
}

impl ToProtocol for String {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let raw = self.as_bytes().to_owned().into_iter();
        let len = VarInt::new(raw.len() as i32).to_protocol_bytes().into_iter();
        len.chain(raw).collect()
    }
}

//Float (f32)

impl FromProtocol for f32 {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let bytes = iter.take(4).collect::<Vec<u8>>();
            if bytes.len() < 4 {
                return Err(ProtocolError::IterEndError);
            }
            Ok(f32::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl ToProtocol for f32 {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

//Double (f64)

impl FromProtocol for f64 {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let bytes = iter.take(8).collect::<Vec<u8>>();
            if bytes.len() < 8 {
                return Err(ProtocolError::IterEndError);
            }
            Ok(f64::from_be_bytes(bytes.try_into().unwrap()))
    }
}

impl ToProtocol for f64 {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

//Boolean (bool)

impl FromProtocol for bool {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let Some(value) = iter.next() else { return Err(ProtocolError::IterEndError) };
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(ProtocolError::NotBoolean),
            }
    }
}

impl ToProtocol for bool {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        match self {
            true => vec![1u8],
            false => vec![0u8],
        }
    }
}

//Unsigned Byte (u8)

impl FromProtocol for u8 {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            match iter.next() {
                Some(ubyte) => Ok(ubyte),
                None => Err(ProtocolError::IterEndError),
            }
    }
}

impl ToProtocol for u8 {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

//Signed Byte (i8)

impl FromProtocol for i8 {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
        match iter.next() {
            Some(ubyte) => Ok(ubyte.to_be_bytes()[0] as i8),
            None => Err(ProtocolError::IterEndError),
        }
    }
}

impl ToProtocol for i8 {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

//Unsigned Short (u16)

impl FromProtocol for u16 {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let array: [u8; 2] = std::convert::TryFrom::try_from(iter.take(2).collect::<Vec<u8>>().as_slice())?;
            Ok(u16::from_be_bytes(array))
    }
}

impl ToProtocol for u16 {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

//Signed Short (i16)

impl FromProtocol for i16 {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let array: [u8; 2] = std::convert::TryFrom::try_from(iter.take(2).collect::<Vec<u8>>().as_slice())?;
            Ok(i16::from_be_bytes(array))
    }
}

impl ToProtocol for i16 {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

//Int (i32)

impl ToProtocol for i32 {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

//Long (i64)

impl FromProtocol for i64 {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let array: [u8; 8] = std::convert::TryFrom::try_from(iter.take(8).collect::<Vec<u8>>().as_slice())?;
            Ok(i64::from_be_bytes(array))
    }
}

impl ToProtocol for i64 {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

//Uuid

impl FromProtocol for Uuid {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let array: [u8; 16] = std::convert::TryFrom::try_from(iter.take(16).collect::<Vec<u8>>().as_slice())?;
            Ok(Uuid::from_u128(u128::from_be_bytes(array)))
    }
}

impl ToProtocol for Uuid {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        //really counterintuitive, but to_u128_le gives an Little Endian representation of a UUID in Big Endian,
        //so we want to retain this byte order by using to_le_bytes(), which contains a Little Endian representation
        //of the data which has been flipped to Big Endian
        self.to_u128_le().to_le_bytes().to_vec()
    }
}

impl Optional for Uuid {}