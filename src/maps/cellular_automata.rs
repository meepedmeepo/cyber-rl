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
    fn build(new_depth: i32) -> Map
    {
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