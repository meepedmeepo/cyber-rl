use std::collections::HashMap;

use bracket_lib::{prelude::Point, random::RandomNumberGenerator};

use crate::{spawns::spawning_system::spawn_region, Position};

use super::{BuilderMap, InitialMapBuilder, Map, MapBuilder, MetaMapBuilder, TileType, MAPHEIGHT, MAPWIDTH};




const MINROOMSIZE : i32 = 8;

pub struct CellularAutomataBuilder {}

impl InitialMapBuilder for CellularAutomataBuilder
{
    fn build_map(&mut self, rng: &mut bracket_lib::prelude::RandomNumberGenerator, build_data: &mut super::BuilderMap) {
        self.build(rng, build_data);
    }
}

impl MetaMapBuilder for CellularAutomataBuilder
{
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) 
    {
        self.apply_iteration(build_data);
    }
}

impl CellularAutomataBuilder
{
    pub fn new() -> Box<CellularAutomataBuilder>
    {
        Box::new(CellularAutomataBuilder {})
    }

    fn build(&mut self, rng: &mut RandomNumberGenerator, build_data : &mut BuilderMap)
    {
        for y in 1..MAPHEIGHT-1
        {
            for x in 1..MAPWIDTH-1
            {
                let roll = rng.roll_dice(1, 100);
                let idx = Map::xy_id(x, y);
                if roll > 55 {build_data.map.map[idx] = TileType::Floor}
                else {build_data.map.map[idx] = TileType::Wall}
            }
        }
        //Iteratively apply cellular automata rules
        for _i in 0..15
        {
            self.apply_iteration( build_data);
        }
    }

        fn apply_iteration(&mut self, build_data : &mut BuilderMap)
        {
            let mut newtiles = build_data.map.map.clone();

            for y in 1..MAPHEIGHT-1
            {
                for x in 1..MAPWIDTH-1
                {
                    let idx = Map::xy_id(x, y);
                    let mut neighbors = 0;

                    if build_data.map.map[idx - 1] == TileType::Wall { neighbors += 1; }
                    if build_data.map.map[idx + 1] == TileType::Wall { neighbors += 1; }
                    if build_data.map.map[idx - MAPWIDTH as usize] == TileType::Wall { neighbors += 1; }
                    if build_data.map.map[idx + MAPWIDTH as usize] == TileType::Wall { neighbors += 1; }
                    if build_data.map.map[idx - (MAPWIDTH as usize - 1)] == TileType::Wall { neighbors += 1; }
                    if build_data.map.map[idx - (MAPWIDTH as usize + 1)] == TileType::Wall { neighbors += 1; }
                    if build_data.map.map[idx + (MAPWIDTH as usize - 1)] == TileType::Wall { neighbors += 1; }
                    if build_data.map.map[idx + (MAPWIDTH as usize + 1)] == TileType::Wall { neighbors += 1; }

                    if neighbors > 4 || neighbors == 0
                    {
                        newtiles[idx] = TileType::Wall;
                    } else 
                    {
                        newtiles[idx] = TileType::Floor;    
                    }
                }
            }

            build_data.map.map = newtiles.clone();
        }

}

