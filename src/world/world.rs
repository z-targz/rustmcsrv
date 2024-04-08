use core::hash;
use std::error::Error;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex, RwLock, Weak};

use dashmap::DashMap;

use chashmap::CHashMap;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::data_types::identifier::Identifier;
use crate::entity::entity_base::EntityBase;
use crate::packet::play::{self, CUpdateTime};
use crate::player::Player;
use crate::THE_SERVER;

use rayon::iter::*;

use super::chunk::{Chunk, ChunkCache};
use super::region::{LoadRegion, TicketRegion, TicketType};

const TIME_DAY: i64 = 0;
const TIME_NOON: i64 = 6000;
const TIME_NIGHT: i64 = 12000;
const TIME_MIDNIGHT: i64 = 18000;

pub(in super) trait WorldTrait {}

pub struct World {
    players: DashMap<i32, Weak<Player>>,
    loaded_entities: DashMap<i32, Weak<dyn EntityBase + Send + Sync>>,
    identifier: Identifier,
    chunk_sections: u8,
    world_age: Mutex<i64>,
    world_time: Mutex<i64>,
    beds_explode: bool,
    ticket_regions: RwLock<HashMap<(i32, i32), TicketRegion>>,
    load_regions: HashMap<(i32, i32), LoadRegion>,
    loaded_chunks: DashMap<(i32, i32), Arc<Mutex<Chunk>>>,
    tickets: Mutex<Vec<Arc<Ticket2>>>,
    chunk_cache: ChunkCache,
}

impl World {
    /// ### NOTE: max_height_times_16 is the world height * 16
    /// For the overworld, this is 24 (for a max height of 384) \
    /// For the nether, this is 16 (for a max height of 256)
    /// 
    /// Use this function when loading the world from a directory
    pub fn new(identifier: Identifier, max_height_times_16: u8, world_age: i64, world_time: i64, beds_explode: bool) -> Self {
        let the_world = World {
            players : DashMap::with_capacity(THE_SERVER.get_max_players() as usize),
            loaded_entities: DashMap::new(),
            identifier : identifier,
            chunk_sections : max_height_times_16,
            world_age : world_age.into(),
            world_time : world_time.into(),
            beds_explode : beds_explode,
            ticket_regions : HashMap::new().into(),
            load_regions : HashMap::new(),
            loaded_chunks : DashMap::new(),
            tickets : Vec::new().into(),
            chunk_cache : ChunkCache::new(NonZeroUsize::new(
                (((THE_SERVER.get_properties().get_view_distance() + 2) * 2 + 1).pow(2) * THE_SERVER.get_properties().get_max_players()
                + ((THE_SERVER.get_properties().get_spawn_chunk_radius() + 2) * 2 + 1).pow(2)) as usize
            ).unwrap()),
        };

        //TODO: use world spawn location instead of 0, 0
        the_world.create_chunk_ticket((0, 0), 34 - THE_SERVER.get_properties().get_spawn_chunk_radius() as u8, -1, TicketType::Start);

        the_world
    }

    /// ### NOTE: max_height_times_16 is the world height * 16
    /// For the overworld, this is 24 (for a max height of 384) \
    /// For the nether, this is 16 (for a max height of 256)
    /// 
    /// Use this function to create a new world and create the necessary files
    pub fn create_new_world(identifier: Identifier, max_height_times_16: u8, beds_explode: bool) -> Option<Self> {
        //TODO: create world files
        let new_world = World {
            players : DashMap::with_capacity(THE_SERVER.get_max_players() as usize),
            loaded_entities: DashMap::new(),
            identifier : identifier,
            chunk_sections : max_height_times_16,
            world_age : 0.into(),
            world_time : TIME_NOON.into(),
            beds_explode : beds_explode,
            ticket_regions : HashMap::new().into(),
            load_regions : HashMap::new(),
            loaded_chunks : DashMap::new(),
            tickets: Vec::new().into(),
            chunk_cache : ChunkCache::new(NonZeroUsize::new(
                (((THE_SERVER.get_properties().get_view_distance() + 2) * 2 + 1).pow(2) * THE_SERVER.get_properties().get_max_players()
                + ((THE_SERVER.get_properties().get_spawn_chunk_radius() + 2) * 2 + 1).pow(2)) as usize
            ).unwrap()),
        };

        Some(new_world)
    }

    pub fn add_player(&self, player_id: i32, weak: Weak<Player>) {
        self.players.insert(player_id, weak);
    }

    pub fn remove_player_by_id(&self, player_id: i32) {
        self.players.remove(&player_id);
    }

    pub fn get_chunk_sections(&self) -> u8 {
        self.chunk_sections
    }

    pub fn load_chunks(&self) {

    }

    pub fn handle_tickets(&mut self) {

        let mut ticket_regions_lock = self.ticket_regions.write().unwrap();
        let mut regions_to_recompute: Vec<_> = 
        ticket_regions_lock.iter_mut()
            .map(|(_, ticket_region)| ticket_region.get_tickets_mut())
            .flat_map(|ticket_hmap| {
                ticket_hmap.extract_if(|_, ticket| {
                    !ticket.is_valid()
                })
            })
            .flat_map(|(_, ticket)| ticket.get_affected_regions().clone())
            .collect();
        
        drop(ticket_regions_lock);

        //TODO: add new tickets

        let view_distance: i32 = THE_SERVER.get_properties().get_view_distance();
        for weak_player in self.players.iter() {
            match weak_player.upgrade() {
                Some(player) => {
                    let player_pos = player.get_position();
                    let chunk_loc = ((player_pos.x as i64 / 16) as i32, (player_pos.z as i64 / 16) as i32);
                    
                    for x in -view_distance..=view_distance {
                        for z in -view_distance..=view_distance {
                            let loc = (chunk_loc.0 + x, chunk_loc.1 + z);
                            let mut affected_regions = self.create_chunk_ticket(loc, 21, 100, TicketType::Player);
                            regions_to_recompute.append(&mut affected_regions);
                        }
                    }
                },
                None =>{self.players.remove(weak_player.key());}
            }
        }

        let ticket_regions_lock = self.ticket_regions.read().unwrap();

        for region in regions_to_recompute {
            let neighbors: Vec<_> = (-1..=1).zip(-1..=1)
                .filter_map(|(x, z)| {
                    ticket_regions_lock.get(&(x, z))
                }).collect();

            let this_region = ticket_regions_lock.get(&region).unwrap();

            if neighbors.iter()
                .flat_map(|neighbor| neighbor.get_tickets())
                .chain(this_region.get_tickets())
                .try_len() //Can unwrap because iter is finite
                .unwrap() == 0 {
                    self.load_regions.insert((region.0, region.1), super::region::LoadRegion::new((region.0, region.1)));
            } else {
                self.load_regions.insert((region.0, region.1), super::region::compute_load_region(this_region, neighbors));
            }
        }
        drop(ticket_regions_lock);

        let mut ticket_regions_lock = self.ticket_regions.write().unwrap();
        ticket_regions_lock.iter_mut()
            .flat_map(|(_, ticket_region)| ticket_region.get_tickets_mut())
            .for_each(|(_, ticket)| ticket.dec_life_time());

        ticket_regions_lock.retain(|_, ticket_region| {
            ticket_region.get_tickets().len() > 0
        })
    }

    pub fn tick(&mut self) {
        let mut world_age_lock = self.world_age.lock().unwrap();
        let mut world_time_lock = self.world_time.lock().unwrap();
        if *world_age_lock % 20 == 0 {
            for weak in self.players.iter() {
                match weak.upgrade() {
                    Some(arc) => {
                        let world_age = *world_age_lock;
                        let world_time = *world_time_lock;
                        crate::RUNTIME.spawn(async move {
                            match arc.send_packet(CUpdateTime::new(world_age, world_time)).await {
                                Ok(_) => (),
                                Err(_) => {
                                    arc.disconnect("Connection lost").await;
                                },
                            }
                        });
                    },
                    None => {
                        self.players.remove(weak.key());
                    }
                }
            }
        }

        //TODO: Plugin Scheduler stuff

        //TODO: World Border Logic

        //TODO: Weather

        *world_age_lock += 1;
        *world_time_lock = (*world_time_lock + 1) % 24000;

        drop(world_age_lock);
        drop(world_time_lock);

        if !self.beds_explode {
            //TODO: Sleeping logic
        }
        

        //TODO: Scheduled commands

        //TODO: Scheduled block ticks
        //TODO: Scheduled fluid ticks

        //TODO: Long time away: Raid Logic

        //TODO: (soon) Chunk Load Level, load chunks
        
        self.handle_tickets();
        
        //TODO: 
        /*for chunk in loaded_chunks {
            spawn mobs
            tick ice and snow (later)
            random ticks
        }*/

        //TODO: (soon) Send block changes to players

        //TODO: whatever tf points of interest are


        //TODO: Send unloaded chunks to chunk cache

        //TODO: (much later, with plugin) Tick dragon fight

        //TODO: for each (concurrent) non-passenger entity:
        /*
            - Check if it can despawn
            - Tick entity
            - Tick passengers
        */

        //TODO: tick block entities

        //TODO: handle game events (whatever this implies)

        //TODO: handle incoming packets from players
        //TODO: send queued packets to players
        //TODO: send player info to players


        //TODO: every 6000 ticks autosave
        //TODO: run pending tasks


        

    }

    pub fn create_chunk_ticket(&self, location: (i32, i32), level: u8, life_time: i64, ticket_type: TicketType) -> Vec<(i32, i32)> {
        let mut regions: HashSet<(i32, i32)> = HashSet::new();
        let radius = level as i32 - 1;
        let p_p = ((location.0 + radius)/32, (location.1 + radius)/32);
        let n_p = ((location.0 - radius)/32, (location.1 + radius)/32);
        let p_n = ((location.0 + radius)/32, (location.1 - radius)/32);
        let n_n = ((location.0 - radius)/32, (location.1 - radius)/32);
        regions.insert(p_p);
        regions.insert(n_p);
        regions.insert(p_n);
        regions.insert(n_n);
        
        let region: (i32, i32) = (location.0 / 32, location.1 / 32);
        let offset: (i32, i32) = (((location.0 % 32) + 32) % 32, ((location.1 % 32) + 32) % 32);

        let affected_regions: Vec<_> = regions.into_iter().collect();

        let ticket = super::region::Ticket::new(offset.0, offset.1, affected_regions.clone(), level, life_time, ticket_type);

        let mut ticket_regions_lock = self.ticket_regions.write().unwrap();

        match ticket_regions_lock.get_mut(&region) {
            Some(ticket_region) => {
                ticket_region.add_ticket(offset, ticket);
            },
            None => {
                let mut ticket_region = TicketRegion::new(region);
                ticket_region.add_ticket(offset, ticket);
                ticket_regions_lock.insert(region, ticket_region);
            }
        }
        drop(ticket_regions_lock);
        affected_regions
    }

    pub fn create_ticket_2(&mut self, ticket: Ticket2) {
        let ticket_arc = Arc::new(ticket);
        self.tickets.lock().unwrap().push(ticket_arc.clone());

        let n = ticket_arc.get_radius() as i32;
        let x = ticket_arc.get_x();
        let z = ticket_arc.get_z();

        ((x-n)..=(x+n)).zip((z-n)..=(z+n)).for_each(|(chunk_x, chunk_z)| {
            match self.loaded_chunks.get(&(chunk_x, chunk_z)) {
                Some(chunk) => {
                    chunk.lock().unwrap().block_tickets.push(Arc::downgrade(&ticket_arc))
                },
                None => {
                    match self.chunk_cache.get(&(chunk_x, chunk_z)) {
                        Some(_) => todo!(),
                        None => todo!(),
                    }
                    //TODO: Load Chunk
                },
            }
        });
        if n >= 1 {
            ((1+x-n)..=(x+n-1)).zip((1+z-n)..=(z+n-1)).for_each(|(chunk_x, chunk_z)| {

            });
        }
        if n >= 2 {
            ((2+x-n)..=(x+n-2)).zip((2+z-n)..=(z+n-2)).for_each(|(chunk_x, chunk_z)| {

            });
        }
    }
}



/*
pub fn fold_tickets<'a>(tickets: impl Iterator<Item = &'a Ticket> + Send + Sync) -> HashMap<(i32, i32), u8> {
    let out: CHashMap<(i32, i32), u8> = CHashMap::new();
    tickets.par_bridge().flat_map(|ticket| {
        out.reserve(ticket.get_capacity());
        ticket.get_map()
    }).for_each(|entry| {
        if out.contains_key(&entry.0) {
            out.insert(entry.0.clone(), std::cmp::max(entry.1.clone(), out.get(entry.0).unwrap().clone()));
        } else {
            out.insert(entry.0.clone(), entry.1.clone());
        }
    });
    out.into_iter().collect()
}
*/

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.chunk_sections == other.chunk_sections
    }
}


#[derive(Clone)]
pub struct Ticket2 {
    center: (i32, i32),
    radius: u8,
    life_time: i64,
}

impl Ticket2 {
    pub fn new(center: (i32, i32), radius: u8, life_time: i64) -> Self {
        Self {
            center : center,
            radius : radius,
            life_time : life_time,
        }
    }
    #[inline]
    pub fn get_x(&self) -> i32 {
        self.center.0
    }

    #[inline]
    pub fn get_z(&self) -> i32 {
        self.center.1
    }

    #[inline]
    pub fn get_center(&self) -> (i32, i32) {
        self.center
    }

    #[inline]
    pub fn get_radius(&self) -> u8 {
        self.radius
    }
}