use attack_system::AttackSystem;
use bracket_lib::prelude::*;
use bracket_lib::color;
use clear_dead_system::ClearDeadSystem;
use damage_system::DamageSystem;
use gamelog::GameLog;
use hecs::*;
use map_indexing_system::MapIndexingSystem;
use menus::inventory_state;
use particles::particle_system;
use particles::ParticleBuilder;
use projectile::projectile_system;
use projectile::ProjectileBuilder;
use ranged_combat::ranged_aim;
use ranged_combat::ranged_aim::TargettingState;
use spawns::spawning_system::EntityType;
use std::cmp::*;
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
mod gui;
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
    projectile_builder : ProjectileBuilder
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
    Targeting { range: i32, item : Entity, aoe: Option<i32> },
    RangedCombat {range: i32, dmg: i32}
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

    state.current_state = ProgramState::PlayerTurn;

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
                gui::draw_gamelog(self, ctx);
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
                gui::draw_gamelog(self, ctx);
                gui::draw_inventory(self, ctx);
            }

            ProgramState::Targeting { range, item, aoe } =>
            {
                ctx.cls();
                draw_map(ctx, &self.map);
                render_system(self, ctx);
                gui::draw_ui(self, ctx);
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
                            self.current_state = ProgramState::PlayerTurn;
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
                gui::draw_gamelog(self, ctx);

                let target_state = ranged_aim::aim_projectile(self, ctx, self.player_pos, range);

                match target_state
                {
                    TargettingState::None => {}
                    TargettingState::Cancel => {self.current_state = ProgramState::AwaitingInput;}

                    TargettingState::Selected { mut path, end  } =>
                    {
                        //only handles the case that the missile targets one entity
                        //let target_point = path.last().expect("Path must have been empty as couldn't find last point!");
                        path.push(end);
                        let dmg = 4;

                        
                        //TODO:!!!!!!!!!!!!!!

                        //self.world.get::<&CombatStats>(self.player_ent.unwrap()).unwrap().power.total;
                        
                        self.projectile_builder.add_request(30., path.into_iter().skip(1).collect::<Vec<_>>(), projectile::ProjectileType::Missile,
                            '/', RGB::named(WHITE), RGB::named(BLACK), 5,dmg );

                        // let target_entities = self.map.get_mob_entities_at_position(self, end);

                        // for ent in target_entities.iter()
                        // {
                        //     AttackSystem::add_attack(self.player_ent
                        //         .expect("Couldn't find player to use their stats for targetting ranged attacks")
                        //         , *ent, self);
                        // }

                        self.current_state = ProgramState::PlayerTurn;
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

    AttackSystem::run(state);
    DamageSystem::run(state);
    ClearDeadSystem::run(state);

    map_indexing_system::MapIndexingSystem::run(state);
    draw_map(ctx, &state.map);
    render_system(state, ctx);
    gui::draw_ui(state, ctx);
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
    RGB::from_f32(1., 0., 0.),
    RGB::from_f32(0., 0., 0.),
    3)
    ,FoV::new(8)
    ,Name{name: "Player".to_string(),}
    , Statistics{max_hp: 80,hp: 80, strength :9, defence : 5},
    CombatStats::new(9, 0)

    , Player{})));

    state.world.spawn((Position::new(xy.x-2, xy.y), Renderable
    {glyph : ']',fg: RGB::named(WHITE), bg: RGB::named(BLACK), order: 2}
    , Name{name: "helmet cringe".to_string()},
    Item{}, Equippable{slot: EquipmentSlot::Head,power_bonus: 0, defence_bonus: 2}
    ));
    state.world.spawn((Position::new(xy.x-2, xy.y+1), Renderable
    {glyph : ']',fg: RGB::named(BLUE), bg: RGB::named(BLACK), order: 2}
    , Name{name: "helmet cringest".to_string()},
    Item{}, Equippable{slot: EquipmentSlot::Head,power_bonus: 0, defence_bonus: 8}
    ));
    spawning_system::room_spawns(state);

    spawning_system::spawn_healing_item(state);
    spawning_system::spawn_damage_item(state);
    let mut i = 1;
    let pos2 = state.map.rooms[0].center();

    spawning_system::spawn_entity(state, &(&0, &"Health Potion".to_string()), xy.x, xy.y+2, EntityType::Item);
    spawning_system::spawn_entity(state, &(&1, &"Wooden Bow".to_string()), xy.x-1, xy.y,EntityType::Item);

// }
}

fn render_system(state:&mut State, ctx: &mut BTerm)
{
    //queries the ECS to get a list of entities to render, collects them into a vec,
    //and then reverse orders them by the order member of the renderable struct

    //runs spawns particles from builder requests and cleans up dead particles before rendering
    //entities
    particle_system::spawn_system(state);
    particle_system::update(state, ctx);
    projectile_system::spawn_projectiles(state);
    projectile_system::update_projectiles(state, ctx);

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


fn main() ->BError 
{
    //println!("Hello, world!");
    let mut context = BTermBuilder::simple(110,50)?
    .with_title("Rust-like")
    .with_fps_cap(60.)
    .build()?;

    let mut gs: State = State{
        world: World::new(),
        map : Map {map :Vec::new(), rooms : Vec::new()
        ,revealed_tiles : vec![false;MAPSIZE]
        ,visible_tiles : vec![false;MAPSIZE]
        ,blocked : vec![false;MAPSIZE]
        ,tile_contents : vec![Vec::new(); MAPSIZE], depth: 0
        },
        rng : bracket_lib::random::RandomNumberGenerator::new(),
        current_state : ProgramState::PlayerTurn,
        player_pos : Point::zero(),
        player_ent: None,
        game_log : GameLog::new(),
        particle_builder : ParticleBuilder::new(),
        projectile_builder : ProjectileBuilder::new()
    };
    // gs.map = Map::create_room_map(&mut gs);
    // gs.map.create_map_corridors();
    //Map::generate_map_checked(&mut gs);
    context.with_post_scanlines(true);
    
    game_init(&mut gs);
    main_loop(context,gs)
}
