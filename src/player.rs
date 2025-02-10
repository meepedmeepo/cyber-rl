use bracket_lib::prelude::*;
use macroquad::input::{get_keys_down, get_keys_pressed, KeyCode};
use crate::{ai::{apply_energy_cost, MyTurn}, attack_system, camera, go_down_stairs, gui::TargettingMode, menus::MenuType, ranged_combat::ranged_aim::select_nearest_target_pos, statistics::Pools, BlocksTiles, BlocksVisibility, Door, EquipmentSlot, Equippable, Equipped, HasMoved, InContainer, Item, RangedTargetting, RangedWeapon, Renderable, TileType, WantsToPickupItem, WantsToRest};

use super::{State,ProgramState,Entity,Map,Name,AttackSystem,FoV,Position};
use std::{ cmp::{max, min}};



pub struct Player
{}

pub fn player_input_system(state: &mut State) -> ProgramState
{

    let keys = get_keys_pressed();
    for key in keys.iter()
    {
        match key
        {
            KeyCode::Kp4 =>return attempt_move(state, -1, 0),
            KeyCode::Kp6 => return attempt_move(state,1,0),
            KeyCode::Kp8 => return attempt_move(state,0,-1),
            KeyCode::Kp2 => return attempt_move(state,0,1),
        
            // KeyCode::A=>try_move(state, -1, 0),
            // KeyCode::D => try_move(state,1,0),
            // KeyCode::W => try_move(state,0,-1),
            // KeyCode::S => try_move(state,0,1),

            // Diagonals
            KeyCode::Kp9 => return attempt_move(state, 1, -1),

            KeyCode::Kp7 => return attempt_move(state, -1, -1),

            KeyCode::Kp3 => return attempt_move(state, 1, 1),

            KeyCode::Kp1 => return attempt_move(state, -1, 1),


            KeyCode::D =>
            {
                let items =state.world.query::<&InContainer>()
                    .iter()
                    .filter(|(_ent, cont)| cont.owner == state.player_ent.unwrap())
                    .map(|(ent, _cont)| (ent, false))
                    .collect::<Vec<_>>();

                return ProgramState::SelectionMenu { items: items.clone(), menu: MenuType::DropItem };
            }
            KeyCode::R =>
            {
                let items =state.world.query::<&Equipped>()
                    .iter()
                    .filter(|(_ent, eq)| eq.owner == state.player_ent.unwrap())
                    .map(|(ent, _eq)| (ent, false))
                    .collect::<Vec<_>>();

                return ProgramState::SelectionMenu { items: items.clone(), menu: MenuType::UnequipItem };
            }

            KeyCode::I => return ProgramState::Inventory,
            KeyCode::Space => 
            {
                state.world.insert_one(state.player_ent.unwrap(), WantsToRest{})
                    .expect("Couldn't insert WantsToRest componenent onto player!");
                let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());
                //apply_energy_cost(state, crate::ai::ActionType::Move, state.player_ent.unwrap());
                return ProgramState::Ticking;
            },
            KeyCode::Kp5 => 
            {
                state.world.insert_one(state.player_ent.unwrap(), WantsToRest{})
                    .expect("Couldn't insert WantsToRest componenent onto player!");
                let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());
                //apply_energy_cost(state, crate::ai::ActionType::Move, state.player_ent.unwrap());
                return ProgramState::Ticking;
            },
            KeyCode::F => 
            {

                let query = state.world.query::<&Equipped>()
                    .iter()
                    .filter(|(_ent,equip) | 
                        equip.slot == EquipmentSlot::Ranged && equip.owner == state.player_ent
                    .expect("Couldn't find player entity to fetch ranged stats for combat"))
                    .map(|(ent, _eq)| ent)
                    .collect::<Vec<_>>();

                if query.len() < 1
                {
                    console::log("F pressed but no ranged weapon equipped!");
                    return ProgramState::AwaitingInput;
                }
                else
                {
                    let dmg = 4;
                        // state.world.query_one_mut::<&CombatStats>(state.player_ent
                        // .expect("Couldn't get player ent for damage for ranged combat"))
                        // .expect("Couldn't get the Combat stats for player for ranged combat").power.total;

                    let range = state.world.query_one_mut::<&RangedWeapon>(query[0])
                            .expect("Couldn't get range of players ranged weapon for range combat").range;
                    
                    if let TargettingMode::Keyboard { cursor_pos} = state.target_mode
                    {
                        state.target_mode = TargettingMode::Keyboard { cursor_pos: 
                            select_nearest_target_pos(state, state.player_ent.unwrap(), cursor_pos) }
                    }
                    return ProgramState::RangedCombat { range, dmg  };
                }
            
            },
            KeyCode::Period => 
            {
                let idx = state.map.xy_idx(state.player_pos.x, state.player_pos.y);
                if state.map.map[idx] == TileType::DownStairs
                {
                    go_down_stairs(state);
                    return ProgramState::Ticking;
                }
                else
                { 
                    return ProgramState::AwaitingInput;
                }
            }
            KeyCode::G => 
            {

                let mut items = Vec::new();

                for ent in state.map.tile_contents[state.map.xy_idx(state.player_pos.x, state.player_pos.y)].iter()
                {
                    if state.world.get::<&Item>(*ent).is_ok()
                    {
                        items.push((*ent, false));
                    }
                }

                if items.len() == 1
                {
                    state.world.insert_one(state.player_ent.unwrap(), WantsToPickupItem{item: items[0].0}).unwrap();
                    
                    let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());
                    apply_energy_cost(state, crate::ai::ActionType::Pickup, state.player_ent.unwrap());
                } else if items.len() > 1
                {
                    return   ProgramState::SelectionMenu { items: items.clone(), menu: MenuType::PickupItem };
                }
                else 
                {
                    console::log("No items to pick up at current location!");
                    return ProgramState::AwaitingInput;
                }

            }

            KeyCode::Semicolon => 
            {
                let (min_x,max_x,min_y,max_y) = camera::get_screen_bounds(state);

                let player_pos = state.player_pos;
                let px = player_pos.x;
                let py = player_pos.y;

                return ProgramState::KeyboardTargetting { cursor_pos: Point::new(px - min_x, py - min_y) };
            }
            _ =>{},

        }

    }
    ProgramState::AwaitingInput
}

fn attempt_move(state: &mut State, delta_x:i32, delta_y:i32) -> ProgramState
{
    match try_move(state, delta_x, delta_y)
    {
        true => {return ProgramState::Ticking;}
        false => { return ProgramState::AwaitingInput;}
    }
}
/// TODO: cleanup this absolute fucking mess holy shit wtf
pub fn try_move(state: &mut State,delta_x:i32,delta_y:i32) -> bool
{
    let mut moved =  false;
    let mut destination_id : usize = 0;
    let (id,_player) =  state.world.query_mut::<&Player>().into_iter().next().expect("No Player found!");
    let mut attacker : Entity = id;
    let mut target = id;

    for(_id,(_player,position,fov)) in state.world.query_mut::<(&Player,&mut Position,&mut FoV)>()
    {
        destination_id = state.map.xy_idx(position.x+delta_x, position.y+delta_y);
        if destination_id >= state.map.blocked.len() 
        {
            return false;
        }
        if !state.map.blocked[destination_id]
        {
            position.x = min(state.map.map_width -1,max(0,position.x+delta_x));
            position.y = min(state.map.map_height - 1,max(0,position.y+delta_y));
            state.player_pos = Point::new(position.x, position.y);
            fov.dirty = true;
            moved = true;
            attacker = _id;
            break;
        }
        else if state.map.map[destination_id] == TileType::Wall
        {
            return false;
        }
        
    }

        let mut door_to_open = Vec::new();

            for (ent, (door, pos)) in state.world.query_mut::<(&Door, &Position)>()
            {
                if state.map.xy_idx(pos.x, pos.y) == destination_id && door.open == false
                {
                    door_to_open.push(ent);
                }
            }

            for door in door_to_open.iter()
            {
                let _ =state.world.remove_one::<BlocksTiles>(*door);
                let _ =state.world.remove_one::<BlocksVisibility>(*door);
                state.world.query_one_mut::<&mut Renderable>(*door).unwrap().glyph = "/".to_string();
                state.world.query_one_mut::<&mut FoV>(state.player_ent.unwrap()).unwrap().dirty = true;

                apply_energy_cost(state, crate::ai::ActionType::OpenDoor, state.player_ent.unwrap());
                let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());
                return true;
            }
        if moved
        {
            state.world.insert_one(state.player_ent.unwrap(), HasMoved{}).unwrap();
            apply_energy_cost(state, crate::ai::ActionType::Move, state.player_ent.unwrap());
            let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());
        }

        if state.map.tile_contents[destination_id].len() > 0 && !moved
        {
            
            let mut found_target = false;
            for potential_target in state.map.tile_contents[destination_id].iter()
            {
                let query = state.world.query_one_mut::<(&Pools,
                &Name)>(*potential_target);
                match query
                {
                    Ok(res) =>
                    {
                        console::log(&format!("I will stab thee now, {}!",res.1.name));
                        target = *potential_target;
                        found_target = true;
                    }
                    Err(_) =>{}
                }
            }

            

            if found_target
            {
                //console::log(format!("Target found! {}",state.world.get::<&Name>(target).expect("No target name found!").name));
                AttackSystem::add_attack(attacker, target, state);
                apply_energy_cost(state, crate::ai::ActionType::Attack, state.player_ent.unwrap());

                let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());

                return true;
            }

        
        }

        true

}