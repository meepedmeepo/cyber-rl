use bracket_lib::prelude::*;
use hecs::*;
use std::cmp::*;
use map::*;
mod map;

struct State
{
    world : World,
    map: Map,
    rng : bracket_lib::random::RandomNumberGenerator,
}

struct Renderable
{
    glyph : char,
    fg : RGB,
    bg : RGB
}

impl Renderable
{
    fn new(glyph: char,fg : RGB, bg: RGB) -> Renderable
    {
        Renderable
        {
            glyph,
            fg,
            bg
        }
    }
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
        let destination_id = Map::xy_id(position.x+delta_x, position.y+delta_y);
        if state.map.map[destination_id] != TileType::Wall
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
        draw_map(ctx, self.map.map.as_mut_slice());
        render_system(self, ctx);
        
    }
}

fn game_init ( state: &mut State)
{
    let xy = state.map.rooms[0].center();
    state.world.spawn((Position::new(xy.x,xy.y),Renderable::new('@',RGB::from_f32(1., 0., 0.),RGB::from_f32(0., 0., 0.)),Player{}));
}

fn render_system(state:&mut State, ctx: &mut BTerm)
{
    for (_id,(position,graphic)) in
    state.world.query::<(&Position,&Renderable)>().iter()
   {
       ctx.set(position.x, position.y,graphic.fg,graphic.bg,graphic.glyph)
   }

}


fn main() ->BError {
    //println!("Hello, world!");
    let context = BTermBuilder::simple80x50()
    .with_title("Rust-like")
    .build()?;

    let mut gs: State = State{
        world: World::new(),
        map : Map {map :Vec::new(), rooms : Vec::new(),},
        rng : bracket_lib::random::RandomNumberGenerator::new(),
    };
    gs.map = Map::create_room_map(&mut gs);
    gs.map.create_map_corridors();
    game_init(&mut gs);
    main_loop(context,gs)
}
