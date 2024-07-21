use std::{cmp::{max, min}, collections::HashMap, fs::File, io::Read, ops::Deref, path::{Path, PathBuf}, sync::Weak};

use crate::world::chunk::Chunk;
use itertools::Itertools;
use lru::LruCache;
use rayon::iter::*;

pub struct Region {
    x: i32,
    z: i32,
    loaded_chunks: HashMap<(i32, i32), Chunk>,
    level_name: String,
    region_file_data: Vec<u8>,
}

impl Region {
    pub fn new(x: i32, z: i32, level_name: String) -> Self {
        Self {
            x : x,
            z : z,
            loaded_chunks : HashMap::new(),
            level_name: level_name.clone(),
            region_file_data: File::open(Path::new(format!("{level_name}/region/").as_str())).unwrap().bytes().try_collect().unwrap()
        }
    }

    pub fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32, cache: &mut Option<LruCache<(i32, i32), Chunk>>) -> Result<&mut Chunk, std::io::Error> {
        match cache {
            //Use Cache
            Some(chunk_cache) => match chunk_cache.pop(&(chunk_x, chunk_z)) {
                Some(cached_chunk) => {
                    self.loaded_chunks.insert((chunk_x, chunk_z), cached_chunk);
                    Ok(self.loaded_chunks.get_mut(&(chunk_x, chunk_z)).unwrap())
                },
                None => {
                    todo!()
                },
            },
            //Load directly from disk
            None => {
                todo!()
            },
        }
    }

    pub fn load_chunks(&mut self, locations: Vec<(i32, i32)>, cache: &mut Option<LruCache<(i32, i32), Chunk>>) -> Result<Vec<(i32, i32)>, std::io::Error> {
        let mut locs = Vec::new();
        for (loc_x, loc_z) in locations {
            match self.load_chunk(loc_x, loc_z, cache
            ) {
                Ok(_) => locs.push((loc_x, loc_z)),
                Err(e) => return Err(e),
            }
        }
        Ok(locs)
    }

    pub fn unload_chunks(&mut self, locations: Vec<(i32, i32)>, cache: Option<&mut LruCache<(i32, i32), Chunk>>) {
        for (loc_x, loc_z) in locations {
            
        }
    }

    pub fn save_chunks(&mut self, locations: Vec<(i32, i32)>) -> Result<(), std::io::Error> {
        for (loc_x, loc_z) in locations {
            
        }
        Ok(())
    }

    pub fn save_all_chunks(&mut self) -> Result<(), std::io::Error> {
        self.save_chunks(self.loaded_chunks.keys().map(|&(loc_x, loc_z)| {
            (loc_x, loc_z)
        }).collect())?;
        Ok(())
    }

    pub fn has_no_loaded_chunks(&self) -> bool {
        self.loaded_chunks.is_empty()
    }

    pub fn get_loaded_chunks(&self) -> &HashMap<(i32, i32), Chunk> {
        &self.loaded_chunks
    }

    pub fn get_loaded_chunks_mut(&mut self) -> &mut HashMap<(i32, i32), Chunk> {
        &mut self.loaded_chunks
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_state: u16) {
        let (chunk_x, chunk_z) = (x / 32, z / 32);
        let (x_offset, z_offset) = (x.rem_euclid(32), z.rem_euclid(32));
        match self.loaded_chunks.get_mut(&(chunk_x as i32, chunk_z as i32)) {
            Some(chunk) => todo!(),
            None => todo!(),
        }
    }

    pub fn set_blocks(&mut self, operations: HashMap<(usize, usize, usize), u16>) {

    }
}

#[derive(Clone)]
pub struct LoadRegion {
    x: i32,
    z: i32,
    //pub load_levels: [[u8; 32];32],
}

impl LoadRegion {
    pub fn new(location: (i32, i32)) -> Self {
        Self {
            x : location.0,
            z : location.1,
            //load_levels : [[34u8; 32];32],
        }
    }
    pub fn get_loc(&self) -> (i32, i32) {
        (self.get_x(), self.get_z())
    }

    #[inline]
    pub fn get_x(&self) -> i32 {
        self.x
    }
    
    #[inline]
    pub fn get_z(&self) -> i32 {
        self.z
    }
}

/*pub fn flatten_load_regions_at_location(location: (i32, i32), load_regions: impl Iterator<Item = LoadRegion> + Send + Sync) -> LoadRegion {
    load_regions.fold(LoadRegion::new(location), |mut cur, nxt| {
        cur.load_levels.par_iter_mut().zip(nxt.load_levels.par_iter()).for_each(|(cur_z, nxt_z)| {
            cur_z.iter_mut().zip(nxt_z.iter()).for_each(|(cur_xz, nxt_xz)| {
                *cur_xz = min(*cur_xz, *nxt_xz);
            })
        });
        cur
    })
}*/

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub enum TicketType {
    Player,
    Forced,
    Start,
    Portal,
    Dragon,
    Teleport,
    Unknown,
}

pub struct Ticket {
    x_offset: i32,
    z_offset: i32,
    affected_regions: Vec<(i32, i32)>,
    level: u8,
    life_time: i64,
    ticket_type: TicketType,
}

impl Ticket {
    pub fn new(x_offset: i32, z_offset: i32, affected_regions: Vec<(i32, i32)>, level: u8, life_time: i64, ticket_type: TicketType) -> Self {
        Ticket {
            x_offset : x_offset,
            z_offset : z_offset,
            affected_regions : affected_regions,
            level : level,
            life_time : life_time,
            ticket_type : ticket_type,
        }
    }

    pub fn get_affected_regions(&self) -> &Vec<(i32, i32)> {
        &self.affected_regions
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn get_type(&self) -> TicketType {
        self.ticket_type
    }

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
}



pub struct TicketRegion {
    x: i32,
    z: i32,
    tickets: HashMap<(i32, i32, TicketType), Ticket>,
    
}

impl TicketRegion {
    pub fn new(location: (i32, i32)) -> Self {
        Self {
            x: location.0,
            z: location.1,
            tickets: HashMap::new(),
        }
    }

    pub fn get_tickets(&self) -> &HashMap<(i32, i32, TicketType), Ticket> {
        &self.tickets
    }

    pub fn get_tickets_mut(&mut self) -> &mut HashMap<(i32, i32, TicketType), Ticket> {
        &mut self.tickets
    }

    pub fn add_ticket(&mut self, offset: (i32, i32), ticket: Ticket) {
        self.tickets.insert((offset.0, offset.1, ticket.get_type()), ticket);
    }
}

pub fn compute_load_region(region: &TicketRegion, neighbors: Vec<&TicketRegion>) -> LoadRegion {
    let mut levels = [[34u8; 32];32];

    let mut tickets: HashMap<(i32, i32), u8> = HashMap::new();

    for neighbor in neighbors {
        let n_x = (region.x - neighbor.x) * 32;
        let n_z = (region.x - neighbor.z) * 32;
        for (_, ticket) in &neighbor.tickets {
            tickets.insert((n_x + ticket.x_offset, n_z + ticket.z_offset), ticket.level);
        }
    }

    tickets.extend(region.tickets.iter().map(|(_, ticket)| ((ticket.x_offset, ticket.z_offset), ticket.level)));

    if tickets.len() > 0 {
        for z in 0usize..=32 {
            for x in 0usize..=32 {
                match tickets.get(&(x as i32, z as i32)) {
                    Some(&level) => levels[z][x] = level,
                    None => {
                        let mut lowest = 34u8;
                        for (&location, &level) in tickets.iter() {
                            lowest = min(lowest, level - min((x as i32 - location.0).abs(), (z as i32 - location.1).abs()) as u8)
                        }
                        levels[z][x] = lowest;
                    }
                }
                
            }
        }
    }

    LoadRegion {
        x : region.x,
        z : region.z,
        //load_levels : levels,
    }
}

