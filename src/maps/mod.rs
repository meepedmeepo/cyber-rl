pub mod map;
pub mod common;
pub mod simple_map;
mod cellular_automata;
use crate::{map::*, State};
use bracket_lib::prelude::Point;
use cellular_automata::CellularAutomataBuilder;
use simple_map::*;

pub trait MapBuilder
{
    fn build(&mut self) -> Map;
    fn spawn_entities(&mut self, state : &mut State);
    fn get_map(&mut self) -> Map;
    fn get_starting_position(&mut self) -> Point;
}

pub fn random_map_builder(new_depth : i32) -> Box<dyn MapBuilder>
{

    Box::new(CellularAutomataBuilder::new(new_depth))
}


