use std::error::Error;

use server_util::error::ProtocolError;

use super::{FromProtocol, ToProtocol};

#[derive(Debug)]
pub enum InvalidPositionError {
    XTooBig,
    XTooSmall,
    YTooBig,
    YTooSmall,
    ZTooBig,
    ZTooSmall,
}

impl Error for InvalidPositionError {}

impl std::fmt::Display for InvalidPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub z: i32,
    pub y: i16,
}

const MIN_26BIT: i32 = -2^25 - 1; //One less because exclusive ranges are experimental in match statements
const MAX_26BIT: i32 = 2^25; //One more because exclusive ranges are experimental in match statements

const MIN_12BIT: i16 = -2^11 - 1; //One less because...
const MAX_12BIT: i16 = 2^11; //One more...

impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Result<Self, InvalidPositionError> {
        match x {
            i32::MIN..=MIN_26BIT => return Err(InvalidPositionError::XTooSmall),
            MAX_26BIT..=i32::MAX => return Err(InvalidPositionError::XTooBig),
            _ => (),
        }
        match z {
            i32::MIN..=MIN_26BIT => return Err(InvalidPositionError::ZTooSmall),
            MAX_26BIT..=i32::MAX => return Err(InvalidPositionError::ZTooBig),
            _ => (),
        }
        match y {
            i16::MIN..=MIN_12BIT => return Err(InvalidPositionError::YTooSmall),
            MAX_12BIT..=i16::MAX => return Err(InvalidPositionError::YTooBig),
            _=> (),
        }
        Ok(Position {
            x : x,
            y : y,
            z : z
        })
    }
}

impl FromProtocol for Position {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            let bytes = iter.take(8).collect::<Vec<u8>>();
            if bytes.len() < 8 {
                return Err(ProtocolError::IterEndError);
            }
            let val = u64::from_be_bytes(bytes.try_into().unwrap());
            let fff: u64 = 0xfff;
            let x = val >> 38;
            let y = val & fff;
            let z = (val & 0x3FFFFFF) << 12;
            Ok(Position::new(x as i32, y as i16, z as i32).unwrap())
    }
}

impl ToProtocol for Position {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        (((self.x as i64 & 0x3FFFFFF) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | (self.y as i64 & 0xFFF)).to_be_bytes().to_vec()
    }
}