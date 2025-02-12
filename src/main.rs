use ai::adjacent_ai_system;
use ai::apply_energy_cost;
use ai::run_initiative;
use ai::Energy;
use ai::MyTurn;
use attack_system::AttackSystem;
use bracket_lib::prelude::*;
use bracket_lib::color;
use clear_dead_system::ClearDeadSystem;
use damage_system::DamageSystem;
use effects::add_effect;
use effects::run_effect_queue;
use gamelog::GameLog;
use gui::draw_cursor;
use gui::keyboard_cursor;
use gui::menu_theme;
use gui::TargettingMode;
use hecs::*;
use hunger::hunger_system;
use hunger::HungerLevel;
use macroquad::color::GRAY;
use macroquad::color::RED;
use macroquad::miniquad::window::quit;
use macroquad::miniquad::RenderingBackend;
use macroquad_text::Fonts;
use map_indexing_system::MapIndexingSystem;
use menus::inventory_state;
use menus::menu_input;
use menus::select_menu_functions;
use menus::MenuSelections;
use menus::MenuType;
use networks::ControlNode;
use networks::NetworkMap;
use networks::NodeOwned;
use new_egui_macroquad::egui::Align2;
use new_egui_macroquad::egui::Context;
use new_egui_macroquad::egui::FontData;
use new_egui_macroquad::egui::FontDefinitions;
use particles::particle_system;
use particles::ParticleBuilder;
use projectile::projectile_system;
use projectile::ProjectileBuilder;
use ranged_combat::ranged_aim;
use ranged_combat::ranged_aim::TargettingState;
use renderer::draw_tiles;
use renderer::CharSize;
use renderer::GraphicGrid;
use renderer::Renderer;
use screen_manager::MANAGER;
use spawns::spawning_system::EntityType;
use statistics::BaseStatistics;
use statistics::Pools;
use statistics::StatPool;
use time_system::time_system;
use std::cmp::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;
use maps::*;
pub mod maps;
mod components;
use components::*;
mod visibility_system;
use visibility_system::*;
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
mod networks;
pub mod camera;
pub mod renderer;
use macroquad::prelude::*;
use new_egui_macroquad as em;
use new_egui_macroquad::egui as egui;

mod screen_manager;
//use map_indexing_system;
#[macro_use]
extern crate lazy_static;



const NOTO_SANS_SYMBOLS : &[u8] = include_bytes!("../assets/fonts/NotoSansSymbols.ttf");
const JULIA : &[u8] = include_bytes!("../assets/fonts/JuliaMono-Bold.ttf");
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
    target_mode : TargettingMode,
    turn_number : i32,
    network_map : NetworkMap,
    renderer : Renderer
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
    PlayAnimation,
    AwaitingMenu {response: Option<Vec<Entity>>, menu_type : screen_manager::MenuType},
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position
{
    x: i32,
    y : i32,
}


impl From<Point> for Position
{
    fn from(item: Point) -> Self
    {
        Position{x: item.x, y: item.y}
    }
}

impl Into<Point> for Position
{
    fn into(self) -> Point
    {
        Point{x: self.x, y: self.y}
    }
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
    state.generate_world_map(state.map.depth + 1);

    for (_id,(_player, pos , fov)) 
        in state.world.query_mut::<(&Player,&mut Position, &mut FoV)>()
    {
        pos.x = state.player_pos.x;
        pos.y = state.player_pos.y;

        fov.visible_tiles.clear();
        fov.dirty = true;
    }

    let msg = format!("You traversed the stairs downwards to the next layer of the dungeon");

    console::log(msg.clone());

    state.game_log.add_log(msg);
    state.world.query_one_mut::<&mut Energy>(state.player_ent.unwrap()).unwrap().value = 100;

    state.current_state = ProgramState::Ticking;

}

#[allow(non_snake_case)]
fn cleanup_ECS(state: &mut State)
{
    let mut turns = Vec::new();
    for (ent, _turn)  in state.world.query_mut::<&MyTurn>()
    {
        turns.push(ent);
    }
    for turn in turns.iter()
    {
        let _ = state.world.remove_one::<MyTurn>(*turn);
    }
    
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

impl State
{
    fn generate_world_map(&mut self, new_depth : i32)
    {
        let mut builder = maps::level_generator(new_depth);

        builder.build_map(&mut self.rng);

        self.map = builder.build_data.map.clone();
        self.player_pos = builder.build_data.starting_position.unwrap().clone();

        if new_depth != 0
        {
            let _ = self.world.insert_one(self.player_ent.unwrap(), Position{x: self.player_pos.x, y: self.player_pos.y});
        }

        builder.spawn_entities(self);

    }
}

impl State{
    fn tick(&mut self)
    {

        match self.current_state.clone()
        {

            ProgramState::AwaitingInput =>
            {
                self.current_state = player_input_system( self);
                item_pickup_system::run(self);
                item_use_system::run(self);
                MapIndexingSystem::run(self);
                ClearDeadSystem::run(self);
                camera::render_camera(self);
                ////render_system(self, ctx);
                gui::draw_ui(self);
                gui::draw_status_box(self);
                gui::draw_gamelog(self);
                //gui::draw_inventory(self, ctx);
            }

            ProgramState::PlayAnimation =>
            {
                camera::render_camera(self);
                effects::run_effect_queue(self);
                effects::run_animation_queue(self);
                //render_system(self, ctx);
                gui::draw_ui(self);
                gui::draw_status_box(self);
                gui::draw_gamelog(self);
                
            }
            ProgramState::Ticking =>
            {
                let mut newrunstate = ProgramState::Ticking;
                while newrunstate == ProgramState::Ticking
                {
                    VisibilitySystem::run(self);
                    newrunstate = run_initiative(self);

                    {
                        ai::quip_system(self);
                        
                        //todo: in all of the systems that can end a turn apply the energy costs to the entities!
                        //check adjacent reactions
                        ai::adjacent_ai_system(self);

                        //check further away reactions
                        ai::visible_ai_system(self);

                        //run current goal behaviour
                        ai::approach_ai_system(self);

                        ai::flee_ai_system(self);

                        //idle movement
                        ai::idle_movement_ai(self);

                        //default behaviour
                        ai::default_move_ai_system(self);
                        //run systems!
                        run_systems(self);

                        
                    }
                }
                self.current_state = newrunstate;
            }


            ProgramState::TextInput {mut text } =>
            {

                //get_input_text(self, &mut text);
                //display_input_text(self,  &text, 5, 15);
                self.current_state = ProgramState::TextInput { text, }
            }

            ProgramState::Inventory =>
            {
                let (min_x,_max_x,min_y, _max_y) = camera::get_screen_bounds(self);

                //insert inventory input function here!
                let invent_state = menus::InventoryMenu::menu_input( self);
                match invent_state
                {
                    inventory_state::Cancel => {self.current_state = ProgramState::AwaitingInput;}
                    inventory_state::Selected => 
                    {
                            apply_energy_cost(self, ai::ActionType::Equip, self.player_ent.unwrap());
                            let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                            self.current_state = ProgramState::Ticking;
                            //return;
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
                        let mut screen_pos = self.player_pos;
                        
                        screen_pos.x -= min_x;
                        screen_pos.y -= min_y;
                        self.target_mode = TargettingMode::Keyboard { cursor_pos: screen_pos };
                        self.current_state = ProgramState::Targeting { range: range, item: item, aoe : aoe };
                    }
                }

                camera::render_camera(self);
                //render_system(self, ctx);
                gui::draw_ui(self);
                gui::draw_status_box(self);
                gui::draw_gamelog(self);
                gui::draw_inventory(self);
            }
            ProgramState::SelectionMenu { mut items, menu  } =>
            {

                camera::render_camera(self);
                //render_system(self, ctx);
                gui::draw_ui(self);
                gui::draw_status_box(self);
                gui::draw_gamelog(self);

                //let (mut input, mut draw) = 
                    //select_menu_functions(menu);

                //input(self, ctx, &mut items)

                match menu_input(self, &mut items)
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

                gui::draw_menu_custom( &items, title, text_colour, highlight, self);
                //gui::draw_pickup_menu(ctx, items, self);
                //draw(ctx,items,self);

            }

            ProgramState::Targeting { range, item, aoe } =>
            {

                camera::render_camera(self);
                //render_system(self, ctx);
                gui::draw_ui(self);
                gui::draw_status_box(self);
                gui::draw_gamelog(self);

                if let TargettingMode::Keyboard { cursor_pos } = self.target_mode
                {
                    gui::draw_tooltip(self, cursor_pos );
                }
                let (inv_state,point) = gui::ranged_target(self, range, aoe);
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
                            apply_energy_cost(self, ai::ActionType::UseItem, self.player_ent.unwrap());
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
                camera::render_camera(self);
                gui::draw_ui(self);
                gui::draw_status_box(self);
                gui::draw_gamelog(self);

                
                
                let target_state = ranged_aim::aim_projectile(self, self.player_pos, range);

                match target_state
                {
                    TargettingState::None => {self.current_state = ProgramState::RangedCombat { range: range, dmg: dmg }}
                    TargettingState::Cancel => {self.current_state = ProgramState::AwaitingInput;}

                    TargettingState::Selected { mut path, end  } =>
                    {
                        path.push(end);
                        let dmg = 4;
                        //TODO:!!!!!!!!!!!!!!
                        
                        let query = self.world.query::<&Equipped>()
                            .iter()
                            .filter(|(_ent,equip) | 
                            equip.slot == EquipmentSlot::Ranged && equip.owner == self.player_ent
                            .expect("Couldn't find player entity to fetch ranged stats for combat"))
                            .map(|(ent, _eq)| ent)
                            .collect::<Vec<_>>();

                        
                        
                        //effects::ra
                        add_effect(Some(self.player_ent.unwrap()), effects::EffectType::RangedFire { item: query[0] }, 
                            effects::Targets::Tile { tile_idx: self.map.xy_idx(end.x, end.y) as i32 });

                        effects::run_effect_queue(self);
                        effects::run_animation_queue(self);

                        //self.projectile_builder.add_request(10., path.into_iter().skip(1).collect::<Vec<_>>(), projectile::ProjectileType::Missile,
                        //    '/', RGB::named(WHITE), RGB::named(BLACK), 5,dmg );
                        let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                        self.current_state = ProgramState::PlayAnimation;
                    }
                }

                if let TargettingMode::Keyboard { cursor_pos } = self.target_mode
                {
                    gui::draw_tooltip(self, cursor_pos );
                }

            }

            ProgramState::KeyboardTargetting { cursor_pos } =>
            {
                camera::render_camera(self);
                ////render_system(self, ctx);
                gui::draw_ui(self);
                gui::draw_status_box(self);
                gui::draw_gamelog(self);

                self.current_state = ProgramState::KeyboardTargetting{cursor_pos: gui::draw_cursor(cursor_pos, self, LIGHT_GREEN)};

                gui::draw_tooltip(self, cursor_pos);

                if INPUT.lock().is_key_pressed(VirtualKeyCode::Escape)
                {
                    self.current_state = ProgramState::AwaitingInput;
                }
            }

            ProgramState::AwaitingMenu { response, menu_type } =>
            {
                camera::render_camera(self);

                if response.is_some()
                {
                    match menu_type
                    {
                        screen_manager::MenuType::Pickup =>
                        {
                            for item in response.unwrap().iter()
                            {
                                self.world.insert_one(*item, InContainer{owner: self.player_ent.unwrap()}).unwrap();
                                        
                                self.world.remove_one::<Position>(*item).unwrap();
                            }

                            self.current_state = ProgramState::AwaitingInput;
                        }

                        screen_manager::MenuType::Unequip =>
                        {
                            let ent = self.player_ent.unwrap();

                            for item in response.unwrap().iter()
                            {
                                self.world.insert_one(*item, InContainer{owner: ent}).unwrap();

                                self.world.remove_one::<Equipped>(*item).unwrap();

                                self.world.insert_one(ent, EquipmentDirty{}).unwrap();
                            }

                            apply_energy_cost(self, ai::ActionType::Equip, self.player_ent.unwrap());
                            let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                            self.current_state = ProgramState::Ticking;
                        }

                        screen_manager::MenuType::Drop =>
                        {
                            let posref = self.world.get::<&Position>(self.player_ent.unwrap()).unwrap();
                            let pos = Point::new(posref.x, posref.y);

                            std::mem::drop(posref);
                            for item in response.unwrap().iter()
                            {
                                self.world.insert_one(*item, Position{x: pos.x, y: pos.y}).unwrap();

                                self.world.remove_one::<InContainer>(*item).unwrap();
                            }

                            apply_energy_cost(self, ai::ActionType::Pickup , self.player_ent.unwrap());

                            let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                            self.current_state = ProgramState::Ticking;
                        }
                        screen_manager::MenuType::Inventory =>
                        {
                            let item = response.unwrap()[0];

                            let (min_x, max_x, min_y, max_y) = camera::get_screen_bounds(self);

                            if let Ok(ranged) = self.world.get::<&RangedTargetting>(item)
                            {
                                let range = ranged.range;
                                std::mem::drop(ranged);

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
                                std::mem::drop(query);
                                let mut screen_pos = self.player_pos;

                                screen_pos.x -= min_x;
                                screen_pos.y -= min_y;
                                self.target_mode = TargettingMode::Keyboard { cursor_pos: screen_pos };
                                self.current_state = ProgramState::Targeting { range: range, item: item, aoe : aoe };

                                return;
                            
                            }

                            if self.world.get::<&Equippable>(item).is_ok()
                            {
                                let equip = self.world.get::<&Equippable>(item).unwrap();
                                    let slot = equip.slot;

                                    std::mem::drop(equip);
    
                                    let _ =self.world.insert_one(self.player_ent.unwrap(), WantsToEquipItem{item, slot});

                                    let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());

                                    self.current_state = ProgramState::Ticking;
                                    return;
                                
                            }

                            let _ = self.world.insert_one(self.player_ent.unwrap(), WantsToUseItem{item, target: None});
                            let _ = self.world.remove_one::<MyTurn>(self.player_ent.unwrap());
                            self.current_state = ProgramState::Ticking;
                            return;

                            
                            
                        }
                        _ => {self.current_state = ProgramState::AwaitingInput}
                    }
                }
            }


            ProgramState::GameOver =>
            {
                self.renderer.draw_char(250, 100, "You have died!", RED);
                if is_key_down(KeyCode::Escape)
                {
                    quit();
                }
            }
            _ =>
            {
                self.current_state = ProgramState::AwaitingInput;
            }
        }
    }
}

fn run_systems(state: &mut State)
{

    VisibilitySystem::run(state);

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

    //camera::render_camera(state);
    effects::run_animation_queue(state);
    camera::render_camera(state);
    gui::draw_ui(state);
    gui::draw_status_box(state);
    gui::draw_gamelog(state);
}

fn game_init ( state: &mut State)
{
    raws::run();
    
    //let item = raws::RawMaster::spawn_named_item(raws::RAWS.lock().unwrap()., new_entity, key, pos)
    //Spawn player object
    state.generate_world_map(0);
    let _asset = maps::RexAssests::new();
    let xy = state.player_pos;
    
    state.player_pos = xy;
    state.player_ent = Some( state.world.spawn((Position::new(xy.x,xy.y),
    Renderable::new("@".to_string(),
    RGB::named(LIME_GREEN),
    RGB::from_f32(0., 0., 0.),
    3)
    ,FoV::new(16)
    ,Name{name: "Player".to_string(),}
    , Pools{hitpoints: StatPool::new(50), exp: statistics::calculate_xp_from_level(1),level: 1, armour_class: Attribute::new(10)
        , hit_die: DiceType::new(1, 10, 1)}
    , BaseStatistics::roll_stats(3)
    , HungerLevel{nutrition: StatPool::new(300)}
    , Energy{value: 100}
    ,Faction{name: "Player".to_string()}
    , Player{})));

    spawning_system::spawn_item_in_backpack(state, &"Ration".to_string(), state.player_ent.unwrap());

    spawning_system::spawn_item_equipped(state, &"Rusted Knuckle Duster".to_string(), state.player_ent.unwrap());


    spawning_system::spawn_item_equipped(state, &"Light Pistol".to_string(), state.player_ent.unwrap());

    state.game_log.add_log("You wake up in your rundown apartment with no memory of what happened last night".to_string());
    state.game_log.add_log("You check your deck for notifications and see a message from an unknown user".to_string());
    state.game_log.add_log("The message simply states 'RUN, *THEY* are coming'".to_string());
    state.world.spawn((FoV::new(10), ControlNode{level: 3}, NodeOwned {owner: state.player_ent.unwrap()}
        , Position{x: xy.x, y: xy.y}));


}

fn create_state(renderer : Renderer) -> State
{
    let mut gs: State = State{
        world: World::new(),

        map : Map 
        {
            map :Vec::new()
            ,revealed_tiles : vec![false;69usize]
            ,visible_tiles : vec![false;69usize]
            ,blocked : vec![false;69usize]
            ,tile_contents : vec![Vec::new(); 69usize], depth: 0, props: HashMap::new()
            , view_blocked : HashSet::new(),
            map_width: 69,
            map_height: 69,
        },

        rng : bracket_lib::random::RandomNumberGenerator::new(),
        current_state : ProgramState::Ticking,
        player_pos : Point::zero(),
        player_ent: None,
        game_log : GameLog::new(),
        particle_builder : ParticleBuilder::new(),
        target_mode: TargettingMode::Keyboard{cursor_pos: Point::zero()},
        turn_number: 0,
        network_map: NetworkMap::empty(),
        renderer,
    };
    game_init(&mut gs);

    gs
}

#[macroquad::main("CyberRL")]
async fn main()
{
    let font = load_ttf_font("./assets/fonts/Mx437_ATI_8x8.ttf")
    .await
    .unwrap();

    let res = Arc::new(renderer::Resources{bmp_font: Arc::new(load_texture("./assets/fonts/fontbmp/nived16x16.png").await.unwrap()
        ), font_width : 16, font_height: 16});

    let mut font_list = Fonts::default();

    font_list.load_font_from_bytes("Julia Mono", JULIA).unwrap();
    font_list.load_font_from_bytes("Noto Sans Symbols", NOTO_SANS_SYMBOLS).unwrap();
    

    //egui::Context::

    let mut fonts = FontDefinitions::default();

    //todo WHAT IN THE FUCKERY IS THIS I HATE IT PLS FUCKING NOT DO THIS PLS FUKCING CHANGE AAAAAAAAAAAAAAAAAAAAAAAAAAAAA
    fonts.font_data.insert("julia".to_owned(), 
    std::sync::Arc::unwrap_or_clone(
        std::sync::Arc::new(FontData::from_static(include_bytes!("../assets/fonts/JuliaMono-Bold.ttf")) )) );

    //Makes newly added julia font the highest priority
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "julia".to_owned());

    //Makes julia font the monospace fallback font
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("julia".to_owned());

    em::cfg(|ctx| {ctx.set_fonts(fonts);});


    let mut rend = renderer::Renderer
    {
        mode : renderer::RenderBackend::MacroQuad,
        default_font: font,
        canvas: GraphicGrid::new(30, 30, 15, 15),
        char_size: CharSize(0, 0, 0),
        map_view_size: (30,20),
        textures: res.clone()
    };

    let size = measure_text("x", Some(&rend.default_font), rend.canvas.tile_height as u16, 1.0);
    rend.char_size = CharSize(size.width as i32, size.height as i32, size.offset_y as i32);
    //let cam = Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 152.0, 320.0, -152.0));
    let mut state = create_state(rend.clone());

    state.renderer.setup_grid();
    loop {
        clear_background(GRAY);
        //set_camera(&Camera2D {
          //  zoom: vec2(1., screen_width() / screen_height()),
            //..Default::default()
        //});
        em::ui(|egui_ctx| {
            gui::mqui::ui_layout(egui_ctx, &state);
            
            MANAGER.lock().unwrap().show(egui_ctx, &mut state);
            
        });

        state.tick();

        em::draw();
        //draw_tiles(&rend);
        
        next_frame().await
    }
    //old_main();
}
