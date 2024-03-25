use std::sync::{Arc, Mutex, Weak};

use dashmap::DashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::data_types::CJSONTextComponent;
use crate::data_types::{vec_3d::Vec3d, Rotation};
use crate::world::World;

pub mod entity_data;



pub enum EntityType {
    Player,
    Item,
}

/// The boolean traits should be implemented as flags of a byte.
/// 
/// `0x01` - is on fire
/// 
/// `0x02` - is crouching
/// 
/// `0x04` - unused
/// 
/// `0x08` - is sprinting
/// 
/// `0x10` - is swimming
/// 
/// `0x20` - is invisible
/// 
/// `0x40` - is glowing
/// 
/// `0x80` - is using elytra
/// 
pub trait EntityBase {


    /// Returns whether the entity is on fire. For now, this should be hard set to false.
    fn is_on_fire(&self) -> bool
        where Self: Sized;
    
    /// Should always be false except for players unless you want to do some weird stuff
    fn is_crouching(&self) -> bool
        where Self: Sized;

    /// Should always be false except for players
    fn is_sprinting(&self) -> bool
        where Self: Sized;

    fn is_swimming(&self) -> bool
        where Self: Sized;
    
    fn is_invisible(&self) -> bool
        where Self: Sized;

    /// Spectral arrow effect
    fn is_glowing(&self) -> bool
        where Self: Sized;

    fn is_using_elytra(&self) -> bool
        where Self: Sized;


    /// Returns the global Entity ID which is a unique ID for each instance of the server
    fn get_eid(&self) -> i32
        where Self: Sized;

    /// Returns a copy of the position of the entity
    fn get_position(&self) -> Vec3d
        where Self: Sized;

    
    
    /// Returns the yaw and pitch of the entity as a `Rotation`
    fn get_look(&self) -> Rotation
        where Self: Sized;
    
    /// Returns the world the entity is located in
    fn get_world(&self) -> Option<Weak<World>>
        where Self: Sized;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    flags: u8,                          //default: 0
    air: i32,                           //default: 300
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<CJSONTextComponent>,   //default: None
    is_custom_name_visible: bool,       //default: false
    is_silent: bool,                    //default: false
    has_no_gravity: bool,               //default: false
    pose: Pose,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum Pose {
    STANDING = 0,
    FALL_FLYING = 1, 
    SLEEPING = 2, 
    SWIMMING = 3, 
    SPIN_ATTACK = 4, 
    SNEAKING = 5, 
    LONG_JUMPING = 6, 
    DYING = 7, 
    CROAKING = 8, 
    USING_TONGUE = 9, 
    SITTING = 10, 
    ROARING = 11, 
    SNIFFING = 12, 
    EMERGING = 13, 
    DIGGING = 14 
}