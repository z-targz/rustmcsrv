use std::error::Error;

use serde::{Serialize, Deserialize};
use server_util::error::ProtocolError;

use uuid::Uuid;

pub type JSON = String;

pub type NBT = Vec<u8>;

pub mod registry;

pub mod angle;
use angle::Angle;

pub trait ToProtocol {
    fn to_protocol_bytes(&self) -> Vec<u8>;
}

pub trait FromProtocol {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized;
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct CJSONTextComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    translatable: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    keybind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    underlined: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    strikethrough: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    obfuscated: Option<bool>,
}


impl CJSONTextComponent {
    pub fn new(
            text: Option<String>, 
            translatable: Option<String>, 
            keybind: Option<String>, 
            color: Option<String>, 
            bold: Option<bool>,
            italic: Option<bool>,
            underlined: Option<bool>,
            strikethrough: Option<bool>,
            obfuscated: Option<bool>,
        ) -> Self {
        CJSONTextComponent {
            text : text,
            translatable : translatable,
            keybind : keybind,
            color : color,
            bold : bold,
            italic : italic,
            underlined : underlined,
            strikethrough : strikethrough,
            obfuscated : obfuscated,
        }
    }

    pub fn from_str(the_str: &str) -> Self {
        CJSONTextComponent::new(
            Some(the_str.to_string()), 
            None, 
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn from_string(the_string: &String) -> Self {
        CJSONTextComponent::new(
            Some(the_string.clone()), 
            None, 
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn color(mut self, hex_code: u8) -> Self {
        assert!(hex_code <= 0xf);
        let color = match hex_code {
            0x0 => "black",
            0x1 => "dark_blue",
            0x2 => "dark_green",
            0x3 => "dark_aqua",
            0x4 => "dark_red",
            0x5 => "dark_purple",
            0x6 => "gold",
            0x7 => "gray",
            0x8 => "dark_gray",
            0x9 => "blue",
            0xa => "green",
            0xb => "aqua",
            0xc => "red",
            0xd => "light_purple",
            0xe => "yellow",
            0xf => "white",
            _ => panic!("cosmic rays"),
        };
        self.color = Some(color.to_string());
        self
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ToProtocol for CJSONTextComponent {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        let mut the_string = serde_json::to_string(self).unwrap();
        //the_string.pop();
        //the_string.remove(0);
        the_string.to_protocol_bytes()
    }
}

pub type PropertyArray = Vec<Property>;

//VarInt

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
        where Self: Sized {
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

//VarLong

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

//PrefixedByteArray

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

//PropertyArray

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

//NBT

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

//InferredByteArray

impl FromProtocol for InferredByteArray {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            Ok(InferredByteArray{bytes : iter.collect()})
    }
}

impl ToProtocol for InferredByteArray {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        self.get_bytes().clone()
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

//Angle

impl FromProtocol for Angle {
    fn from_protocol_iter(iter: &mut impl Iterator<Item = u8>) -> Result<Self, ProtocolError> 
        where Self: Sized {
            match iter.next() {
                Some(byte) => {
                    Ok(Angle::new(byte as f64 * angle::CONVERSION_FACTOR_FROM_NETWORK))
                },
                None => Err(ProtocolError::IterEndError)?
            }
    }
}

impl ToProtocol for Angle {
    fn to_protocol_bytes(&self) -> Vec<u8> {
        vec![(((self.get_degrees() * angle::CONVERSION_FACTOR_TO_NETWORK) as i32 % 0xff) as u8)]
    }
}





pub trait Optional {}

impl Optional for VarInt {}

impl Optional for InferredByteArray {}

impl Optional for Uuid {}

pub fn read_option<T>(iter: &mut impl Iterator<Item = u8>) -> Result<Option<T>, Box<dyn Error + Send + Sync>> 
where
    T: Optional + FromProtocol
{
    let Some(is_some) = iter.next() else { return Err(ProtocolError::IterEndError)? };
    let is_some = is_some != 0;
    if is_some {
        Ok(Some(T::from_protocol_iter(iter)?))
    } else {
        Ok(None)
    }
}

pub fn create_option<T>(option: Option<T>) -> Vec<u8>
where
    T: Optional + ToProtocol
{
    if option.is_some() {
        let mut out = vec![1u8];
        out.append(&mut option.unwrap().to_protocol_bytes());
        out
    } else {
        vec![0u8]
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_read_var_int() {
        let varint_0: Vec<u8> = vec![0x00];
        let mut iter0 = varint_0.into_iter();
        assert_eq!(0, read_var_int(&mut iter0).unwrap());

        let varint_1: Vec<u8> = vec![0x01];
        let mut iter1 = varint_1.into_iter();
        assert_eq!(1, read_var_int(&mut iter1).unwrap());

        let varint_2: Vec<u8> = vec![0x02];
        let mut iter2 = varint_2.into_iter();
        assert_eq!(2, read_var_int(&mut iter2).unwrap());

        let varint_127: Vec<u8> = vec![0x7f];
        let mut iter127 = varint_127.into_iter();
        assert_eq!(127, read_var_int(&mut iter127).unwrap());

        let varint_128: Vec<u8> = vec![0x80, 0x01];
        let mut iter128 = varint_128.into_iter();
        assert_eq!(128, read_var_int(&mut iter128).unwrap());

        let varint_255: Vec<u8> = vec![0xff, 0x01];
        let mut iter255 = varint_255.into_iter();
        assert_eq!(255, read_var_int(&mut iter255).unwrap());

        let varint_25565: Vec<u8> = vec![0xdd, 0xc7, 0x01];
        let mut iter25565 = varint_25565.into_iter();
        assert_eq!(25565, read_var_int(&mut iter25565).unwrap());

        let varint_2097151: Vec<u8> = vec![0xff, 0xff, 0x7f];
        let mut iter2097151 = varint_2097151.into_iter();
        assert_eq!(2097151, read_var_int(&mut iter2097151).unwrap());

        let varint_2147483647: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0x07];
        let mut iter2147483647 = varint_2147483647.into_iter();
        assert_eq!(2147483647, read_var_int(&mut iter2147483647).unwrap());

        let varint_minus1: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0x0f];
        let mut iter_minus1 = varint_minus1.into_iter();
        assert_eq!(-1, read_var_int(&mut iter_minus1).unwrap());

        let varint_minus2147483648: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x08];
        let mut iter_minus2147483648 = varint_minus2147483648.into_iter();
        assert_eq!(-2147483648, read_var_int(&mut iter_minus2147483648).unwrap());

        let varint_toolarge: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        let mut iter_toolarge = varint_toolarge.into_iter();
        assert!(read_var_int(&mut iter_toolarge).is_err());

        let varint_toolarge2: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        let mut iter_toolarge2 = varint_toolarge2.into_iter();
        assert!(read_var_int(&mut iter_toolarge2).is_err());

        let varint_malformed: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80];
        let mut iter_malformed = varint_malformed.into_iter();
        assert!(read_var_int(&mut iter_malformed).is_err());

        let varint_test_iter: Vec<u8> = vec![0xdd, 0xc7, 0x01, 0x80, 0x7f];
        let mut iter_test_iter = varint_test_iter.into_iter();
        assert_eq!(25565, read_var_int(&mut iter_test_iter).unwrap());
        assert_eq!(0x80, iter_test_iter.next().unwrap());
        assert_eq!(0x7f, iter_test_iter.next().unwrap());

    }
    #[test]
    fn test_read_var_long() {
        let varlong_0: Vec<u8> = vec![0x00];
        let mut iter0 = varlong_0.into_iter();
        assert_eq!(0, read_var_long(&mut iter0).unwrap());

        let varlong_1: Vec<u8> = vec![0x01];
        let mut iter1 = varlong_1.into_iter();
        assert_eq!(1, read_var_long(&mut iter1).unwrap());

        let varlong_2: Vec<u8> = vec![0x02];
        let mut iter2 = varlong_2.into_iter();
        assert_eq!(2, read_var_long(&mut iter2).unwrap());

        let varlong_127: Vec<u8> = vec![0x7f];
        let mut iter127 = varlong_127.into_iter();
        assert_eq!(127, read_var_long(&mut iter127).unwrap());

        let varlong_128: Vec<u8> = vec![0x80, 0x01];
        let mut iter128 = varlong_128.into_iter();
        assert_eq!(128, read_var_long(&mut iter128).unwrap());

        let varlong_255: Vec<u8> = vec![0xff, 0x01];
        let mut iter255 = varlong_255.into_iter();
        assert_eq!(255, read_var_long(&mut iter255).unwrap());

        let varlong_2147483647: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0x07];
        let mut iter2147483647 = varlong_2147483647.into_iter();
        assert_eq!(2147483647, read_var_long(&mut iter2147483647).unwrap());

        let varlong_9223372036854775807: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        let mut iter9223372036854775807 = varlong_9223372036854775807.into_iter();
        assert_eq!(9223372036854775807, read_var_long(&mut iter9223372036854775807).unwrap());

        let varlong_minus1: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
        let mut iter_minus1 = varlong_minus1.into_iter();
        assert_eq!(-1, read_var_long(&mut iter_minus1).unwrap());

        let varlong_minus2147483648: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01];
        let mut iter_minus2147483648 = varlong_minus2147483648.into_iter();
        assert_eq!(-2147483648, read_var_long(&mut iter_minus2147483648).unwrap());

        let varlong_minus9223372036854775808: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];
        let mut iter_minus9223372036854775808 = varlong_minus9223372036854775808.into_iter();
        assert_eq!(-9223372036854775808, read_var_long(&mut iter_minus9223372036854775808).unwrap());

        let varlong_toolarge: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        let mut iter_toolarge = varlong_toolarge.into_iter();
        assert!(read_var_long(&mut iter_toolarge).is_err());

        let varlong_toolarge2: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        let mut iter_toolarge2 = varlong_toolarge2.into_iter();
        assert!(read_var_long(&mut iter_toolarge2).is_err());

        let varlong_malformed: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80];
        let mut iter_malformed = varlong_malformed.into_iter();
        assert!(read_var_long(&mut iter_malformed).is_err());

        let varlong_test_iter: Vec<u8> = vec![0xdd, 0xc7, 0x01, 0x80, 0x7f];
        let mut iter_test_iter = varlong_test_iter.into_iter();
        assert_eq!(25565, read_var_long(&mut iter_test_iter).unwrap());
        assert_eq!(0x80, iter_test_iter.next().unwrap());
        assert_eq!(0x7f, iter_test_iter.next().unwrap());

    }

    #[test]
    fn test_create_var_int() {
        let varint_0: Vec<u8> = vec![0x00];
        assert_eq!(varint_0, create_var_int(0));
    
        let varint_1: Vec<u8> = vec![0x01];
        assert_eq!(varint_1, create_var_int(1));

        let varint_2: Vec<u8> = vec![0x02];
        assert_eq!(varint_2, create_var_int(2));

        let varint_127: Vec<u8> = vec![0x7f];
        assert_eq!(varint_127, create_var_int(127));

        let varint_128: Vec<u8> = vec![0x80, 0x01];
        assert_eq!(varint_128, create_var_int(128));

        let varint_255: Vec<u8> = vec![0xff, 0x01];
        assert_eq!(varint_255, create_var_int(255));

        let varint_25565: Vec<u8> = vec![0xdd, 0xc7, 0x01];
        assert_eq!(varint_25565, create_var_int(25565));

        let varint_2097151: Vec<u8> = vec![0xff, 0xff, 0x7f];
        assert_eq!(varint_2097151, create_var_int(2097151));

        let varint_2147483647: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0x07];
        assert_eq!(varint_2147483647, create_var_int(2147483647));

        let varint_minus1: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0x0f];
        assert_eq!(varint_minus1, create_var_int(-1));

        let varint_minus2147483648: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x08];
        assert_eq!(varint_minus2147483648, create_var_int(i32::min_value()));
        
    }

    #[test]
    fn test_create_var_long() {
        let varlong_0: Vec<u8> = vec![0x00];
        assert_eq!(varlong_0, create_var_long(0));

        let varlong_1: Vec<u8> = vec![0x01];
        assert_eq!(varlong_1, create_var_long(1));

        let varlong_2: Vec<u8> = vec![0x02];
        assert_eq!(varlong_2, create_var_long(2));

        let varlong_127: Vec<u8> = vec![0x7f];
        assert_eq!(varlong_127, create_var_long(127));

        let varlong_128: Vec<u8> = vec![0x80, 0x01];
        assert_eq!(varlong_128, create_var_long(128));

        let varlong_255: Vec<u8> = vec![0xff, 0x01];
        assert_eq!(varlong_255, create_var_long(255));

        let varlong_2147483647: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0x07];
        assert_eq!(varlong_2147483647, create_var_long(2147483647));

        let varlong_9223372036854775807: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        assert_eq!(varlong_9223372036854775807, create_var_long(9223372036854775807));

        let varlong_minus1: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
        assert_eq!(varlong_minus1, create_var_long(-1));

        let varlong_minus2147483648: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01];
        assert_eq!(varlong_minus2147483648, create_var_long(-2147483648));

        let varlong_minus9223372036854775808: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];
        assert_eq!(varlong_minus9223372036854775808, create_var_long(-9223372036854775808));
    }

    #[test]
    fn test_strings() {
        //test that encoding and decoding strings works
        let string1 = "Hello, World!".to_string();
        let vector = create_string(&string1);
        let string2 = read_string(&mut vector.into_iter()).unwrap();
        assert_eq!(string1, string2);

        //test that connection closing can be handled
        let string1 = "Hello".to_string();
        let mut bytes = create_var_int(6).into_iter().chain(string1.bytes());
        let string2 = read_string(&mut bytes);
        assert!(string2.is_err());
        assert_eq!(string2.err().unwrap().to_string(), ProtocolError::IterEndError.to_string());

        //test that data following the string will not be read
        let string1 = "Hello, ".to_string();
        let string2 = "World!".to_string();
        let bytes1 = create_string(&string1);
        let bytes2 = create_string(&string2);

        let mut bytes = bytes1.into_iter().chain(bytes2.into_iter());
        assert_eq!(read_string(&mut bytes).unwrap(), string1);
    }
    #[test]
    fn test_uuid() {
        let uuid: Uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let uuid_vector = create_uuid(uuid);
        let mut uuid_iter = uuid_vector.into_iter();
        let uuid2 = read_uuid(&mut uuid_iter).unwrap();
        println!("uuid 1: {}", uuid.to_string());
        println!("uuid 2: {}", uuid2.to_string());
        assert_eq!(uuid, uuid2);
    }

    #[bench]
    // This is the fastest implementation so far
    fn create_nbt_a(bencher: &mut Bencher) {
        let nbt = registry::get_registry_nbt().unwrap();
        fn create_nbt_temp(nbt: &NBT) -> Vec<u8> {
            let mut out = vec![10u8];
            out.extend(nbt.as_slice()[3..].iter());
            out
        }
        bencher.iter(|| {
            let _ = create_nbt_temp(&nbt.clone());
        });
    }
    #[bench]
    fn read_nbt_a(bencher: &mut Bencher) {
        let nbt = registry::get_registry_nbt().unwrap();
        fn create_nbt_temp(nbt: NBT) -> Vec<u8> {
            let mut out = vec![10u8];
            out.extend(nbt.as_slice()[3..].into_iter());
            out
        }
        let nbt_temp = create_nbt_temp(nbt);
        let nbt_iter = nbt_temp.into_iter();
        fn read_nbt_temp(iter: &mut impl Iterator<Item = u8>) -> Result<NBT, ()> {
            iter.next(); // Skip first element (0x0a)
            let mut out = vec![10u8, 0u8, 0u8];
            out.extend(iter);
            Ok(out)
        }
        bencher.iter(|| {
            let x = read_nbt_temp(&mut nbt_iter.clone()).unwrap();
        });
    }
    #[bench]
    fn read_nbt_b(bencher: &mut Bencher) {
        let nbt = registry::get_registry_nbt().unwrap();
        fn create_nbt_temp(nbt: NBT) -> Vec<u8> {
            let mut out = vec![10u8];
            out.extend(nbt.as_slice()[3..].into_iter());
            out
        }
        let nbt_temp = create_nbt_temp(nbt);
        let nbt_iter = nbt_temp.into_iter();
        fn read_nbt_temp(iter: &mut impl Iterator<Item = u8>) -> Result<NBT, ()> {
            iter.next(); // Skip first element (0x0a)
            Ok(vec![10u8, 0u8, 0u8].into_iter().chain(iter).collect())
        }
        bencher.iter(|| {
            let x = read_nbt_temp(&mut nbt_iter.clone()).unwrap();
        });
    }
}*/