use std::collections::HashSet;

use crate::{maps::waveform_collapse::tile_idx_in_chunk, Map, TileType, MAPHEIGHT, MAPWIDTH};

use super::MapChunk;



pub fn build_patterns(map : &Map, chunk_size : i32
    , include_flipping : bool, dedupe : bool) -> Vec<Vec<TileType>>
    {
        let chunks_x = MAPWIDTH / chunk_size;
        let chunks_y = MAPHEIGHT / chunk_size;

        let mut patterns = Vec::new();

        for cy in 0 .. chunks_y
        {
            for cx in 0..chunks_x
            {
                //normal orientation
                let mut pattern : Vec<TileType> = Vec::new();
                let start_x =  cx * chunk_size;
                let end_x = (cx + 1) * chunk_size;
                let start_y = cy * chunk_size;
                let end_y = (cy + 1) * chunk_size;

                for y in start_y .. end_y
                {
                    for x in start_x .. end_x
                    {
                        let idx = Map::xy_id(x, y);
                        pattern.push(map.map[idx]);
                    }
                }

                patterns.push(pattern);

                if include_flipping 
                {
                    // Flip horizontal
                    pattern = Vec::new();
                    for y in start_y .. end_y {
                        for x in start_x .. end_x {
                            let idx = Map::xy_id(end_x - (x+1), y);
                            pattern.push(map.map[idx]);
                        }
                    }
                    patterns.push(pattern);
    
                    // Flip vertical
                    pattern = Vec::new();
                    for y in start_y .. end_y {
                        for x in start_x .. end_x {
                            let idx = Map::xy_id(x, end_y - (y+1));
                            pattern.push(map.map[idx]);
                        }
                    }
                    patterns.push(pattern);
    
                    // Flip both
                    pattern = Vec::new();
                    for y in start_y .. end_y {
                        for x in start_x .. end_x {
                            let idx = Map::xy_id(end_x - (x+1), end_y - (y+1));
                            pattern.push(map.map[idx]);
                        }
                    }
                    patterns.push(pattern);
                }


            }
        }

        //Remove duplicate patterns!
        if dedupe
        {
            let set : HashSet<Vec<TileType>> = patterns.drain(..).collect();
            patterns.extend(set.into_iter());
        }


        patterns
    }


// pub fn render_pattern_to_map(map : &mut Map, chunk: &MapChunk, chunk_size : i32, start_x : i32, start_y : i32)
// {
//     let mut i = 0usize;

//     for tile_y in 0..chunk_size
//     {
//         for tile_x in 0..chunk_size
//         {
//             let map_idx = Map::xy_id(start_x+tile_x, start_y+tile_y);
//             map.map[map_idx] = chunk.pattern[i];
//             //map.vi
//         }
//     }
// }


pub fn patterns_to_constraints(patterns : Vec<Vec<TileType>>, chunk_size : i32) -> Vec<MapChunk>
{
    let mut constraints : Vec<MapChunk> = Vec::new();

    for p in patterns
    {
        let mut new_chunk = MapChunk
        {
            pattern: p,
            exits: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            has_exits : true,
            compatible_with: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        };

        for exit in new_chunk.exits.iter_mut()
        {
            for _i in 0..chunk_size
            {
                exit.push(false);
            }
        }

        let mut n_exits = 0;
        for x in 0..chunk_size
        {
            //checks for north-bound exits
            let north_idx = tile_idx_in_chunk(chunk_size, x, 0);
            if new_chunk.pattern[north_idx] == TileType::Floor
            {
                new_chunk.exits[0][x as usize] = true;
                n_exits += 1;
            }

            //Check for south-bound exits
            let south_idx = tile_idx_in_chunk(chunk_size, x, chunk_size-1);
            if new_chunk.pattern[south_idx] == TileType::Floor
            {
                new_chunk.exits[1][x as usize] = true;
                n_exits += 1;
            }

            //Check for west-bound exits
            let west_idx = tile_idx_in_chunk(chunk_size, 0, x);
            if new_chunk.pattern[west_idx] == TileType::Floor
            {
                new_chunk.exits[2][x as usize] = true;
                n_exits += 1;
            }

            //Check for east-bound exits
            let east_idx = tile_idx_in_chunk(chunk_size, chunk_size-1, x);
            if new_chunk.pattern[east_idx] == TileType::Floor
            {
                new_chunk.exits[3][x as usize] = true;
                n_exits += 1;
            }
        }

        if n_exits == 0
        {
            new_chunk.has_exits = false;
        }

        constraints.push(new_chunk);
    }

    //Creates a compatibility matrix

    //this clone is to stop borrow checker problems of double dipping into constraints!
    let ch = constraints.clone();
    for c in constraints.iter_mut()
    {
        for (j, potential) in ch.iter().enumerate()
        {
            //if there are no exits, they are compatible
            if !c.has_exits || !potential.has_exits 
            {
                for compat in c.compatible_with.iter_mut()
                {
                    compat.push(j);
                }
            }else 
            {
                //Evaluate compatibility by direction
                for (direction, exit_list) in c.exits.iter_mut().enumerate()
                {
                    let opposite = match direction
                    {
                        0 => 1,//Our north, their south
                        1 => 0,//Our south, their north
                        2 => 3,//Our west, their east
                        _ => 2 //Our east, their west
                    };

                    let mut it_fits = false;
                    let mut has_any = false;

                    for (slot, can_enter) in exit_list.iter().enumerate()
                    {
                        if *can_enter
                        {
                            has_any = true;
                            if potential.exits[opposite][slot]
                            {
                                it_fits = true;
                            }
                        }
                    }

                    if it_fits 
                    {
                        c.compatible_with[direction].push(j);
                    }

                    if !has_any
                    {
                        //There are no exits on this side
                        //can only match if other edge has no exits!

                        let matching_exit_count = potential.exits[opposite].iter()
                            .filter(|a| !**a).count();

                        if matching_exit_count == 0
                        {
                            c.compatible_with[direction].push(j);
                        }
                    }
                }    
            }
        }
    }

    constraints
}