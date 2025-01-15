use std::collections::HashMap;

use bracket_lib::prelude::Point;

use crate::{spawns::spawning_system::spawn_region, Position};

use super::{Map, MapBuilder, TileType, MAPHEIGHT, MAPWIDTH};




const MINROOMSIZE : i32 = 8;

pub struct CellularAutomataBuilder
{
    map : Map,
    starting_position : Point,
    depth : i32,
    noise_areas : HashMap<i32, Vec<usize>>,

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

            //Find starting point by starting at middle and heading left until a floor tile is found
            self.starting_position = Point::new(MAPWIDTH/2, MAPHEIGHT/2);
            let mut start_idx = Map::xy_id(self.starting_position.x, self.starting_position.y);

            while self.map.map[start_idx] != TileType::Floor
            {
                self.starting_position.x -= 1;
                start_idx = Map::xy_id(self.starting_position.x, self.starting_position.y);
            }

            //use a djikstra map to find what areas are unreachable from starting position and cull them
            let map_starts : Vec<usize> = vec![start_idx];

            let djikstra_map = bracket_lib::pathfinding::DijkstraMap::new(MAPWIDTH, MAPHEIGHT,
                &map_starts, &self.map,  200.0);

            let mut exit_tile = (0, 0.0f32);
            for (i, tile) in self.map.map.iter_mut().enumerate()
            {
                if *tile == TileType::Floor
                {
                    let distance_to_start = djikstra_map.map[i];
                    //this indicates that it is unreachable from starting position
                    if distance_to_start == std::f32::MAX
                    {
                        *tile = TileType::Wall;
                    }
                    else
                    {
                        //if it is futher away than current exit candidate set this to be the exit
                        if distance_to_start > exit_tile.1
                        {
                            exit_tile.0  = i;
                            exit_tile.1 = distance_to_start;
                        }    
                    }
                }
            }

            self.map.map[exit_tile.0] = TileType::DownStairs;
        
        //Builds up a new noise map to use for entity spawning
        let mut rng = bracket_lib::random::RandomNumberGenerator::new();
        let mut noise = bracket_lib::noise::FastNoise::seeded(rng.roll_dice(1, 65536) as u64);

        noise.set_noise_type(bracket_lib::noise::NoiseType::Cellular);
        noise.set_frequency(0.08);
        noise.set_cellular_distance_function(bracket_lib::noise::CellularDistanceFunction::Manhattan);

        for y in 1..MAPHEIGHT-1
        {
            for x in 1..MAPWIDTH-1
            {
                let idx = Map::xy_id(x, y);
                if self.map.map[idx] == TileType::Floor
                {
                    let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.;
                    let cell_value = cell_value_f as i32;

                    if self.noise_areas.contains_key(&cell_value)
                    {
                        self.noise_areas.get_mut(&cell_value).unwrap().push(idx);
                    }
                    else 
                    {
                        self.noise_areas.insert(cell_value, vec![idx]);
                    }

                }
            }
        }


        self.map.clone()
    }
    
    fn spawn_entities(&mut self, state : &mut crate::State) {
        for area in self.noise_areas.iter()
        {
            spawn_region(state, area.1, self.depth);
        }
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
            starting_position : Point::zero(),
            noise_areas : HashMap::new(),
        }
    }
}