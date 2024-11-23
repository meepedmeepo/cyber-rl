use bracket_lib::prelude::*;
//use crate::rect;
#[derive(PartialEq,Clone, Copy)]
pub enum TileType
{
    Floor,Wall,
}

pub fn create_map() -> Vec<TileType> 
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
fn create_room_map()
{
    let mut rooms : Vec<Rect>;
   let rng = bracket_lib::random::RandomNumberGenerator::new();
}

fn create_room( rng : &mut bracket_lib::random::RandomNumberGenerator) -> Rect
{ 
    let x1 =rng.range(1, 79);
    let x2 = rng.range(x1+3,x1+7);
    let y1 = rng.range(1,49);
    let y2 = rng.range(y1+3, y1+6);
    Rect::with_exact(x1,x2,y2,y2)

}