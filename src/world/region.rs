use std::{cmp::{max, min}, collections::HashMap, fs::File, ops::Deref, path::PathBuf, sync::Weak};

use super::chunk::Chunk;
use rayon::iter::*;

pub struct Region {
    x: i32,
    z: i32,
}

impl Region {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x : x,
            z : z,
        }
    }

    pub(in super) fn load_chunks(&self, locations: Vec<(i32, i32)>) -> Result<Vec<((i32, i32), Chunk)>, std::io::Error> {
        Ok(vec![])
    }

    pub(in super) fn save_chunks(&self, chunks: Vec<((i32, i32), Chunk)>) -> Result<(), std::io::Error> {
        
        Ok(())
    }
}

#[derive(Clone)]
pub struct LoadRegion {
    x: i32,
    z: i32,
    pub load_levels: [[u8; 32];32],
}

impl LoadRegion {
    pub fn new(location: (i32, i32)) -> Self {
        Self {
            x : location.0,
            z : location.1,
            load_levels : [[34u8; 32];32],
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

pub fn flatten_load_regions_at_location(location: (i32, i32), load_regions: impl Iterator<Item = LoadRegion> + Send + Sync) -> LoadRegion {
    load_regions.fold(LoadRegion::new(location), |mut cur, nxt| {
        cur.load_levels.par_iter_mut().zip(nxt.load_levels.par_iter()).for_each(|(cur_z, nxt_z)| {
            cur_z.iter_mut().zip(nxt_z.iter()).for_each(|(cur_xz, nxt_xz)| {
                *cur_xz = min(*cur_xz, *nxt_xz);
            })
        });
        cur
    })
}

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
        load_levels : levels,
    }
}

