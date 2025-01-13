use bracket_lib::prelude::Point;

use crate::Position;

use super::{Map, MapBuilder, TileType, MAPHEIGHT, MAPWIDTH};




const MINROOMSIZE : i32 = 8;

pub struct CellularAutomataBuilder
{
    map : Map,
    starting_position : Point,
    depth : i32,

}


impl MapBuilder for CellularAutomataBuilder
{
    fn build(&mut self) -> Map
    {
        let mut rng = bracket_lib::random::RandomNumberGenerator::new();

        //Completely randomize the map to start with
        for y in 1..MAPHEIGHT-1
        {
            for x in 1..MAPWIDTH-1
            {
                let roll = rng.roll_dice(1, 100);
                let idx = Map::xy_id(x, y);
                if roll > 55 {self.map.map[idx] = TileType::Floor}
                else {self.map.map[idx] = TileType::Wall}
            }
        }

        //Iteratively apply cellular automata rules
        for _i in 0..15
        {
            let mut newtiles = self.map.map.clone();

            for y in 1..MAPHEIGHT-1
            {
                for x in 1..MAPWIDTH-1
                {
                    let idx = Map::xy_id(x, y);
                    let mut neighbors = 0;

                    if self.map.map[idx - 1] == TileType::Wall { neighbors += 1; }
                    if self.map.map[idx + 1] == TileType::Wall { neighbors += 1; }
                    if self.map.map[idx - MAPWIDTH as usize] == TileType::Wall { neighbors += 1; }
                    if self.map.map[idx + MAPWIDTH as usize] == TileType::Wall { neighbors += 1; }
                    if self.map.map[idx - (MAPWIDTH as usize - 1)] == TileType::Wall { neighbors += 1; }
                    if self.map.map[idx - (MAPWIDTH as usize + 1)] == TileType::Wall { neighbors += 1; }
                    if self.map.map[idx + (MAPWIDTH as usize - 1)] == TileType::Wall { neighbors += 1; }
                    if self.map.map[idx + (MAPWIDTH as usize + 1)] == TileType::Wall { neighbors += 1; }

                    if neighbors > 4 || neighbors == 0
                    {
                        newtiles[idx] = TileType::Wall;
                    } else 
                    {
                        newtiles[idx] = TileType::Floor;    
                    }
                }
            }

            self.map.map = newtiles.clone();
        }

        self.map.clone()
    }
    
    fn spawn_entities(&mut self, state : &mut crate::State) {
        todo!()
    }
    
    fn get_map(&mut self) -> Map {
        self.map.clone()
    }
    
    fn get_starting_position(&mut self) -> bracket_lib::prelude::Point {
        self.starting_position
    }
}


impl CellularAutomataBuilder
{
    pub fn new(new_depth : i32) -> CellularAutomataBuilder
    {
        CellularAutomataBuilder
        {
            depth : new_depth,
            map : Map::new(new_depth),
            starting_position : Point::zero()
        }
    }
}