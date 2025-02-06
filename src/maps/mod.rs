pub mod map;
pub mod common;
pub mod simple_map;
mod cellular_automata;
mod room_based_spawns;
mod room_based_starting_pos;
mod room_based_stairs;
mod area_starting_pos;
mod cull_unreachable;
mod  voronoi_spawning;
mod distant_exit;
mod prefab_builder;
mod dogleg_corridors;
mod bsp_dungeon;
mod bsp_corridors;
mod room_sorter;
mod corridors_nearest_neighbour;
mod corridor_spawner;
mod door_placement;
mod rex_assests;
mod waveform_collapse;
mod voronoi;
mod locations;
mod tile_type;
mod border_wall;


use crate::{map::*, spawns::spawning_system::{self, get_entity_type, EntityType}, State};
use bracket_lib::{prelude::{Point, Rect}, random::RandomNumberGenerator};
use cellular_automata::CellularAutomataBuilder;
use simple_map::*;
use room_based_spawns::*;
use room_based_starting_pos::*;
use room_based_stairs::*;
use area_starting_pos::*;
use cull_unreachable::*;
use voronoi_spawning::*;
use distant_exit::*;
use prefab_builder::*;
use dogleg_corridors::*;
use bsp_dungeon::*;
use bsp_corridors::*;
use room_sorter::*;
use corridors_nearest_neighbour::*;
use corridor_spawner::*;
use door_placement::*;
pub use rex_assests::*;
use waveform_collapse::*;
use voronoi::*;
pub use locations::*;
pub use tile_type::*;
use border_wall::*;
//s
pub trait MapBuilder
{
    fn build(&mut self) -> Map;
    fn spawn_entities(&mut self, state : &mut State);
    fn get_map(&mut self) -> Map;
    fn get_starting_position(&mut self) -> Point;
}

pub fn random_map_builder(new_depth : i32, width : i32, height : i32) -> BuilderChain
{
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


pub struct BuilderMap
{
    pub spawn_list : Vec<(usize, String)>,
    pub map : Map,
    pub starting_position : Option<Point>,
    pub rooms : Option<Vec<Rect>>,
    pub corridors :Option<Vec<Vec<usize>>>,
}

pub struct BuilderChain
{
    starter : Option<Box<dyn InitialMapBuilder>>,
    builders : Vec<Box<dyn MetaMapBuilder>>,
    pub build_data : BuilderMap
}

impl BuilderChain
{
    pub fn new(new_depth : i32, width : i32, height : i32) -> BuilderChain
    {
        BuilderChain
        {
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap
            {
                spawn_list: Vec::new(),
                map: Map::new(new_depth, width, height),
                starting_position: None,
                rooms: None,
                corridors: None,
            }
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>)
    {
        match  self.starter 
        {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have 1 starting builder!")    
        };
    }

    pub fn with(&mut self, metabuilder : Box<dyn MetaMapBuilder>)
    {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self, rng : &mut bracket_lib::random::RandomNumberGenerator)
    {
        match &mut self.starter
        {
            None => panic!("Cannot run map builder chain without starting builder!"),
            Some(starter) =>
            {
                starter.build_map(rng, &mut self.build_data);
            }
        }

         //Build the rest of the layers one by oneself.lay_concrete(build_data);
        for metabuilder in self.builders.iter_mut()
        {
            metabuilder.build_map(rng, &mut self.build_data);
        }
    }

    pub fn spawn_entities(&mut self, state : &mut State)
    {
        for entity in self.build_data.spawn_list.iter()
        {
            let entity_type = get_entity_type( &entity.1);

            spawning_system::spawn_entity(state, &(&entity.0, &entity.1)
                ,entity.0 as i32 % state.map.map_width, entity.0 as i32 / state.map.map_width, entity_type);
        }
    }
}

pub trait InitialMapBuilder
{
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap);
}
pub trait MetaMapBuilder
{
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap);
}
