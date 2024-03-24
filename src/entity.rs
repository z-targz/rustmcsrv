use std::sync::{Arc, Mutex, Weak};

use dashmap::DashMap;

use crate::data_types::{vec_3d::Vec3d, Rotation};

use crate::world::World;


pub struct Entities {
    entities: DashMap<i32, Weak<dyn EntityBase>>,
    entity_id_cap: std::sync::Mutex<i32>,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            entities : DashMap::new(),
            entity_id_cap : Mutex::new(0)
        }
    }
    pub fn get_next_eid(&self) -> i32 {
        let mut lock = self.entity_id_cap.lock().unwrap();
        let x: i32 = *lock;
        *lock += 1;
        x
    }
}

pub enum EntityType {
    Player,
    Item,
}

pub trait EntityBase {
    /// Returns the global Entity ID which is a unique ID for each instance of the server
    fn get_eid(&self) -> i32
        where Self: Sized;

    /// Returns a copy of the position of the entity
    fn get_position(&self) -> Vec3d
        where Self: Sized;

    /// Returns whether the entity is on fire. For now, this should be hard set to false.
    fn is_on_fire(&self) -> bool
        where Self: Sized;
    
    /// Returns the yaw and pitch of the entity as a `Rotation`
    fn get_look(&self) -> Rotation
        where Self: Sized;
    
    /// Returns the world the entity is located in
    fn get_world(&self) -> Option<Arc<World>>
        where Self: Sized;
}

