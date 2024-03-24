use std::error::Error;

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