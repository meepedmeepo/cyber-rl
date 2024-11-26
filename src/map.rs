

use bracket_lib::prelude::*;
//use crate::rect;
#[derive(PartialEq,Clone, Copy)]
pub enum TileType
{
    Floor,Wall,
}

pub struct MapRoomBundle
{
    pub map : Vec<TileType>,
    pub rooms: Vec<Rect>,
}

impl MapRoomBundle
{
    fn new(map : Vec<TileType> , rooms:Vec<Rect>) -> MapRoomBundle
    {
        MapRoomBundle
        {
            map,
            rooms,
        }

    }

}

pub fn _create_map() -> Vec<TileType> 
{
let mut  map = vec![TileType::Floor; 80*50];
for x in 0..80
{
    map[xy_id(x, 0)] = TileType::Wall;
    map[xy_id(x,49)] = TileType::Wall;
}
for y in 0..50
{
    map[xy_id(0,y)] = TileType::Wall;
    map[xy_id(79,y)] = TileType::Wall;
}
map
}
pub fn xy_id(x:i32,y:i32) ->usize 
{
    (y as usize *80)+ x as usize
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
pub fn create_room_map() -> MapRoomBundle
{
    let mut  map = vec![TileType::Wall; 80*50];
   
    let mut rooms : Vec<Rect> = Vec::new();
    let mut rng = bracket_lib::random::RandomNumberGenerator::new();
    
    while rooms.len() < 5
    {
        let  room = create_room(&mut rng);
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
        r.for_each(|xy| map[xy_id(xy.x, xy.y)] = TileType::Floor);
    }
    
    MapRoomBundle::new(map, rooms)

}

pub fn create_room( rng : &mut bracket_lib::random::RandomNumberGenerator) -> Rect
{ 
    let x =rng.range(1, 79);
    let w = rng.range(4,15);
    let y: i32 = rng.range(1,49);
    let h = rng.range(3,7);
    //Rect::with_exact(x1,x2,y2,y2)
    Rect::with_size(x, y, w, h)

}