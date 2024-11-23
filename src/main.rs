use bracket_lib::prelude::*;
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
    fg : RGB,
    bg : RGB
}

impl Graphic
{
    fn new(img: char,fg : RGB, bg: RGB) -> Graphic
    {
        Graphic
        {
            img,
            fg,
            bg
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
 x: i32,
 y : i32,
}

impl Position
{
    fn new (x : i32, y: i32) -> Position
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
            VirtualKeyCode::A=>try_move(state, -1, 0),
            VirtualKeyCode::D => try_move(state,1,0),
            VirtualKeyCode::W => try_move(state,0,-1),
            VirtualKeyCode::S => try_move(state,0,1),
            _ =>{},

        }

    }
}

fn try_move(state: &mut State,delta_x:i32,delta_y:i32)
{
    for(_id,(_player,position)) in state.world.query_mut::<(&Player,&mut Position)>()
    {
        let destination_id = xy_id(position.x+delta_x, position.y+delta_y);
        if state.map[destination_id] != TileType::Wall
        {
        position.x = min(79,max(0,position.x+delta_x));
        position.y = min(49,max(0,position.y+delta_y));
        }
    }

}

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        //ctx.print(1, 1, "Heya nerds");
        player_input_system(ctx, self);
        draw_map(ctx, self.map.as_mut_slice());
        render_system(self, ctx);
        
    }
}

fn game_init ( state: &mut State)
{
    state.world.spawn((Position::new(15,25),Graphic::new('@',RGB::from_f32(1., 1., 1.),RGB::from_f32(0., 0., 0.)),Player{}));
}

fn render_system(state:&mut State, ctx: &mut BTerm)
{
    for (_id,(position,graphic)) in
    state.world.query::<(&Position,&Graphic)>().iter()
   {
       ctx.set(position.x, position.y,graphic.fg,graphic.bg,graphic.img)
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
if  x > 79
{
x = 0;
y += 1;
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
