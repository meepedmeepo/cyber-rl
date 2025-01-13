pub mod map;
pub mod common;
pub mod simple_map;
mod cellular_automata;
use crate::map::*;


pub trait MapBuilder
{
    fn build(new_depth: i32) -> Map;
}



