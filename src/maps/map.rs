
use std::collections::{hash_map, HashMap};

use hecs::Entity;
use bracket_lib::prelude::*;
use bracket_lib::pathfinding::{SmallVec,DistanceAlg,a_star_search};
use crate::statistics::Pools;
use crate::State;


pub const MAPWIDTH : i32 = 78;
pub const MAPHEIGHT : i32 = 32;
pub const MAPSIZE : usize = MAPWIDTH as usize * MAPHEIGHT as usize;
//use crate::rect;

pub fn new(new_depth : i32) -> Map
{
    Map
    {
        map : vec![TileType::Wall; MAPSIZE],

        revealed_tiles : vec![false; MAPSIZE],
        visible_tiles : vec![false; MAPSIZE],
        blocked : vec![false; MAPSIZE],
        tile_contents : vec![Vec::new(); MAPSIZE],
        depth: new_depth,
        props: HashMap::new(),
        
    }
}



#[derive(PartialEq,Clone, Copy, Debug)]
pub enum TileType
{
    Floor, Wall, DownStairs,
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
}

impl BaseMap for Map
{
    fn is_opaque(&self, _idx: usize) -> bool {
        if self.map[_idx as usize] == TileType::Wall { true}
        else 
        {false}
    }   
    fn get_available_exits(&self, idx:usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % MAPWIDTH;
        let y = idx as i32 / MAPWIDTH;
        let w = MAPWIDTH as usize;
    
        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };

            // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, 1.45)); }
    
        exits
    }
    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        let w = MAPWIDTH as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
    
}

impl Algorithm2D for Map
{
    fn dimensions(&self) -> Point {
        Point::new(MAPWIDTH,MAPHEIGHT)
    }

}
impl Map
{

    fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > MAPWIDTH-1 || y < 1 || y > MAPHEIGHT-1 { return false; }
        let idx = Map::xy_id(x, y);
        !self.blocked[idx as usize]
    }

    pub fn new(new_depth : i32) -> Map
    {
        Map
        {
            map: vec![TileType::Wall; MAPSIZE],
            revealed_tiles : vec![false;MAPSIZE],
            visible_tiles: vec![false;MAPSIZE],
            blocked : vec![false;MAPSIZE],
            tile_contents : vec![Vec::new(); MAPSIZE] , 
            depth: new_depth,
            props: HashMap::new(),
        }

    }
    pub fn xy_id(x:i32,y:i32) ->usize 
    {
    (y as usize * MAPWIDTH as usize)+ x as usize
    }

    pub fn populate_blocked(&mut self)
    {
        for (i,tile) in self.map.iter_mut().enumerate()
        {
            self.blocked[i] = *tile == TileType::Wall;
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
        for ent in self.tile_contents[Map::xy_id(position.x, position.y)].iter()
        {
            if state.world.get::<&Pools>(*ent).is_ok()
            {
                mobs.push(*ent);
            }
        }
        mobs
    }

}

pub fn _create_map() -> Vec<TileType> 
{
let mut  map = vec![TileType::Floor; 80*50];
for x in 0..80
{
    map[Map::xy_id(x, 0)] = TileType::Wall;
    map[Map::xy_id(x,49)] = TileType::Wall;
}
for y in 0..50
{
    map[Map::xy_id(0,y)] = TileType::Wall;
    map[Map::xy_id(79,y)] = TileType::Wall;
}
map
}


pub fn draw_map(ctx:&mut BTerm,map:&Map)
{
let mut x = 0;
let mut y = 0;
for tile in map.map.iter()
{
if map.revealed_tiles[Map::xy_id(x,y)] == true
{
    let glyph : FontCharType;
    let mut fg;
match tile
{
    TileType::Floor =>
    {
        glyph = FontCharType::from('.' as u8);
        fg = RGB::named(RED3);
    }// ctx.set(x, y, RGB::from_f32(0.5,0.5,0.5),RGB::from_f32(0., 0., 0.), '.'),
    TileType::Wall => 
    {
        glyph = wall_glyph(map, x, y);
        fg = RGB::from_f32(0.1,1.,0.);
    }//ctx.set(x, y, RGB::from_f32(0.,1.,0.),RGB::from_f32(0., 0., 0.), '#'),
    TileType::DownStairs =>
    {
        glyph = FontCharType::from('>' as u8);
        fg = RGB::named(WHITE);

    }
}
if !map.visible_tiles[Map::xy_id(x, y)]
{
    fg = fg.to_greyscale()
}
ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
}
x+= 1;
if  x > MAPWIDTH - 1
{
x = 0;
y += 1;
}
}
}


impl Map
{

fn apply_rooms(&mut self,rooms: &Vec<Rect>)
{
    for r in rooms.iter()
    {
        r.for_each(|xy| self.map[Map::xy_id(xy.x, xy.y)] = TileType::Floor);
    }
    }

}

fn wall_glyph(map : &Map, x: i32, y: i32) -> FontCharType
{
    if x < 1 || x > MAPWIDTH-2 || y < 1 || y > MAPHEIGHT-2 as i32 { return 35; }
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
    let idx = Map::xy_id(x, y);

    map.map[idx] == TileType::Wall && map.revealed_tiles[idx]
}


