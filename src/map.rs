

use bracket_lib::prelude::*;
use crate::State;
//use crate::rect;
#[derive(PartialEq,Clone, Copy)]
pub enum TileType
{
    Floor,Wall,
}

pub struct Map
{
    pub map : Vec<TileType>,
    pub rooms: Vec<Rect>,
}

impl Map
{
    pub fn new(map : Vec<TileType> , rooms:Vec<Rect>) -> Map
    {
        Map
        {
            map,
            rooms,
        }

    }
    pub fn xy_id(x:i32,y:i32) ->usize 
    {
    (y as usize *80)+ x as usize
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


pub fn draw_map(ctx:&mut BTerm,map:&[TileType])
{
let mut x = 0;
let mut y = 0;
for tile in map.iter()
{
match tile
{
    TileType::Floor => ctx.set(x, y, RGB::from_f32(0.5,0.5,0.5),RGB::from_f32(0., 0., 0.), '.'),
    TileType::Wall => ctx.set(x, y, RGB::from_f32(0.,1.,0.),RGB::from_f32(0., 0., 0.), '#'),
}
x+= 1;
if  x > 79
{
x = 0;
y += 1;
}
}
}


impl Map
{
pub fn create_room_map(state : &mut State) -> Map
{
    let mut  map = vec![TileType::Wall; 80*50];
   
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
                break;
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
    let x =state.rng.range(1, 74);
    let mut  w = state.rng.range(4,15);
    let y: i32 = state.rng.range(1,43);
    let mut h = state.rng.range(3,15);
    //Rect::with_exact(x1,x2,y2,y2)
    if x+w > 78
    {
        w = 78-x;
    }
    if y + h > 48
    {
        h = 48 - y;
    }
    Rect::with_size(x, y, w, h)

}

pub fn create_map_corridors(&mut self)
{
        //let mut start : Point;
        //let mut target : Point;
        let mut rooms : Vec<Rect> = Vec::new();
        rooms = self.generate_simple_corridors(  &mut rooms);
        rooms = self.generate_simple_corridors( &mut rooms);
    
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

