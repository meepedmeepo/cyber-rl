use bracket_lib ::prelude::*;
use hecs::*;
use std::cmp::*;

struct State
{
    world : World,
    map: Vec<TileType>,
}

struct Graphic
{
    img : char,
}

impl Graphic
{
    fn new(img: char) -> Graphic
    {
        Graphic
        {
            img,
        }
    }
}
#[derive(PartialEq,Clone, Copy)]
pub enum TileType
{
    Floor,Wall,
}

fn create_map() -> Vec<TileType> 
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

fn xy_id(x:i32,y:i32) ->usize 
{
    (y as usize *80)+ x as usize
}
struct Position
{
 x: i16,
 y : i16,
}

impl Position
{
    fn new (x : i16, y: i16) -> Position
    {
        Position
        {
            x,
            y,
        }
    }

}


struct Player
{}

fn player_input_system(ctx:&BTerm, state: &mut State)
{
    match ctx.key
    {
        None => {},
        Some(key) => match key
        {
            VirtualKeyCode::Left =>try_move(state, -1, 0),
            VirtualKeyCode::Right => try_move(state,1,0),
            VirtualKeyCode::Up => try_move(state,0,-1),
            VirtualKeyCode::Down => try_move(state,0,1),
            _ =>{},


        }

    }
}

fn try_move(state: &mut State,delta_x:i16,delta_y:i16)
{
    for(_id,(_player,position)) in state.world.query_mut::<(&Player,&mut Position)>()
    {
        position.x = min(79,max(0,position.x+delta_x));
        position.y = min(49,max(0,position.y+delta_y));
    }

}

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Heya nerds");
        player_input_system(ctx, self);
        render_system(self, ctx);
        
    }
}

fn game_init ( state: &mut State)
{
    state.world.spawn((Position::new(15,25),Graphic::new('@'),Player{}));
}

fn render_system(state:&mut State, ctx: &mut BTerm)
{
    for (_id,(position,graphic)) in
    state.world.query::<(&Position,&Graphic)>().iter()
   {
       ctx.print(position.x, position.y,graphic.img)
   }

}
fn draw_map(ctx:&mut BTerm,map:&[TileType])
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
if  {x >
    
}

}

}
fn main() ->BError {
    //println!("Hello, world!");
    let context = BTermBuilder::simple80x50()
    .with_title("Rust-like")
    .build()?;

    let mut gs: State = State{
        world: World::new(),
        map : create_map(),
    };
    game_init(&mut gs);
    main_loop(context,gs)
}
