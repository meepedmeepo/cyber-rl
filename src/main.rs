use attack_system::AttackSystem;
use bracket_lib::prelude::*;
use damage_system::DamageSystem;
use hecs::*;
use std::cmp::*;
use map::*;
mod map;
mod components;
use components::*;
mod visibility_system;
use visibility_system::*;
mod monster_ai_system;
use monster_ai_system::*;
mod map_indexing_system;
mod attack_system;
mod damage_system;
//use map_indexing_system;

pub struct State
{
    world : World,
    map: Map,
    rng : bracket_lib::random::RandomNumberGenerator,
    current_state: ProgramState,
    player_pos: Point,
}

pub struct Renderable
{
    glyph : char,
    fg : RGB,
    bg : RGB
}
pub struct Name
{
    name : String,
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
#[derive(PartialEq,Copy,Clone)]
pub enum ProgramState
{
    Paused,
    ExecutingTurn,
}


pub struct Position
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

fn player_input_system(ctx:&BTerm, state: &mut State) -> ProgramState
{
    match ctx.key
    {
        None => {return ProgramState::Paused;},
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
            _ =>{return ProgramState::Paused;},

        }

    }
    ProgramState::ExecutingTurn
}
/// TODO: cleanup this absolute fucking mess holy shit wtf
fn try_move(state: &mut State,delta_x:i32,delta_y:i32)
{
    let mut moved =  false;
    let mut destination_id : usize = 0;
    let (id,(_player)) =  state.world.query_mut::<(&Player)>().into_iter().next().expect("No Player found!");
    let mut attacker : Entity = id;
    let mut target = id;

    for(_id,(_player,position,fov)) in state.world.query_mut::<(&Player,&mut Position,&mut FoV)>()
    {
        destination_id = Map::xy_id(position.x+delta_x, position.y+delta_y);
        if !state.map.blocked[destination_id]
        {
        position.x = min(79,max(0,position.x+delta_x));
        position.y = min(49,max(0,position.y+delta_y));
        state.player_pos = Point::new(position.x, position.y);
        fov.dirty = true;
        moved = true;
        attacker = _id;
        break;
        }
        
    }
        if state.map.tile_contents[destination_id].len() > 0 && !moved
        {
            
            let mut found_target = false;
            for potential_target in state.map.tile_contents[destination_id].iter()
            {
                // for (entity,(_stats,name,_pos)) in 
                // state.world.query::<(&Statistics,&Name,&Position)>().
                // {
                //     target = entity;
                //     console::log(&format!("I will stab thee now, {}!",name.name));
                // }
                let query = state.world.query_one_mut::<(&Statistics,
                &Name)>(*potential_target);
                match query
                {
                    Ok(res) =>
                    {
                        console::log(&format!("I will stab thee now, {}!",res.1.name));
                        target = *potential_target;
                        found_target = true;
                    }
                    Err(_) =>{return;}
                }
            }
            if found_target
            {
                //console::log(format!("Target found! {}",state.world.get::<&Name>(target).expect("No target name found!").name));
                AttackSystem::add_attack(attacker, target, state);
            }

        
        }

}

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm) {
        if self.current_state == ProgramState::ExecutingTurn
        {
            run_systems(self, ctx);
        }
        else
        {
            ctx.cls();
            self.current_state = player_input_system(ctx, self);
            draw_map(ctx, &self.map);
            render_system(self, ctx);
        }
    }
}

fn run_systems(state: &mut State, ctx: &mut BTerm)
{
    ctx.cls();
   
    VisibilitySystem::run(state);
    MonsterAI::run(state);

    AttackSystem::run(state);
    DamageSystem::run(state);

    map_indexing_system::MapIndexingSystem::run(state);
    draw_map(ctx, &state.map);
    render_system(state, ctx);
    state.current_state = ProgramState::Paused;
}

fn game_init ( state: &mut State)
{
    //Spawn player object
    let xy = state.map.rooms[0].center();
    state.player_pos = xy;
    state.world.spawn((Position::new(xy.x,xy.y),
    Renderable::new('@',
    RGB::from_f32(1., 0., 0.),
    RGB::from_f32(0., 0., 0.))
    ,FoV::new(8)
    ,Name{name: "Player".to_string(),}
    , Statistics{max_hp: 40,hp: 40, strength :5, defence : 5}
    , Player{}));
    let mut i = 1;
    //Spawn test purple goblin enemies in every room apart from the starting room.
    for room in state.map.rooms.iter().skip(1)
    {
    let pos = room.center();
    {
    state.world.spawn((Position::new(pos.x, pos.y),
    Renderable::new('g', RGB::from_f32(1., 0., 1.), RGB::from_f32(0.,0.,0.)),
    FoV::new(5),Monster{},BlocksTiles {},Statistics{max_hp:12,hp:12,defence: 5,strength: 2},Name{name: format!("Goblin {}",i)}));
    i += 1;
    }
}
}

fn render_system(state:&mut State, ctx: &mut BTerm)
{
    for (_id,(position,graphic)) in
    state.world.query::<(&Position,&Renderable)>().iter()
   {
        let idx = Map::xy_id(position.x, position.y);
        if state.map.visible_tiles[idx]
        {
            ctx.set(position.x, position.y,graphic.fg,graphic.bg,graphic.glyph)
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
        map : Map {map :Vec::new(), rooms : Vec::new(),width:80,height:50
        ,revealed_tiles : vec![false;80*50]
        ,visible_tiles : vec![false;80*50]
        ,blocked : vec![false;80*50]
        ,tile_contents : vec![Vec::new(); 80*50]
        },
        rng : bracket_lib::random::RandomNumberGenerator::new(),
        current_state : ProgramState::ExecutingTurn,
        player_pos : Point::zero(),
    };
    // gs.map = Map::create_room_map(&mut gs);
    // gs.map.create_map_corridors();
    Map::generate_map_checked(&mut gs);
    game_init(&mut gs);
    main_loop(context,gs)
}
