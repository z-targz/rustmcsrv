use std::error::Error;
use std::pin::Pin;

use server_util::error::IterEndError;

use futures::stream::Stream;
use futures::stream::StreamExt;

use uuid::Uuid;

pub type VarInt = i32;

pub type JSON = String;

///A byte array prefixed by its length as a VarInt
pub struct PrefixedByteArray {
    bytes: Vec<u8>,
}

///A byte array inferred from packet length. This is always at the end of the packet, so we just collect the iterator and return it
pub struct InferredByteArray {
    bytes: Vec<u8>,
}

pub struct Property {
    name: String,
    value: String,
    is_signed: bool,
    signature: Option<String>,
}

/// Reads a [VarInt](https://wiki.vg/Protocol#Type:VarInt) from a `u8` iterator, returning an `i32`.
/// 
/// The bytes will be consumed from the iterator.
/// 
/// See [https://wiki.vg/Protocol#VarInt_and_VarLong](https://wiki.vg/Protocol#VarInt_and_VarLong) for more details
/// 
/// # Arguments:
/// * `iter:&mut impl Iterator<Item = u8>` - the iterator to read the bytes from
/// 
pub fn read_var_int(iter: &mut impl Iterator<Item = u8>) -> Result<VarInt, Box<dyn Error + Send + Sync>> {
    let mut out: i32 = 0;
    for i in 0..4 {
        let Some(val) = iter.next() else { return Err(IterEndError{})? };
        out += i32::from(val & 0x7f) << 7*i;
        if val & 0x80 == 0 {
            return Ok(out);
        }
    }
    let Some(val) = iter.next() else { return Err(IterEndError{})? };
    if (val) & 0x80 != 0 {
        return Err("VarInt too large.")?
    }
    out += i32::from(val & 0x7f) << 7*4;
    Ok(out)
}

/// Unused
async fn read_var_int_async(stream: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<VarInt, Box<dyn Error + Send + Sync>> {
    let mut out: i32 = 0;
    for i in 0..4 {
        let Some(val) =  stream.next().await else { return Err(IterEndError{})? } ;
        out += i32::from(val & 0x7f) << 7*i;
        if val & 0x80 == 0 {
            return Ok(out);
        }
    }
    let Some(val) = stream.next().await else { return Err(IterEndError{})? };
    if (val) & 0x80 != 0 {
        return Err("VarInt too large.")?
    }
    out += i32::from(val & 0x7f) << 7*4;
    
    Ok(out)
}

/// reads a [VarLong](https://wiki.vg/Protocol#Type:VarLong) from a `u8` iterator, returning an `i64`.
/// 
/// the bytes will be consumed from the iterator.
/// 
/// see [https://wiki.vg/Protocol#VarInt_and_VarLong](https://wiki.vg/Protocol#VarInt_and_VarLong) for more details
/// 
/// # Arguments:
/// * `iter:&mut impl Iterator<Item = u8>` - the iterator to read the bytes from
///
pub fn read_var_long(iter: &mut impl Iterator<Item = u8>) -> Result<i64, Box<dyn Error + Send + Sync>> {
    let mut out: i64 = 0;
    for i in 0..9 {
        let Some(val) = iter.next() else { return Err(IterEndError{})? };
        out += i64::from(val & 0x7f) << 7*i;
        if val & 0x80 == 0 {
            return Ok(out);
        }
    }
    let Some(val) = iter.next() else { return Err(IterEndError{})? };
    if (val) & 0x80 != 0 {
        return Err("VarLong too large.")?
    }
    out += i64::from(val & 0x7f) << 7*9;
    Ok(out)
}

pub fn read_string(iter: &mut impl Iterator<Item = u8>) -> Result<String, Box<dyn Error + Send + Sync>> {
    let len = read_var_int(iter)? as usize;
    let raw = iter.take(len).collect::<Vec<u8>>();
    if raw.len() < len {
        Err(IterEndError{})?
    }
    Ok(String::from_utf8(raw)?)        
}

pub fn read_prefixed_byte_array(iter: &mut impl Iterator<Item = u8>) -> Result<PrefixedByteArray, Box<dyn Error + Send + Sync>> {
    let len = read_var_int(iter)? as usize;
    let raw = iter.take(len).collect::<Vec<u8>>();
    if raw.len() < len {
        Err(IterEndError{})?
    }
    Ok(PrefixedByteArray{bytes : raw})
}

pub fn read_inferred_byte_array(iter: &mut impl Iterator<Item = u8>) -> Result<InferredByteArray, Box<dyn Error + Send + Sync>> {
    Ok(InferredByteArray{bytes : iter.collect()})
}

pub fn read_float(iter: &mut impl Iterator<Item = u8>) -> Result<f32, Box<dyn Error + Send + Sync>> {
    //TODO: replace with take_forced(4)
    let bytes = iter.take(4).collect::<Vec<u8>>();
    if bytes.len() < 4 {
        Err(IterEndError{})?
    }
    Ok(f32::from_be_bytes(bytes.try_into().unwrap()))
}

pub fn read_double(iter: &mut impl Iterator<Item = u8>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    //TODO: replace with take_forced(4)
    let bytes = iter.take(8).collect::<Vec<u8>>();
    if bytes.len() < 8 {
        Err(IterEndError{})?
    }
    Ok(f64::from_be_bytes(bytes.try_into().unwrap()))
}

pub fn read_bool(iter: &mut impl Iterator<Item = u8>) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let Some(value) = iter.next() else { return Err(IterEndError{})? };
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err("Not a Boolean value!")?,
    }
}

pub fn read_ushort(iter: &mut impl Iterator<Item = u8>) -> Result<u16, Box<dyn Error + Send + Sync>> {
    let array: [u8; 2] = std::convert::TryFrom::try_from(iter.take(2).collect::<Vec<u8>>().as_slice())?;
    Ok(u16::from_be_bytes(array))
}

pub fn read_long(iter: &mut impl Iterator<Item = u8>) -> Result<i64, Box<dyn Error + Send + Sync>> {
    let array: [u8; 8] = std::convert::TryFrom::try_from(iter.take(8).collect::<Vec<u8>>().as_slice())?;
    Ok(i64::from_be_bytes(array))
}

pub fn read_uuid(iter: &mut impl Iterator<Item = u8>) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
    let array: [u8; 16] = std::convert::TryFrom::try_from(iter.take(16).collect::<Vec<u8>>().as_slice())?;
    Ok(Uuid::from_u128(u128::from_be_bytes(array)))
}

pub fn create_var_int(i: VarInt) -> Vec<u8> {
    let mut value: u32 = i.to_le() as u32;
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

pub fn create_var_long(l: i64) -> Vec<u8> {
    let mut value:u64 = l.to_le() as u64;
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

pub fn create_string(s: &String) -> Vec<u8> {
    let raw = s.as_bytes().to_owned().into_iter();
    let len = create_var_int(raw.len() as i32).into_iter();
    len.chain(raw).collect()
}

pub fn create_prefixed_byte_array(array: &PrefixedByteArray) -> Vec<u8> {
    create_var_int(array.bytes.len() as i32).into_iter().chain(array.bytes.clone().into_iter()).collect()
}

pub fn create_inferred_byte_array(array: &InferredByteArray) -> Vec<u8> {
    array.bytes.clone()
}

pub fn create_float(f: f32) -> Vec<u8> {
    f.to_be_bytes().to_vec()
}

pub fn create_double(d: f64) -> Vec<u8> {
    d.to_be_bytes().to_vec()
}

pub fn create_bool(b: bool) -> Vec<u8> {
    match b {
        true => vec![1u8],
        false => vec![0u8],
    }
}

pub fn create_ushort(us: u16) -> Vec<u8> {
    us.to_be_bytes().to_vec()
}

pub fn create_long(l: i64) -> Vec<u8> {
    l.to_be_bytes().to_vec()
}

pub fn create_uuid(uuid: Uuid) -> Vec<u8> {
    //really counterintuitive, but to_u128_le gives an Little Endian representation of a UUID in Big Endian,
    //so we want to retain this byte order by using to_le_bytes(), which contains a Little Endian representation
    //of the data which has been flipped to Big Endian
    uuid.to_u128_le().to_le_bytes().to_vec()
}

pub trait Optional<T> {
    fn read_func(iter: &mut impl Iterator<Item = u8>) -> Result<T, Box<dyn Error + Send + Sync>>;
    fn create_func(data: T) -> Vec<u8>;
}

impl Optional<VarInt> for VarInt {
    #[inline]
    fn read_func(iter: &mut impl Iterator<Item = u8>) -> Result<VarInt, Box<dyn Error + Send + Sync>> {
        read_var_int(iter)
    }
    #[inline]
    fn create_func(i: VarInt) -> Vec<u8> {
        create_var_int(i)
    }
}

impl Optional<InferredByteArray> for InferredByteArray {
    #[inline]
    fn read_func(iter: &mut impl Iterator<Item = u8>) -> Result<InferredByteArray, Box<dyn Error + Send + Sync>> {

        read_inferred_byte_array(iter)
    }
    #[inline]
    fn create_func(data: InferredByteArray) -> Vec<u8> {
        create_inferred_byte_array(&data)
    }
}

pub fn read_option<T>(iter: &mut impl Iterator<Item = u8>) -> Result<Option<T>, Box<dyn Error + Send + Sync>> 
where
    T: Optional<T>
{
    let Some(is_some) = iter.next() else { return Err(IterEndError{})? };
    let is_some = is_some != 0;
    if is_some {
        Ok(Some(T::read_func(iter)?))
    } else {
        Ok(None)
    }
}



pub fn create_option<T>(option: Option<T>) -> Vec<u8>
where
    T: Optional<T>
{
    if option.is_some() {
        let mut out = vec![1u8];
        out.extend(T::create_func(option.unwrap()).into_iter());
        out
    } else {
        vec![0u8]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(string2.err().unwrap().to_string(), IterEndError{}.to_string());

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
    
}