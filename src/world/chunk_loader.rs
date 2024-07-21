use std::collections::HashMap;

use super::anvil::region::Region;

pub enum ChunkLoader
{
    Static {
        loader: StaticLoader,
    },

    Vanilla {
        loader: VanillaLoader,
    }
}

impl ChunkLoader
{
    pub fn new_static_region(
        level_name: String, 
        (x_min, z_min): (i32, i32), 
        (x_max, z_max): (i32, i32)
    ) -> Result<ChunkLoader, std::io::Error> {
        Self::new_static(
            level_name, 
            (x_min..=x_max)
                .flat_map(|x| {
                    (z_min..=z_max).map(move |z| (x, z))
                }).collect()
        )
    }

    pub fn new_static(level_name: String, locations: Vec<(i32, i32)>) -> Result<ChunkLoader, std::io::Error> {
        let mut out = Self::Static {
            loader: StaticLoader {
                level_name: level_name,
                locations: locations.clone(),
                loaded_regions: HashMap::new(),
            }
        };

        if let ChunkLoader::Static{ ref mut loader } = out {
            loader.load_chunks(locations)?
        }
        Ok(out)
    }
    pub fn new_vanilla(level_name: String) -> Self {
        Self::Vanilla {
            loader: VanillaLoader {
                level_name: level_name,
                loaded_regions: HashMap::new(),
            }
        }
    }

    pub fn get_loader(&mut self) -> Box<&mut dyn Loader> {
        match self {
            ChunkLoader::Static { loader } => Box::new(loader),
            ChunkLoader::Vanilla { loader } => Box::new(loader),
        }
    }
}

/// Used to load a static chunk region
pub struct StaticLoader {
    level_name: String,
    locations: Vec<(i32, i32)>,
    loaded_regions: HashMap<(i32, i32), Region>,
}

impl Loader for StaticLoader {
    fn get_loaded_regions(&mut self) -> &mut HashMap<(i32, i32), Region> {
        &mut self.loaded_regions
    }

    fn get_level_name(&self) -> &String {
        &self.level_name
    }
    
    fn tick_chunks(&mut self) {
        for (_, region) in &mut self.loaded_regions {
            region.get_loaded_chunks_mut().into_iter().for_each(|(_, chunk)| {
                chunk.tick_blocks();
                chunk.tick_entities();
            })
        }
    }
}

pub struct VanillaLoader {
    level_name: String,
    loaded_regions: HashMap<(i32, i32), Region>,
    //TODO
}

impl Loader for VanillaLoader {
    fn get_loaded_regions(&mut self) -> &mut HashMap<(i32, i32), Region> {
        &mut self.loaded_regions
    }

    fn get_level_name(&self) -> &String {
        &self.level_name
    }

    fn tick_chunks(&mut self) {
        todo!();
    }
}

pub trait Loader {
    fn load_chunks(&mut self, locations: Vec<(i32, i32)>) -> Result<(), std::io::Error> {
        let mut region_operations: HashMap<(i32, i32),Vec<(i32, i32)>> = HashMap::new();
        locations.into_iter().for_each(|(x, z)| {
            region_operations.entry((x / 32, z / 32)).or_default().push((x.rem_euclid(32), z.rem_euclid(32)))
        });

        let level_name = self.get_level_name().clone();
        
        for ((reg_x, reg_z), offsets) in region_operations {
            match self.get_loaded_regions().entry((reg_x, reg_z)).or_insert_with(|| {
                Region::new(reg_x, reg_z, level_name.clone())
            }).load_chunks(offsets, &mut None) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn unload_chunks(&mut self, locations: Vec<(i32, i32)>) {
        let mut region_operations: HashMap<(i32, i32),Vec<(i32, i32)>> = HashMap::new();
        locations.into_iter().for_each(|(x, z)| {
            region_operations.entry((x / 32, z / 32)).or_default().push((x.rem_euclid(32), z.rem_euclid(32)))
        });

        let level_name = self.get_level_name().clone();

        for ((reg_x, reg_z), offsets) in region_operations {
            self.get_loaded_regions().entry((reg_x, reg_z)).or_insert_with(|| { //TODO: Why?
                Region::new(reg_x, reg_z, level_name.clone())
            }).unload_chunks(offsets, None);
            if self.get_loaded_regions().get(&(reg_x, reg_z)).unwrap().has_no_loaded_chunks() {
                self.get_loaded_regions().remove(&(reg_x, reg_z));
            }
        }
    }

    fn save_world(&mut self) -> Result<(), std::io::Error> {
        for (_, region) in self.get_loaded_regions() {
            match region.save_all_chunks() {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn get_loaded_regions(&mut self) -> &mut HashMap<(i32, i32), Region>;

    fn get_level_name(&self) -> &String;

    fn tick_chunks(&mut self);
}

mod tests {

}