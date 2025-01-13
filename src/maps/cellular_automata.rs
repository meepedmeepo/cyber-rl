use crate::Position;

use super::{Map, MapBuilder};




const MINROOMSIZE : i32 = 8;

pub struct CellularAutomataBuilder
{
    map : Map,
    starting_position : Position,
    depth : i32,

}


impl MapBuilder for CellularAutomataBuilder
{
    fn build(&mut self) -> Map
    {
        todo!()   
    }
    
    fn spawn_entities(&mut self, state : &mut crate::State) {
        todo!()
    }
    
    fn get_map(&mut self) -> Map {
        todo!()
    }
    
    fn get_starting_position(&mut self) -> bracket_lib::prelude::Point {
        todo!()
    }
}


impl CellularAutomataBuilder
{
    pub fn new(new_depth : i32) -> CellularAutomataBuilder
    {
        // CellularAutomataBuilder
        // {
        //     map: Map::new(map, rooms),
            
        // }
        todo!()
    }
}