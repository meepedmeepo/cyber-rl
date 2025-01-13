use ai::adjacent_ai_system;
use ai::apply_energy_cost;
use ai::run_initiative;
use ai::Energy;
use ai::MyTurn;
use ai::TURN_QUEUE;
use attack_system::AttackSystem;
use bracket_lib::prelude::*;
use bracket_lib::color;
use clear_dead_system::ClearDeadSystem;
use damage_system::DamageSystem;
use effects::run_effect_queue;
use gamelog::GameLog;
use gui::display_input_text;
use gui::get_input_text;
use gui::menu_theme;
use gui::TargettingMode;
use hecs::*;
use hunger::hunger_system;
use hunger::HungerLevel;
use map_indexing_system::MapIndexingSystem;
use menus::inventory_state;
use menus::menu_input;
use menus::select_menu_functions;
use menus::MenuSelections;
use menus::MenuType;
use particles::particle_system;
use particles::ParticleBuilder;
use projectile::projectile_system;
use projectile::ProjectileBuilder;
use ranged_combat::ranged_aim;
use ranged_combat::ranged_aim::TargettingState;
use spawns::spawning_system::EntityType;
use statistics::BaseStatistics;
use statistics::Pools;
use statistics::StatPool;
use time_system::time_system;
use std::cmp::*;
use std::collections::HashMap;
use maps::*;
pub mod maps;
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
mod player;
mod item_pickup_system;
use player::*;
mod menus;
mod item_use_system;
mod item_equip_system;
pub mod raws;
use maps::map::*;
pub mod gamelog;
mod calculate_attribute_system;
mod particles;
mod spawns;
use spawns::*;
use spawns::spawning_system;
mod ranged_combat;
mod projectile;
mod statistics;
mod gui;
mod hunger;
mod time_system;
pub mod effects;
mod entry_trigger_system;
mod prop_trigger_system;
mod ai;
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
    game_log : GameLog,
    particle_builder : ParticleBuilder,
    projectile_builder : ProjectileBuilder,
    target_mode : TargettingMode,
    turn_number : i32,
}


#[derive(PartialEq,Clone)]
pub enum ProgramState
{
    Paused,
    ExecutingTurn,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Inventory,
    Targeting { range: i32, item : Entity, aoe: Option<i32> },
    RangedCombat {range: i32, dmg: i32},
    KeyboardTargetting {cursor_pos : Point},
    Ticking,
    SelectionMenu{items : Vec<(Entity, bool)>, menu : MenuType},
    TextInput {text: Vec<char>},
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

pub fn go_down_stairs(state: &mut State)
{
    cleanup_ECS(state);
    //Map::generate_map_checked(state);
    state.map = simple_map::SimpleMapBuilder::build(state.map.depth+1);
    state.player_pos = state.map.rooms[0].center();
    let roompos = state.map.rooms.last().expect("Room list is empty!").center();
    let idx = Map::xy_id(roompos.x, roompos.y);
    state.map.map[idx] = TileType::DownStairs;

    for (_id,(_player, pos , fov)) 
        in state.world.query_mut::<(&Player,&mut Position, &mut FoV)>()
    {
        pos.x = state.player_pos.x;
        pos.y = state.player_pos.y;

        fov.visible_tiles.clear();
        fov.dirty = true;
    }

    spawning_system::room_spawns(state);

    let msg = format!("You traversed the stairs downwards to the next layer of the dungeon");

    console::log(msg.clone());

    state.game_log.add_log(msg);

    state.current_state = ProgramState::Ticking;

}

#[allow(non_snake_case)]
fn cleanup_ECS(state: &mut State)
{
    let mut entities_to_delete = Vec::new();
    for entity in state.world.iter()
    {
        let mut should_delete = true;

        if entity.entity() == state.player_ent.expect("Couldn't find player entity in cleanup ECS!")
        {
            should_delete = false;
        }

        if entity.get::<&InContainer>()
        .is_some_and(|cont| 
            cont.owner == state.player_ent.expect("Couldn't find player entity in cleanup ECS!"))
        {
            should_delete = false;
        }

        if entity.get::<&Equipped>()
        .is_some_and(|eq| 
            eq.owner == state.player_ent.expect("Couldn't find player entity in cleanup ECS!"))
        {
            should_delete = false;
        }


        if should_delete
        {
            entities_to_delete.push(entity.entity());
        }
    }

    for entity in entities_to_delete
    {
        state.world.despawn(entity)
        .expect("Can't delete entity that has been marked for removal when cleaning up ECS!");
    }
}

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm)
    {

        match self.current_state.clone()
        {

            ProgramState::AwaitingInput =>
            {
                ctx.cls();
                self.current_state = player_input_system(ctx, self);
                item_pickup_system::run(self);
                item_use_system::run(self);
                MapIndexingSystem::run(self);
                ClearDeadSystem::run(self);
                draw_map(ctx, &self.map);
                render_system(self, ctx);
                gui::draw_ui(self, ctx);
                gui::draw_status_box(self, ctx);
                gui::draw_gamelog(self, ctx);
                //gui::draw_inventory(self, ctx);
            }
            ProgramState::Ticking =>
            {
                VisibilitySystem::run(self);
                self.current_state = run_initiative(self);
                if self.current_state == ProgramState::AwaitingInput
                {
                    return;
                }
                else
                {
                    //makes sure all the visual information for the ai is up to date!
                    

                    //todo: in all of the systems that can end a turn apply the energy costs to the entities!
                    //check adjacent reactions
                    ai::adjacent_ai_system(self);

                    //check further away reactions
                    ai::visible_ai_system(self);

                    //run current goal behaviour
                    ai::approach_ai_system(self);
                    ai::flee_ai_system(self);
                    //default behaviour
                    ai::default_move_ai_system(self);
                    //run systems!
                    run_systems(self, ctx);
                }
            }

            ProgramState::PlayerTurn =>
            {  
                hunger_system(self);
                run_systems(self, ctx);
                time_system(self);

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

            ProgramState::TextInput {mut text } =>
            {
                ctx.cls();
                get_input_text(self, ctx, &mut text);
                display_input_text(self, ctx, &text, 5, 15);
                self.current_state = ProgramState::TextInput { text, }
            }

            ProgramState::Inventory =>
            {
                ctx.cls();
                //insert inventory input function here!
                let invent_state = menus::InventoryMenu::menu_input(ctx, self);
                match invent_state
                {
                    inventory_state::Cancel => {self.current_state = ProgramState::AwaitingInput;}
                    inventory_state::Selected => 
                    {
                            apply_energy_cost(self, ai::ActionType::Equip, self.player_ent.unwrap());
                            let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                            self.current_state = ProgramState::Ticking;
                            return;
                    }
                    inventory_state::None => {}
                    inventory_state::TargetedItem { item, range } =>
                    {
                        let query = self.world.get::<&AoE>(item);
                        let mut aoe = None;
                        match query
                        {
                            Ok(ref aoe_comp) =>
                            {
                                aoe = Some(aoe_comp.radius);
                            }
                            Err(_) => {}
                        }
                        self.current_state = ProgramState::Targeting { range: range, item: item, aoe : aoe };
                    }
                }

                draw_map(ctx, &self.map);
                render_system(self, ctx);
                gui::draw_ui(self, ctx);
                gui::draw_status_box(self, ctx);
                gui::draw_gamelog(self, ctx);
                gui::draw_inventory(self, ctx);
            }

            ProgramState::SelectionMenu { mut items, menu  } =>
            {
                ctx.cls();
                draw_map(ctx, &self.map);
                render_system(self, ctx);
                gui::draw_ui(self, ctx);
                gui::draw_status_box(self, ctx);
                gui::draw_gamelog(self, ctx);

                //let (mut input, mut draw) = 
                    //select_menu_functions(menu);

                //input(self, ctx, &mut items)

                match menu_input(self, ctx, &mut items)
                {
                    MenuSelections::Cancel => {self.current_state = ProgramState::AwaitingInput; return;}
                    MenuSelections::NoInput => {}
                    MenuSelections::ToggleSelected => {self.current_state = ProgramState::SelectionMenu { items: items.clone(), menu };}
                    MenuSelections::Execute =>
                    {
                        match menu
                        {
                        //TODO: change this to actually use the item pickup system like normal oof
                            MenuType::PickupItem =>
                            {
                                for (item, is_selected) in items.iter()
                                {
                                    if *is_selected
                                    {
                                        self.world.insert_one(*item, InContainer{owner: self.player_ent.unwrap()}).unwrap();
                                        
                                        self.world.remove_one::<Position>(*item).unwrap();
                                    }
                                }
                            }

                            MenuType::DropItem =>
                            {
                                let pos = self.player_pos;
                                for (item, is_selected) in items.iter()
                                {
                                    if *is_selected
                                    {
                                        self.world.insert_one(*item, Position{x: pos.x, y: pos.y}).unwrap();

                                        self.world.remove_one::<InContainer>(*item).unwrap();
                                    }
                                }
                                apply_energy_cost(self, ai::ActionType::Equip, self.player_ent.unwrap());
                                let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                                self.current_state = ProgramState::Ticking;
                                return;
                            }

                            MenuType::UnequipItem =>
                            {
                                let ent = self.player_ent.unwrap();

                                for (item, is_selected) in items.iter()
                                {
                                    if *is_selected
                                    {
                                        self.world.insert_one(*item, InContainer{owner: ent}).unwrap();

                                        self.world.remove_one::<Equipped>(*item).unwrap();

                                        self.world.insert_one(ent, EquipmentDirty{}).unwrap();
                                    }
                                }
                                apply_energy_cost(self, ai::ActionType::Equip, self.player_ent.unwrap());
                                let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                                self.current_state = ProgramState::Ticking;
                                return;
                            }


                            _ => {}
                        }

                        self.current_state = ProgramState::AwaitingInput;
                    }
                }

                let(title, text_colour, highlight) = menu_theme(menu);

                gui::draw_menu_custom(ctx, &items, title, text_colour, highlight, self);
                //gui::draw_pickup_menu(ctx, items, self);
                //draw(ctx,items,self);

            }

            ProgramState::Targeting { range, item, aoe } =>
            {
                ctx.cls();
                draw_map(ctx, &self.map);
                render_system(self, ctx);
                gui::draw_ui(self, ctx);
                gui::draw_status_box(self, ctx);
                gui::draw_gamelog(self, ctx);
                let (inv_state,point) = gui::ranged_target(self, ctx, range, aoe);
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
                            apply_energy_cost(self, ai::ActionType::Attack, self.player_ent.unwrap());
                            let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                            self.current_state = ProgramState::Ticking;
                        }
                    }
                    inventory_state::None => {}

                    _ => {}
                }
            }

            ProgramState::RangedCombat { range , dmg } =>
            {
                ctx.cls();
                draw_map(ctx, &self.map);
                render_system(self, ctx);
                gui::draw_ui(self, ctx);
                gui::draw_status_box(self, ctx);
                gui::draw_gamelog(self, ctx);

                let target_state = ranged_aim::aim_projectile(self, ctx, self.player_pos, range);

                match target_state
                {
                    TargettingState::None => {}
                    TargettingState::Cancel => {self.current_state = ProgramState::AwaitingInput;}

                    TargettingState::Selected { mut path, end  } =>
                    {
                        path.push(end);
                        let dmg = 4;
                        //TODO:!!!!!!!!!!!!!!
                        
                        self.projectile_builder.add_request(10., path.into_iter().skip(1).collect::<Vec<_>>(), projectile::ProjectileType::Missile,
                            '/', RGB::named(WHITE), RGB::named(BLACK), 5,dmg );
                        let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                        self.current_state = ProgramState::Ticking;
                    }
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
    item_equip_system::run(state);
    item_use_system::run(state);
    calculate_attribute_system::run(state);

    entry_trigger_system::run(state);
    prop_trigger_system::run(state);

    AttackSystem::run(state);
    DamageSystem::run(state);

    effects::run_effect_queue(state);
    ClearDeadSystem::run(state);

    map_indexing_system::MapIndexingSystem::run(state);

    
    
    state.target_mode = TargettingMode::Keyboard { cursor_pos: state.player_pos };

    draw_map(ctx, &state.map);
    render_system(state, ctx);
    gui::draw_ui(state, ctx);
    gui::draw_status_box(state, ctx);
    gui::draw_gamelog(state, ctx);
    //state.current_state = ProgramState::Paused;
}

fn game_init ( state: &mut State)
{
    raws::run();
    
    state.map = simple_map::SimpleMapBuilder::build(0);
    //let item = raws::RawMaster::spawn_named_item(raws::RAWS.lock().unwrap()., new_entity, key, pos)
    //Spawn player object
    let xy = state.map.rooms[0].center();
    
    state.player_pos = xy;
    state.player_ent = Some( state.world.spawn((Position::new(xy.x,xy.y),
    Renderable::new('@',
    RGB::named(LIME_GREEN),
    RGB::from_f32(0., 0., 0.),
    3)
    ,FoV::new(8)
    ,Name{name: "Player".to_string(),}
    , Pools{hitpoints: StatPool::new(50), exp: statistics::calculate_xp_from_level(1),level: 1, armour_class: Attribute::new(10)
        , hit_die: DiceType::new(1, 10, 1)}
    , BaseStatistics::roll_stats(3)
    , HungerLevel{nutrition: StatPool::new(300)}
    , Energy{value: 100}
    ,Faction{name: "Player".to_string()}
    , Player{})));

    spawning_system::spawn_item_in_backpack(state, &"Ration".to_string(), state.player_ent.unwrap());
    spawning_system::room_spawns(state);

    spawning_system::spawn_healing_item(state);
    spawning_system::spawn_damage_item(state);
    let mut i = 1;
    let pos2 = state.map.rooms[0].center();

    //spawning_system::spawn_entity(state, &(&0, &"Health Potion".to_string()), xy.x, xy.y+2, EntityType::Item);
    //spawning_system::spawn_entity(state, &(&1, &"Wooden Bow".to_string()), xy.x-1, xy.y,EntityType::Item);

// }
}

fn render_system(state:&mut State, ctx: &mut BTerm)
{
    //queries the ECS to get a list of entities to render, collects them into a vec,
    //and then reverse orders them by the order member of the renderable struct

    //runs spawns particles from builder requests and cleans up dead particles before rendering
    //entities/
    particle_system::spawn_system(state);
    particle_system::update(state, ctx);
    projectile_system::spawn_projectiles(state);
    projectile_system::update_projectiles(state, ctx);

    let mut entities_to_render  = 
        state.world.query_mut::<(&Position,&Renderable)>().without::<&Hidden>()
        .into_iter()
        .map(|ent|{(ent.1.0,ent.1.1)})
        .collect::<Vec<_>>();

    
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

fn main() ->BError 
{
    //println!("{}", std::env::current_dir().unwrap().display());
    //println!("Hello, world!");
    let mut context = BTermBuilder::new()
    .with_dimensions(110, 45)
    .with_resource_path("resources/")
    .with_font("dbyte_2x.png", 12 , 16)
    .with_tile_dimensions(12, 16)
    .with_simple_console(110, 45, "dbyte_2x.png")
    .with_title("Rust-like")
    .with_fps_cap(60.)
    .with_advanced_input(true)
    .build()?;

    let mut gs: State = State{
        world: World::new(),
        map : Map {map :Vec::new(), rooms : Vec::new()
        ,revealed_tiles : vec![false;MAPSIZE]
        ,visible_tiles : vec![false;MAPSIZE]
        ,blocked : vec![false;MAPSIZE]
        ,tile_contents : vec![Vec::new(); MAPSIZE], depth: 0, props: HashMap::new()},
        rng : bracket_lib::random::RandomNumberGenerator::new(),
        current_state : ProgramState::TextInput { text: Vec::new() },
        player_pos : Point::zero(),
        player_ent: None,
        game_log : GameLog::new(),
        particle_builder : ParticleBuilder::new(),
        projectile_builder : ProjectileBuilder::new(),
        target_mode: TargettingMode::Keyboard{cursor_pos: Point::zero()},
        turn_number: 0,
    };
    
    //context.with_post_scanlines(true);

    context.post_screenburn = true;
    
    game_init(&mut gs);
    main_loop(context,gs)
}
