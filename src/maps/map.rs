
use std::collections::{hash_map, HashMap, HashSet};
use std::f32::consts::PI;

use hecs::Entity;
use bracket_lib::prelude::*;
use bracket_lib::pathfinding::{SmallVec,DistanceAlg,a_star_search};
use crate::statistics::Pools;
use crate::State;
use super::tile_type::*;
use super::TileType;

//pub const MAPSIZE : usize = map.map_width as usize * map.map_height as usize;
//use crate::rect;
impl Map
{
    pub fn new(new_depth : i32, width : i32, height : i32) -> Map
    {
        Map
        {
            map : vec![TileType::Floor; (width*height) as usize],

            revealed_tiles : vec![false; (width*height) as usize],
            visible_tiles : vec![false; (width*height) as usize],
            blocked : vec![false; (width*height) as usize],
            tile_contents : vec![Vec::new(); (width*height) as usize],
            depth: new_depth,
            props: HashMap::new(),
            view_blocked : HashSet::new(),
            map_width: width,
            map_height: height,
            
        }
    }

    pub fn idx_to_pos(&self, idx : usize) -> Point
    {
        Point::new(idx as i32 % self.map_width, idx as i32 / self.map_width)
    }
}





#[derive(Debug, Clone)]
pub struct Map
{
    pub map : Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_contents: Vec<Vec<Entity>>,
    pub depth : i32,
    pub props : HashMap<i32,Entity>,
    pub view_blocked : HashSet<usize>,
    pub map_width : i32,
    pub map_height : i32,
}

impl BaseMap for Map
{
    fn is_opaque(&self, idx: usize) -> bool 
    {
        if idx > 0 && idx < self.map.len()
        {
            if tile_opaque(self.map[idx])  || self.view_blocked.contains(&idx) { true}
            else 
            {false}
        }
        else 
        {
            true
        }
    } 
    fn get_available_exits(&self, idx:usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % self.map_width;
        let y = idx as i32 / self.map_width;
        let w = self.map_width as usize;
        let tt = self.map[idx];
    
        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, tile_cost(tt))) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, tile_cost(tt))) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, tile_cost(tt))) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, tile_cost(tt))) };

            // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, tile_cost(tt) * 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, tile_cost(tt) * 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, tile_cost(tt) * 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, tile_cost(tt) * 1.45)); }
    
        exits
    }
    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        let w = self.map_width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
    
}

impl Algorithm2D for Map
{
    fn dimensions(&self) -> Point {
        Point::new(self.map_width,self.map_height)
    }

}
impl Map
{
    pub fn xy_idx(&self, x : i32,y : i32) -> usize
    {
        (y as usize * self.map_width as usize)+ x as usize
    }

    fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > self.map_width-1 || y < 1 || y > self.map_height-1 { return false; }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx as usize]
    }

    pub fn populate_blocked(&mut self)
    {
        for (i,tile) in self.map.iter_mut().enumerate()
        {
            self.blocked[i] = !tile_walkable(*tile);
        }
    }

    pub fn reset_tile_contents(&mut self)
    {
        for (_i, tile_contents) in self.tile_contents.iter_mut().enumerate()
        {
            tile_contents.clear();
        }
    }

pub fn get_mob_entities_at_position(&self, state: &State, position: Point) -> Vec<Entity>
    {
        let mut mobs = Vec::new();
        for ent in self.tile_contents[self.xy_idx(position.x, position.y)].iter()
        {
            if state.world.get::<&Pools>(*ent).is_ok()
            {
                mobs.push(*ent);
            }
        }
        mobs
    }

}




impl Map
{

fn apply_rooms(&mut self,rooms: &Vec<Rect>)
{
    for r in rooms.iter()
    {
        r.for_each(|xy| {
            let idx = self.xy_idx(xy.x, xy.y);
            self.map[idx] = TileType::Floor});
        
    }
    }

}

fn wall_glyph(map : &Map, x: i32, y: i32) -> FontCharType
{
    if x < 1 || x > map.map_width-2 || y < 1 || y > map.map_height-2 as i32 { return 35; }
    let mut mask : u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) { mask +=1; }
    if is_revealed_and_wall(map, x, y + 1) { mask +=2; }
    if is_revealed_and_wall(map, x - 1, y) { mask +=4; }
    if is_revealed_and_wall(map, x + 1, y) { mask +=8; }

    match mask {
        0 => { 9 } // Pillar because we can't see neighbors
        1 => { 186 } // Wall only to the north
        2 => { 186 } // Wall only to the south
        3 => { 186 } // Wall to the north and south
        4 => { 205 } // Wall only to the west
        5 => { 188 } // Wall to the north and west
        6 => { 187 } // Wall to the south and west
        7 => { 185 } // Wall to the north, south and west
        8 => { 205 } // Wall only to the east
        9 => { 200 } // Wall to the north and east
        10 => { 201 } // Wall to the south and east
        11 => { 204 } // Wall to the north, south and east
        12 => { 205 } // Wall to the east and west
        13 => { 202 } // Wall to the east, west, and south
        14 => { 203 } // Wall to the east, west, and north
        15 => { 206 }  // ╬ Wall on all sides
        _ => { 35 } // We missed one?
    }
}

fn is_revealed_and_wall(map : &Map, x: i32, y: i32) -> bool
{
    let idx = map.xy_idx(x, y);

    map.map[idx] == TileType::Wall && map.revealed_tiles[idx]
}


