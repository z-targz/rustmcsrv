
use std::collections::HashMap;


use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::Weak;
use std::hash::Hash;


use dashmap::DashMap;



use lru::LruCache;
use std::sync::Mutex;

//use crate::entity::entity_base::EntityBase;

use crate::entity::entity::Entity;




// #[derive(Clone)]
// pub enum LoadType {
//     EntityTicking,
//     BlockTicking,
//     Border,
//     Unloaded,
// }


/// Make sure to remove the invalid weak pointers when ticking the chunk!
pub struct Chunk {
    x: i32,
    z: i32,
    sections: Vec<Option<ChunkSection>>,
    entities: DashMap<i32, Weak<Mutex<Entity>>>,
    //pub entity_tickets: Vec<Weak<Ticket2>>,
    //pub block_tickets: Vec<Weak<Ticket2>>,
    //pub border_tickets: Vec<Weak<Ticket2>>,
}

impl Clone for Chunk {
    fn clone(&self) -> Self {
        Self { 
            x: self.x,
            z: self.z,
            sections: self.sections.clone(), 
            entities: self.entities.clone(), 
            //entity_tickets: Vec::new(), 
            //block_tickets: Vec::new(), 
            //border_tickets: Vec::new(), 
        }
    }
}

impl Hash for Chunk {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.z.hash(state);
    }
}

impl Chunk {
    #[inline]
    pub fn new(x: i32, z: i32, height: i32) -> Self {
        Self {
            x : x,
            z : z,
            sections : std::iter::repeat(None).take((height/16) as usize).collect(),
            entities : DashMap::new(),
            //entity_tickets : Vec::new().into(),
            //block_tickets: Vec::new().into(),
            //border_tickets : Vec::new().into(),
        }
    }

    #[inline]
    pub fn get_num_chunk_sections(&self) -> u8 {
        self.sections.len() as u8
    }

    #[inline]
    pub fn get_coords(&self) -> (i32, i32) {
        (self.get_x(), self.get_x())
    }

    #[inline]
    pub fn get_x(&self) -> i32 {
        self.x
    }
    
    #[inline]
    pub fn get_z(&self) -> i32 {
        self.z
    }

    pub fn tick_entities(&mut self) {
        for entry in self.entities.iter() {
            match entry.value().upgrade() {
                Some(entity) => {
                    ()
                },
                None => {
                    self.entities.remove(entry.key());
                }
            }
        };
    }

    pub fn tick_blocks(&mut self) {
        todo!();
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_state: u16) {

        let chunk_section_idx = y / 16;
        let chunk_section_offset_y = y % 16;
        
        //NOTE: Make sure to do the bounds checking and y adjustment in the world's set block method
        unsafe {
            match self.sections.get_unchecked_mut(chunk_section_idx) {
                Some(section) => section.set_block(x, chunk_section_offset_y, z, block_state),
                None => ChunkSection { data: Default::default() }.set_block(x, chunk_section_offset_y, z, block_state),
            }
        }
    }

    pub fn set_blocks(&mut self, operations: HashMap<(usize, usize, usize), u16>) {
        for ((x, y, z), block_state) in operations {
            let chunk_section_idx = y / 16;
            let chunk_section_offset_y = y % 16;

            //NOTE: Make sure to do the bounds checking and y adjustment in the world's set block method
            unsafe {
                match self.sections.get_unchecked_mut(chunk_section_idx) {
                    Some(section) => section.set_block(x, chunk_section_offset_y, z, block_state),
                    None => ChunkSection { data: Default::default() }.set_block(x, chunk_section_offset_y, z, block_state),
                }
            }
        }
    }

}

#[derive(Clone)]
pub struct ChunkSection {
    data: [[[u16; 16]; 16]; 16],
}

impl ChunkSection {
    pub fn new(data: [[[u16; 16]; 16]; 16]) -> Self {
        ChunkSection {
            data: data
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_state: u16) {
        self.data[x][y][z] = block_state;
    }

    pub fn set_blocks(&mut self, operations: HashMap<(usize, usize, usize), u16>) {
        for ((x, y, z), block_state) in operations {
            self.data[x][y][z] = block_state;
        }
    }
}

pub struct ChunkCache {
    max_capacity: NonZeroUsize,
    cache: LruCache<(i32, i32), Arc<std::sync::Mutex<Chunk>>>
}

impl ChunkCache {
    pub fn new(max_capacity: NonZeroUsize) -> Self {
        Self {
            max_capacity : max_capacity,
            cache : LruCache::new(max_capacity),
        }
    }

    pub fn add(&mut self, chunk: Chunk) -> Option<((i32, i32), Arc<Mutex<Chunk>>)> {
        self.cache.push((chunk.x, chunk.z), Arc::new(Mutex::new(chunk)))
    }

    pub async fn save(&self) -> Vec<((i32, i32), Chunk)> {
        self.cache.iter()
            .map(|(a, b)| 
                (a.clone(), b.clone().lock().unwrap().clone())
            ).collect()
    }

    pub fn evacuate_unchecked(self) -> Vec<((i32, i32), Chunk)> {
        self.cache.into_iter()
            .map(|(a, b)| {
                (a, Arc::into_inner(b).unwrap().into_inner().unwrap())
            }).collect()
    }

    pub fn get(&mut self, at: &(i32, i32)) -> Option<Arc<Mutex<Chunk>>> {
        self.cache.get(at).cloned()
    }
}


/*
#[derive(Clone)]
pub struct Ticket {
    center_location: (i32, i32),
    map: HashMap<(i32, i32), u8>,
    life_time: i64,
    radius: u8,
}


#[derive(Clone)]
pub struct Ticket2 {
    center_location: (i32, i32),
    regions: HashMap<(i32, i32), LoadRegion>,
    life_time: i64,
    level: u8,
}

pub struct Ticket3 {
    position: (i32, i32),
    level: u8,
}

impl Ticket3 {
    pub fn get_pos(&self) -> (i32, i32) {
        self.position
    }
    
    pub fn get_level(&self) -> u8 {
        self.level
    }
}

impl Ticket {
    pub fn get_center(&self) -> (i32, i32) {
        self.center_location
    }

    pub fn create_single_chunk_ticket(center_location: (i32, i32), level: u8, life_time: i64) -> Option<Self> {
        let mut the_map = HashMap::with_capacity((level as usize-1).pow(2));
        the_map.insert(center_location, level);
        let c = center_location;
        for n in 1..level as i32 {
            ((2-n)..n).for_each(|k| {
                the_map.insert((c.0 + n, c.1 + k), (level as i32 - n) as u8);
                the_map.insert((c.0 - n, c.1 - k), (level as i32 - n) as u8);
                the_map.insert((c.0 - k, c.1 + n), (level as i32 - n) as u8);
                the_map.insert((c.0 + k, c.1 - n), (level as i32 - n) as u8);
            })
        }
        Some(Self {
            center_location : center_location,
            map : the_map,
            life_time : life_time,
            radius : level - 1,
        })
    }

    /*pub fn create_single_chunk_ticket2(center: (i32, i32), level: u8, life_time: i64) -> Option<HashMap<(i32, i32), LoadRegion>> {
        let mut regions: HashMap<(i32, i32), LoadRegion> = HashMap::new();
        let radius = level as i32 - 1;
        let p_p = ((center.0 + radius)/32, (center.1 + radius)/32);
        let n_p = ((center.0 - radius)/32, (center.1 + radius)/32);
        let p_n = ((center.0 + radius)/32, (center.1 - radius)/32);
        let n_n = ((center.0 - radius)/32, (center.1 - radius)/32);
        regions.insert(p_p, LoadRegion::new(p_p));
        regions.insert(n_p, LoadRegion::new(n_p));
        regions.insert(p_n, LoadRegion::new(p_n));
        regions.insert(n_n, LoadRegion::new(n_n));

        regions.get_mut(&center).unwrap().load_levels[(center.0 % 32) as usize][(center.1 % 32) as usize] = level;
        for n in 1..level as i32 {
            ((2-n)..n).for_each(|k| {
                let p0 = (center.0 + n, center.1 + k);
                regions.get_mut(&(p0.0/32, p0.1/32)).unwrap().load_levels[(p0.0 % 32) as usize][(p0.1 % 32) as usize] = (level as i32 - n) as u8;
                let p1 = (center.0 - n, center.1 - k);
                regions.get_mut(&(p1.0/32, p1.1/32)).unwrap().load_levels[(p1.0 % 32) as usize][(p1.1 % 32) as usize] = (level as i32 - n) as u8;
                let p2 = (center.0 - k, center.1 + n);
                regions.get_mut(&(p2.0/32, p2.1/32)).unwrap().load_levels[(p2.0 % 32) as usize][(p2.1 % 32) as usize] = (level as i32 - n) as u8;
                let p3 = (center.0 + k, center.1 - n);
                regions.get_mut(&(p3.0/32, p3.1/32)).unwrap().load_levels[(p3.0 % 32) as usize][(p3.1 % 32) as usize] = (level as i32 - n) as u8;
            })
        }
        Some(regions)
        
    }*/

    /// Do not give this ticket to the world, give this ticket to the player and clone it each tick,
    /// setting the new `life_time` to `1` tick. 
    /// 
    /// This way, we do not waste compute resources recreating
    /// the ticket every tick unless the player has moved into a new chunk.
    /// 
    /// The player instance needs to recreate its ticket using this method upon moving into a different chunk.
    /// 
    /*
    pub fn create_player_ticket(center_location: (i32, i32)) -> Option<Self> {
        let v = crate::THE_SERVER.get_properties().get_view_distance();
        let level = 3;
        let mut the_map = HashMap::with_capacity((2 * (v as usize + 1 + 2)).pow(2));

        let c = center_location;
        the_map.insert(center_location.clone(), 3);
        ((-v)..=v).zip((-v)..=v).for_each(|(i, j)| {
                the_map.insert((c.0 + i, c.1 + j), level);
            }
        );
        
        ((-v)..=(v+1)).for_each(|k| {
            the_map.insert((c.0 + v + 1, c.1 + k    ), level - 1);
            the_map.insert((c.0 - v - 1, c.1 - k    ), level - 1);
            the_map.insert((c.0 - k    , c.1 + v + 1), level - 1);
            the_map.insert((c.0 + k    , c.1 - v - 1), level - 1);
        });

        ((-v-1)..=(v+2)).for_each(|k| {
            the_map.insert((c.0 + v + 2, c.1 + k    ), level - 2);
            the_map.insert((c.0 - v - 2, c.1 - k    ), level - 2);
            the_map.insert((c.0 - k    , c.1 + v + 2), level - 2);
            the_map.insert((c.0 + k    , c.1 - v - 2), level - 2);
        });

        Some(Self {
            center_location : center_location,
            map : the_map,
            life_time : -1,
            radius : v as u8 + 2,
        }) 
    }*/

    /// Decrements the lifetime by 1 tick
    pub fn dec_life_time(&mut self) {
        if self.life_time > 0 {
            self.life_time -= 1;
        }
    }

    pub fn set_life_time(&mut self, lifetime: i64) {
        self.life_time = lifetime;
    }

    pub fn is_valid(&self) -> bool {
        self.life_time != 0
    }

    pub fn get_map(&self) -> &HashMap<(i32, i32), u8> {
        &self.map
    }

    pub fn get_capacity(&self) -> usize {
        self.map.len()
    }
}
*/