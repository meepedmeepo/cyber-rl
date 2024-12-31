use bracket_lib::prelude::*;
use crate::{attack_system, go_down_stairs, statistics::Pools, EquipmentSlot, Equippable, Equipped, Item, RangedTargetting, RangedWeapon, TileType, WantsToPickupItem};

use super::{State,ProgramState,MAPHEIGHT,MAPWIDTH,Entity,Map,Name,AttackSystem,FoV,Position};
use std::{clone, cmp::{max, min}};



pub struct Player
{}

pub fn player_input_system(ctx:&BTerm, state: &mut State) -> ProgramState
{
    match ctx.key
    {
        None => {return ProgramState::AwaitingInput;},
        Some(key) => match key
        {
            VirtualKeyCode::Numpad4 =>try_move(state, -1, 0),
            VirtualKeyCode::Numpad6 => try_move(state,1,0),
            VirtualKeyCode::Numpad8 => try_move(state,0,-1),
            VirtualKeyCode::Numpad2 => try_move(state,0,1),
            VirtualKeyCode::A=>try_move(state, -1, 0),
            VirtualKeyCode::D => try_move(state,1,0),
            VirtualKeyCode::W => try_move(state,0,-1),
            VirtualKeyCode::S => try_move(state,0,1),
            VirtualKeyCode::I => return ProgramState::Inventory,
            VirtualKeyCode::Space => return ProgramState::PlayerTurn,
            VirtualKeyCode::F => 
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

                    return ProgramState::RangedCombat { range, dmg  };
                }
            
            },
            VirtualKeyCode::Period => 
            {
                let idx = Map::xy_id(state.player_pos.x, state.player_pos.y);
                if state.map.map[idx] == TileType::DownStairs
                {
                    go_down_stairs(state);
                    return ProgramState::PlayerTurn;
                }
                else
                { 
                    return ProgramState::AwaitingInput;
                }
            }
            VirtualKeyCode::G => 
            {
                let mut item : Option<Entity> = None;
                for ent in state.map.tile_contents[Map::xy_id(state.player_pos.x, state.player_pos.y)].iter()
                {

                    match state.world.get::<&Item>(*ent)
                    {
                        Ok(_) =>
                        {
                            item = Some(*ent);
                            break;
                        }

                        Err(_) => 
                        {}
                    }
                }
                match item
                {
                    Some(i) =>
                    {
                        state.world.insert_one(Option::expect(state.player_ent
                            , "Couldn't find player entity to insert WantsToPickupItem component!"),
                  WantsToPickupItem{item: i})
                            .expect("Couldn't insert WantsToPickupItem component onto the player");
                        return ProgramState::AwaitingInput;
                    }
                    None =>
                    {
                        return ProgramState::AwaitingInput;
                    }
                }

            }
            _ =>{return ProgramState::AwaitingInput;},

        }

    }
    ProgramState::PlayerTurn
}


/// TODO: cleanup this absolute fucking mess holy shit wtf
pub fn try_move(state: &mut State,delta_x:i32,delta_y:i32)
{
    let mut moved =  false;
    let mut destination_id : usize = 0;
    let (id,_player) =  state.world.query_mut::<&Player>().into_iter().next().expect("No Player found!");
    let mut attacker : Entity = id;
    let mut target = id;

    for(_id,(_player,position,fov)) in state.world.query_mut::<(&Player,&mut Position,&mut FoV)>()
    {
        destination_id = Map::xy_id(position.x+delta_x, position.y+delta_y);
        if !state.map.blocked[destination_id]
        {
        position.x = min(MAPWIDTH -1,max(0,position.x+delta_x));
        position.y = min(MAPHEIGHT - 1,max(0,position.y+delta_y));
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