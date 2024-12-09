use attack_system::AttackSystem;
use bracket_lib::prelude::*;
use bracket_lib::color;
use clear_dead_system::ClearDeadSystem;
use damage_system::DamageSystem;
use hecs::*;
use map_indexing_system::MapIndexingSystem;
use menus::inventory_state;
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
mod clear_dead_system;
mod gui;
mod spawning_system;
mod player;
mod item_pickup_system;
use player::*;
mod menus;
mod item_use_system;
pub mod raws;
use crate::{MAPHEIGHT,MAPWIDTH};
//use map_indexing_system;
#[macro_use]
extern crate lazy_static;


pub struct State
{
    world : World,
    map: Map,
    rng : bracket_lib::random::RandomNumberGenerator,
    current_state: ProgramState,
    player_pos: Point,
    player_ent :Option<Entity>,
}
#[derive(Clone, Copy)]
pub struct Renderable
{
    glyph : char,
    fg : RGB,
    bg : RGB,
    order : i32,
}
#[derive(Clone)]
pub struct Name
{
    name : String,
}
impl Renderable
{
    fn new(glyph: char,fg : RGB, bg: RGB,order : i32) -> Renderable
    {
        Renderable
        {
            glyph,
            fg,
            bg,
            order
        }
    }
}
#[derive(PartialEq,Copy,Clone)]
pub enum ProgramState
{
    Paused,
    ExecutingTurn,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Inventory,
    Targeting { range: i32, item : Entity, }
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



impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm) {

        match self.current_state
        {

            ProgramState::AwaitingInput =>
            {
                ctx.cls();
                self.current_state = player_input_system(ctx, self);
                item_pickup_system::run(self);
                item_use_system::run(self);
                MapIndexingSystem::run(self);
                draw_map(ctx, &self.map);
                render_system(self, ctx);
                gui::draw_ui(self, ctx);
                //gui::draw_inventory(self, ctx);
            }

            ProgramState::PlayerTurn =>
            {
                run_systems(self, ctx);
                self.current_state = ProgramState::MonsterTurn;
            }

            ProgramState::MonsterTurn =>
            {
                run_systems(self, ctx);
                if self.current_state != ProgramState::GameOver
                {
                    self.current_state = ProgramState::AwaitingInput;
                }
            }

            ProgramState::Inventory =>
            {
                ctx.cls();
                //insert inventory input function here!
                let invent_state = menus::InventoryMenu::menu_input(ctx, self);
                match invent_state
                {
                    inventory_state::Cancel => {self.current_state = ProgramState::AwaitingInput;}
                    inventory_state::Selected => {self.current_state = ProgramState::PlayerTurn;}
                    inventory_state::None => {}
                    inventory_state::TargetedItem { item, range } =>
                    {
                        self.current_state = ProgramState::Targeting { range: range, item: item };
                    }
                }
                draw_map(ctx, &self.map);
                render_system(self, ctx);
                gui::draw_ui(self, ctx);
                gui::draw_inventory(self, ctx);
            }
            ProgramState::Targeting { range, item } =>
            {
                ctx.cls();
                draw_map(ctx, &self.map);
                render_system(self, ctx);
                gui::draw_ui(self, ctx);
                let (inv_state,point) = gui::_ranged_target(self, ctx, range);
                match inv_state
                {
                    inventory_state::Cancel =>{ self.current_state = ProgramState::AwaitingInput;}
                    inventory_state::Selected =>
                    {
                        if point.is_some()
                        {
                            let point = point.expect("Couldn't find point even tho it is Some??? like wtf");
                            self.world.insert_one(self.player_ent.expect("Couldn't find player"),
                             WantsToUseItem{item:item, target: Some(point)})
                             .expect("Couldn't insert WantsToUseItem onto player for ranged targeting!");
                            self.current_state = ProgramState::PlayerTurn;

                        }
                    
                    }
                    inventory_state::None => {}

                    _ => {}

                    
                }
            }
            ProgramState::GameOver =>
            {
                ctx.cls();
                ctx.draw_box(20, 10, 40, 20, color::WHITE, color::BLACK);
                ctx.print_color_centered_at(40, 21, color::WHITE, color::BLACK, "You have died!");
                let inp = ctx.key;
                match inp
                {
                    Some(key) =>
                    {
                        match key
                        {
                            bracket_lib::terminal::VirtualKeyCode::Escape => ctx.quit(),
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
            _ =>
            {
                self.current_state = ProgramState::AwaitingInput;
            }
        }
    }
}

fn run_systems(state: &mut State, ctx: &mut BTerm)
{
    ctx.cls();
   
    VisibilitySystem::run(state);
    if state.current_state == ProgramState::MonsterTurn
    {
        MonsterAI::run(state);
    }
    item_use_system::run(state);
    AttackSystem::run(state);
    DamageSystem::run(state);
    ClearDeadSystem::run(state);

    map_indexing_system::MapIndexingSystem::run(state);
    draw_map(ctx, &state.map);
    render_system(state, ctx);
    gui::draw_ui(state, ctx);
    //state.current_state = ProgramState::Paused;
}

fn game_init ( state: &mut State)
{
    
    //Spawn player object
    let xy = state.map.rooms[0].center();
    state.player_pos = xy;
    state.player_ent = Some( state.world.spawn((Position::new(xy.x,xy.y),
    Renderable::new('@',
    RGB::from_f32(1., 0., 0.),
    RGB::from_f32(0., 0., 0.),
    3)
    ,FoV::new(8)
    ,Name{name: "Player".to_string(),}
    , Statistics{max_hp: 40,hp: 40, strength :5, defence : 5}
    ,ItemContainer{items: Vec::new()}
    , Player{})));
    spawning_system::spawn_healing_item(state);
    spawning_system::spawn_damage_item(state);
    let mut i = 1;
    //Spawn test purple goblin enemies in every room apart from the starting room.
    for room in state.map.rooms.iter().skip(1)
    {
    let pos = room.center();
    {
    state.world.spawn((Position::new(pos.x, pos.y),
    Renderable::new('g', RGB::from_f32(1., 0., 1.), RGB::from_f32(0.,0.,0.),3),
    FoV::new(5),Monster{},BlocksTiles {},Statistics{max_hp:12,hp:12,defence: 5,strength: 2},Name{name: format!("Goblin {}",i)}));
    i += 1;
    }
}
}

fn render_system(state:&mut State, ctx: &mut BTerm)
{
    //queries the ECS to get a list of entities to render, collects them into a vec,
    //and then reverse orders them by the order member of the renderable struct
    let mut entities_to_render  = 
    state.world.query_mut::<(&Position,&Renderable)>()
    .into_iter()
    .map(|ent|{(ent.1.0,ent.1.1)})
    .collect::<Vec<_>>();

    //todo test if this puts the lower order first like it should do
    entities_to_render.sort_by_key(|a| a.1.order);

    for ent in entities_to_render
    {
        let idx = Map::xy_id(ent.0.x, ent.0.y);
        if state.map.visible_tiles[idx]
        {
            ctx.set(ent.0.x, ent.0.y, ent.1.fg, ent.1.bg, ent.1.glyph);
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
        map : Map {map :Vec::new(), rooms : Vec::new()
        ,revealed_tiles : vec![false;MAPSIZE]
        ,visible_tiles : vec![false;MAPSIZE]
        ,blocked : vec![false;MAPSIZE]
        ,tile_contents : vec![Vec::new(); MAPSIZE]
        },
        rng : bracket_lib::random::RandomNumberGenerator::new(),
        current_state : ProgramState::PlayerTurn,
        player_pos : Point::zero(),
        player_ent: None,
    };
    // gs.map = Map::create_room_map(&mut gs);
    // gs.map.create_map_corridors();
    Map::generate_map_checked(&mut gs);
    game_init(&mut gs);
    main_loop(context,gs)
}
