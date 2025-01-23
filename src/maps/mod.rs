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
//
pub trait MapBuilder
{
    fn build(&mut self) -> Map;
    fn spawn_entities(&mut self, state : &mut State);
    fn get_map(&mut self) -> Map;
    fn get_starting_position(&mut self) -> Point;
}

pub fn random_map_builder(new_depth : i32) -> Box<dyn MapBuilder>
{

    //Box::new(CellularAutomataBuilder::new(new_depth))
    todo!()
}


pub struct BuilderMap
{
    pub spawn_list : Vec<(usize, String)>,
    pub map : Map,
    pub starting_position : Option<Point>,
    pub rooms : Option<Vec<Rect>>,
}

pub struct BuilderChain
{
    starter : Option<Box<dyn InitialMapBuilder>>,
    builders : Vec<Box<dyn MetaMapBuilder>>,
    pub build_data : BuilderMap
}

impl BuilderChain
{
    pub fn new(new_depth : i32) -> BuilderChain
    {
        BuilderChain
        {
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap
            {
                spawn_list: Vec::new(),
                map: Map::new(new_depth),
                starting_position: None,
                rooms: None,
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

         //Build the rest of the layers one by one
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
                ,entity.0 as i32 % MAPWIDTH, entity.0 as i32 / MAPWIDTH, entity_type);
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
