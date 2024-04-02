use std::error::Error;

use server_util::error::ProtocolError;

pub mod registry;

pub mod angle;
pub mod death_location;
pub mod identifier;
pub mod prefixed_byte_array;
pub mod inferred_byte_array;
pub mod nbt;
pub mod position;
pub mod property_array;
pub mod statistic_array;
pub mod text_component;
pub mod var_int;
pub mod var_long;
pub mod vec_3d;

pub mod data_types;


#[allow(unused)]
pub use {
    angle::Angle,
    death_location::DeathLocation,
    identifier::Identifier, 
    prefixed_byte_array::PrefixedByteArray,
    inferred_byte_array::InferredByteArray,

    nbt::NBT,
    position::Position,
    property_array::Property,
    property_array::PropertyArray,
    statistic_array::Statistic,
    statistic_array::StatisticArray,
    text_component::TextComponent,
    var_int::VarInt,
    var_long::VarLong,
    vec_3d::Vec3d, 
    
    data_types::*,
};

#[derive(Debug, Copy, Clone)]
pub struct Rotation(pub Angle, pub Angle);


//Leaving this here so we don't break macros
pub trait Optional {}

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