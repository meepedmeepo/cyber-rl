
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
        rooms : Vec::new(),
 
        revealed_tiles : vec![false; MAPSIZE],
        visible_tiles : vec![false; MAPSIZE],
        blocked : vec![false; MAPSIZE],
        tile_contents : vec![Vec::new(); MAPSIZE],
        depth: new_depth,
        
    }
}



#[derive(PartialEq,Clone, Copy)]
pub enum TileType
{
    Floor, Wall, DownStairs,
}

pub struct Map
{
    pub map : Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_contents: Vec<Vec<Entity>>,
    pub depth : i32,
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

    pub fn new(map : Vec<TileType> , rooms:Vec<Rect>) -> Map
    {
        Map
        {
            map,
            rooms,
            revealed_tiles : vec![false;MAPSIZE],
            visible_tiles: vec![false;MAPSIZE],
            blocked : vec![false;MAPSIZE],
            tile_contents : vec![Vec::new(); MAPSIZE] , depth: 0
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
    let glyph;
    let mut fg;
match tile
{
    TileType::Floor =>
    {
        glyph = '.';
        fg = RGB::from_f32(0.85, 0.85, 0.85);
    }// ctx.set(x, y, RGB::from_f32(0.5,0.5,0.5),RGB::from_f32(0., 0., 0.), '.'),
    TileType::Wall => 
    {
        glyph = '#';
        fg = RGB::from_f32(0.1,1.,0.);
    }//ctx.set(x, y, RGB::from_f32(0.,1.,0.),RGB::from_f32(0., 0., 0.), '#'),
    TileType::DownStairs =>
    {
        glyph = '>';
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
    pub fn check_map_validity(&self) -> bool
    {
        let p1 = self.rooms[0].center();
        for room in self.rooms.iter().skip(1)
        {
            let p2 = room.center();
            if !self.check_room_path(p1,p2)
            {
                return false;
            }
        }
        true
    }
    pub fn check_room_path(&self, p1 : Point, p2 : Point) -> bool
    {
        //let distance = DistanceAlg::Pythagoras.distance2d(p1,p2);
        let path = a_star_search(Map::xy_id(p1.x, p1.y), Map::xy_id(p2.x, p2.y), self);
        if path.success && path.steps.len() > 2
        {
            return true;
        }
        return false;
    }
    pub fn generate_map_checked(state: &mut State)
    {
        let is_valid = false;
        let mut i = 1;
        while !is_valid
        {
           state.map = Map::create_room_map(state);
           state.map.create_map_corridors();
           if state.map.check_map_validity()
           {
            console::log(format!("Successfully generated map after {} tries!",i));

            let pos = state.map.rooms[state.map.rooms.len()-1].center();

            let idx = Map::xy_id(pos.x, pos.y);

            state.map.map[idx] = TileType::DownStairs;

            return;
           }
           else 
           {
            console::log(format!("Failed to generate valid map! Attempt {}!",i));
            i+=1;
           }
        }
    }


pub fn create_room_map(state : &mut State) -> Map
{
    let mut  map = vec![TileType::Wall; MAPSIZE];
   
    let mut rooms : Vec<Rect> = Vec::new();

    
    while rooms.len() < 14
    {
        let  room = Map::create_room(state);
        let mut intersects = false;
        for r in rooms.iter()
        {
            if room.intersect(r)
            {
                intersects = true;
            }
        }
        if !intersects
        {
            rooms.push(room);
        }
    }
    for r in rooms.iter()
    {
        r.for_each(|xy| map[Map::xy_id(xy.x, xy.y)] = TileType::Floor);
    }
    
    Map::new(map, rooms)

}

pub fn create_room( state : &mut State) -> Rect
{ 
    let x =state.rng.range(1, MAPWIDTH - 6);
    let mut  w = state.rng.range(4,15);
    let y: i32 = state.rng.range(1,MAPHEIGHT - 6);
    let mut h = state.rng.range(3,15);
    //Rect::with_exact(x1,x2,y2,y2)
    if x+w > MAPWIDTH -2
    {
        w = MAPWIDTH-2-x;
    }
    if y + h > MAPHEIGHT -2
    {
        h = MAPHEIGHT-2 - y;
    }
    Rect::with_size(x, y, w, h)

}

pub fn create_map_corridors(&mut self)
{
        //let mut start : Point;
        //let mut target : Point;
        let mut rooms : Vec<Rect> = Vec::new();
        rooms = self.generate_simple_corridors(  &mut rooms);
        //rooms = self.generate_simple_corridors( &mut rooms);
    
        self.apply_rooms(&mut rooms);

}

fn generate_simple_corridors (&self,
    rooms :&mut Vec<Rect>) ->  Vec<Rect>
{
    let mut start: Point;
    let mut target: Point;
    let mut rng = bracket_lib::random::RandomNumberGenerator::new();
    for r in self.rooms.iter()
    {
        let start_x = rng.range(r.x1,r.x2);
        let start_y = rng.range(r.y1, r.y2);
        start = Point::new(start_x, start_y);

        let mut is_valid_target = false;
        let mut target_room = *r;
        while !is_valid_target
         {
            target_room = self.rooms[rng.range(0, self.rooms.len())];
            
            if target_room!= *r
            {
                is_valid_target = true;
            }
        }

        //let target_room = state.maproom.rooms[rng.range(0, state.maproom.rooms.len())];
        
        let target_x = rng.range(target_room.x1+1, target_room.x2);
        let target_y = rng.range(target_room.y1+1, target_room.y2);
        
        target = Point::new(target_x,target_y);
    
        let  r1 = Rect::with_exact(start.x, start.y, target.x+2, start.y+2);
        let  r2 = Rect::with_exact(target.x,start.y,target.x+2,target.y+2);
        rooms.push(r1);
        rooms.push(r2);
    }
    rooms.to_vec()
}


fn apply_rooms(&mut self,rooms: &Vec<Rect>)
{
    for r in rooms.iter()
    {
        r.for_each(|xy| self.map[Map::xy_id(xy.x, xy.y)] = TileType::Floor);
    }
    }

}

