mod area_starting_pos;
mod border_wall;
mod bsp_corridors;
mod bsp_dungeon;
mod cellular_automata;
pub mod common;
mod corridor_spawner;
mod corridors_nearest_neighbour;
mod cull_unreachable;
mod distant_exit;
mod dogleg_corridors;
mod door_placement;
mod locations;
pub mod map;
mod prefab_builder;
mod rex_assests;
mod room_based_spawns;
mod room_based_stairs;
mod room_based_starting_pos;
mod room_sorter;
pub mod simple_map;
mod tile_type;
mod voronoi;
mod voronoi_spawning;
mod waveform_collapse;

use crate::{
    map::*,
    map_indexing::SPATIAL_INDEX,
    spawns::spawning_system::{self, get_entity_type, EntityType},
    State,
};
use area_starting_pos::*;
use border_wall::*;
use bracket_lib::{
    prelude::{Algorithm2D, Point, Rect},
    random::RandomNumberGenerator,
};
use bsp_corridors::*;
use bsp_dungeon::*;
use cellular_automata::CellularAutomataBuilder;
use corridor_spawner::*;
use corridors_nearest_neighbour::*;
use cull_unreachable::*;
use distant_exit::*;
use dogleg_corridors::*;
use door_placement::*;
pub use locations::*;
use prefab_builder::*;
pub use rex_assests::*;
use room_based_spawns::*;
use room_based_stairs::*;
use room_based_starting_pos::*;
use room_sorter::*;
use simple_map::*;
pub use tile_type::*;
use voronoi::*;
use voronoi_spawning::*;
use waveform_collapse::*;
//s
pub trait MapBuilder {
    fn build(&mut self) -> Map;
    fn spawn_entities(&mut self, state: &mut State);
    fn get_map(&mut self) -> Map;
    fn get_starting_position(&mut self) -> Point;
}

pub fn random_map_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(new_depth, width, height);
    //builder.start_with(CellularAutomataBuilder::new());
    //builder.start_with(VoronoiCellBuilder::pythagoras());
    //builder.with(WaveformCollapseBuilder::new());
    builder.start_with(CellularAutomataBuilder::new());
    builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    builder.with(BorderWall::new());
    builder.with(CullUnreachable::new());
    builder.with(VoronoiSpawning::new());
    builder.with(DistantExitBuilder::new());

    //builder.start_with(BspDungeon::new());
    //builder.with(RoomSorter::new());
    //builder.with(BspCorridors::new());
    //builder.with(CorridorsNearestNeighbour::new());
    //builder.with(RoomBasedStartingPosition::new());
    //builder.with(CullUnreachable::new());
    //builder.with(RoomBasedSpawns::new());
    //builder.with(CorridorSpawner::new());
    builder.with(DoorPlacement::new());
    //builder.with(RoomBasedStairs::new());

    builder
}

pub struct BuilderMap {
    pub spawn_list: Vec<(usize, String)>,
    pub map: Map,
    pub starting_position: Option<Point>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data: BuilderMap,
}

impl BuilderChain {
    pub fn new(new_depth: i32, width: i32, height: i32) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap {
                spawn_list: Vec::new(),
                map: Map::new(new_depth, width, height),
                starting_position: None,
                rooms: None,
                corridors: None,
            },
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have 1 starting builder!"),
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self, rng: &mut bracket_lib::random::RandomNumberGenerator) {
        {
            SPATIAL_INDEX
                .lock()
                .unwrap()
                .resize(self.build_data.map.dimensions().to_unsigned_tuple());
        }
        match &mut self.starter {
            None => panic!("Cannot run map builder chain without starting builder!"),
            Some(starter) => {
                starter.build_map(rng, &mut self.build_data);

                SPATIAL_INDEX
                    .lock()
                    .unwrap()
                    .resize(self.build_data.map.dimensions().to_unsigned_tuple());
            }
        }

        //Build the rest of the layers one by one self.lay_concrete(build_data);
        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(rng, &mut self.build_data);
            SPATIAL_INDEX
                .lock()
                .unwrap()
                .resize(self.build_data.map.dimensions().to_unsigned_tuple());
        }
    }

    pub fn spawn_entities(&mut self, state: &mut State) {
        for entity in self.build_data.spawn_list.iter() {
            let entity_type = get_entity_type(&entity.1);

            spawning_system::spawn_entity(
                state,
                &(&entity.0, &entity.1),
                entity.0 as i32 % state.map.map_width,
                entity.0 as i32 / state.map.map_width,
                entity_type,
            );
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap);
}
pub trait MetaMapBuilder {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap);
}
