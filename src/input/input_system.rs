use bracket_lib::prelude::Point;
use hecs::Entity;

use crate::{
    Position, ProgramState, State,
    ai::{MyTurn, apply_energy_cost},
    attack_system::AttackSystem,
    camera,
    components::{
        BlocksTiles, BlocksVisibility, Door, EquipmentSlot, Equipped, FoV, HasMoved, InContainer,
        Item, Name, RangedWeapon, Renderable, WantsToPickupItem, WantsToRest,
    },
    effects::{EffectType, Targets, add_effect},
    gamelog::DEBUGLOG,
    go_down_stairs,
    gui::{TargettingMode, mqui::ItemWindowMode},
    map_indexing::SPATIAL_INDEX,
    maps::TileType,
    player::Player,
    ranged_combat::ranged_aim::select_nearest_target_pos,
    screen_manager::{self, MANAGER},
    statistics::Pools,
};

use super::{Command, INPUT};

///Rework of player_input_system using the new command enum instead of direct reading of inputs
pub fn input_system(state: &mut State) -> ProgramState {
    if let Some(command) = INPUT.lock().get_command() {
        match command {
            Command::Move { pos } => attempt_move(state, pos.x, pos.y),
            Command::Drop => drop_item(state),
            Command::GoDownStairs => {
                let idx = state.map.xy_idx(state.player_pos.x, state.player_pos.y);
                if state.map.map[idx] == TileType::DownStairs {
                    go_down_stairs(state);
                    return ProgramState::Ticking;
                } else {
                    return ProgramState::AwaitingInput;
                }
            }
            Command::Unequip => unequip_item(state),
            Command::Inventory => open_inventory(state),
            Command::Wait => {
                {
                    state
                        .world
                        .insert_one(state.player_ent.unwrap(), WantsToRest {})
                        .expect("Couldn't insert WantsToRest componenent onto player!");
                    let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());
                    //apply_energy_cost(state, crate::ai::ActionType::Move, state.player_ent.unwrap());
                    return ProgramState::Ticking;
                }
            }
            Command::Fire => fire_ranged_weapon(state),
            Command::Pickup => pickup_items(state),
            Command::Look => {
                let (min_x, _, min_y, _) = camera::get_screen_bounds(state);

                let player_pos = state.player_pos;
                let px = player_pos.x;
                let py = player_pos.y;

                return ProgramState::KeyboardTargetting {
                    cursor_pos: Point::new(px - min_x, py - min_y),
                };
            }

            Command::DevConsole => {
                MANAGER.lock().unwrap().toggle_view();
                ProgramState::AwaitingInput
            }

            _ => ProgramState::AwaitingInput,
        }
    } else {
        ProgramState::AwaitingInput
    }
}

fn pickup_items(state: &mut State) -> ProgramState {
    let mut items = Vec::new();

    let player_pos = state.player_pos;
    let player_idx = state.map.xy_idx(player_pos.x, player_pos.y);

    SPATIAL_INDEX
        .lock()
        .unwrap()
        .for_each_tile_content(player_idx, state, |entity, state| {
            if let Ok(_) = state.world.get::<&Item>(entity) {
                items.push((entity, false));
            }
        });

    if items.is_empty() {
        ProgramState::AwaitingInput
    } else if items.len() == 1 {
        state
            .world
            .insert_one(
                state.player_ent.unwrap(),
                WantsToPickupItem { item: items[0].0 },
            )
            .unwrap();

        let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());
        apply_energy_cost(
            state,
            crate::ai::ActionType::Pickup,
            state.player_ent.unwrap(),
        );
        //signifies that player's turn has now ended
        ProgramState::Ticking
    } else if items.len() > 1 {
        let menu_type = screen_manager::MenuType::Pickup;

        MANAGER.lock().unwrap().create_menu(
            items,
            "Pickup:".to_string(),
            crate::gui::mqui::ItemWindowMode::Multiple,
            menu_type,
            state,
        );

        return ProgramState::AwaitingMenu {
            response: None,
            menu_type,
        };
        //return   ProgramState::SelectionMenu { items: items.clone(), menu: MenuType::PickupItem };
    } else {
        state
            .game_log
            .add_log(String::from("No items to pick up at current location!"));
        return ProgramState::AwaitingInput;
    }
}

fn fire_ranged_weapon(state: &mut State) -> ProgramState {
    {
        let query = state
            .world
            .query::<&Equipped>()
            .iter()
            .filter(|(_ent, equip)| {
                equip.slot == EquipmentSlot::Ranged
                    && equip.owner
                        == state
                            .player_ent
                            .expect("Couldn't find player entity to fetch ranged stats for combat")
            })
            .map(|(ent, _eq)| ent)
            .collect::<Vec<_>>();

        if query.len() < 1 {
            DEBUGLOG.add_log(String::from("F pressed but no ranged weapon equipped!"));
            return ProgramState::AwaitingInput;
        } else {
            let dmg = 4;
            // state.world.query_one_mut::<&CombatStats>(state.player_ent
            // .expect("Couldn't get player ent for damage for ranged combat"))
            // .expect("Couldn't get the Combat stats for player for ranged combat").power.total;

            let range = state
                .world
                .query_one_mut::<&RangedWeapon>(query[0])
                .expect("Couldn't get range of players ranged weapon for range combat")
                .range;

            if let TargettingMode::Keyboard { cursor_pos } = state.target_mode {
                state.target_mode = TargettingMode::Keyboard {
                    cursor_pos: select_nearest_target_pos(
                        state,
                        state.player_ent.unwrap(),
                        cursor_pos,
                    ),
                }
            }
            return ProgramState::RangedCombat { range, dmg };
        }
    }
}

fn open_inventory(state: &mut State) -> ProgramState {
    {
        let items = state
            .world
            .query::<(&Item, &InContainer, &Name)>()
            .iter()
            .filter(|ent| ent.1.1.owner == state.player_ent.unwrap())
            .map(|ent| (ent.0, false))
            .collect::<Vec<(Entity, bool)>>();

        let menu_type = screen_manager::MenuType::Inventory;

        MANAGER.lock().unwrap().create_menu(
            items,
            "Inventory:".to_string(),
            ItemWindowMode::Single,
            menu_type,
            state,
        );

        return ProgramState::AwaitingMenu {
            response: None,
            menu_type,
        };
    }
}

fn unequip_item(state: &mut State) -> ProgramState {
    {
        let items = state
            .world
            .query::<&Equipped>()
            .iter()
            .filter(|(_ent, eq)| eq.owner == state.player_ent.unwrap())
            .map(|(ent, _eq)| (ent, false))
            .collect::<Vec<_>>();

        let menu_type = screen_manager::MenuType::Unequip;

        MANAGER.lock().unwrap().create_menu(
            items,
            "Unequip Items:".to_string(),
            crate::gui::mqui::ItemWindowMode::Multiple,
            menu_type,
            state,
        );

        return ProgramState::AwaitingMenu {
            response: None,
            menu_type,
        };
    }
}

fn drop_item(state: &mut State) -> ProgramState {
    {
        let items = state
            .world
            .query::<&InContainer>()
            .iter()
            .filter(|(_ent, cont)| cont.owner == state.player_ent.unwrap())
            .map(|(ent, _cont)| (ent, false))
            .collect::<Vec<_>>();

        let menu_type = screen_manager::MenuType::Drop;
        MANAGER.lock().unwrap().create_menu(
            items,
            "Drop Items:".to_string(),
            crate::gui::mqui::ItemWindowMode::Multiple,
            menu_type,
            state,
        );

        return ProgramState::AwaitingMenu {
            response: None,
            menu_type,
        };
    }
}

fn attempt_move(state: &mut State, delta_x: i32, delta_y: i32) -> ProgramState {
    match try_move(state, delta_x, delta_y) {
        true => {
            return ProgramState::Ticking;
        }
        false => {
            return ProgramState::AwaitingInput;
        }
    }
}

pub fn try_move(state: &mut State, delta_x: i32, delta_y: i32) -> bool {
    let mut moved = false;
    let mut destination_id: usize = 0;
    let (id, _player) = state
        .world
        .query_mut::<&Player>()
        .into_iter()
        .next()
        .expect("No Player found!");
    let mut attacker: Entity = id;
    let mut target = id;

    let spatial_map = SPATIAL_INDEX.lock().unwrap();

    let (_, position, fov) = state
        .world
        .query_one_mut::<(&Player, &mut Position, &mut FoV)>(state.player_ent.unwrap())
        .unwrap();

    destination_id = state.map.xy_idx(position.x + delta_x, position.y + delta_y);
    if destination_id >= state.map.map.len() {
        return false;
    }
    if !spatial_map.is_tile_blocked(destination_id) {
        position.x = std::cmp::min(
            state.map.map_width - 1,
            std::cmp::max(0, position.x + delta_x),
        );
        position.y = std::cmp::min(
            state.map.map_height - 1,
            std::cmp::max(0, position.y + delta_y),
        );
        state.player_pos = Point::new(position.x, position.y);
        fov.dirty = true;
        moved = true;
        attacker = id;
    } else if state.map.map[destination_id] == TileType::Wall {
        return false;
    }

    if !moved {
        let mut door_to_open = Vec::new();

        for (ent, (door, pos)) in state.world.query_mut::<(&Door, &Position)>() {
            if state.map.xy_idx(pos.x, pos.y) == destination_id && door.open == false {
                door_to_open.push(ent);
            }
        }

        for door in door_to_open.iter() {
            add_effect(
                state.player_ent,
                EffectType::ToggleDoor,
                Targets::Single { target: *door },
            );

            apply_energy_cost(
                state,
                crate::ai::ActionType::OpenDoor,
                state.player_ent.unwrap(),
            );
            let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());
            return true;
        }
    }
    if moved {
        state
            .world
            .insert_one(state.player_ent.unwrap(), HasMoved {})
            .unwrap();

        apply_energy_cost(
            state,
            crate::ai::ActionType::Move,
            state.player_ent.unwrap(),
        );
        let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());
        return true;
    }

    let contents = spatial_map.get_tile_contents(destination_id);
    if contents.len() > 0 && !moved {
        let mut found_target = false;
        for potential_target in contents.iter() {
            let query = state
                .world
                .query_one_mut::<(&Pools, &Name)>(*potential_target);
            match query {
                Ok(res) => {
                    print!("I will stab thee now, {}!", res.1.name);
                    target = *potential_target;
                    found_target = true;
                }
                Err(_) => {}
            }
        }

        if found_target {
            //console::log(format!("Target found! {}",state.world.get::<&Name>(target).expect("No target name found!").name));
            AttackSystem::add_attack(attacker, target, state);
            apply_energy_cost(
                state,
                crate::ai::ActionType::Attack,
                state.player_ent.unwrap(),
            );

            let _ = state.world.remove_one::<MyTurn>(state.player_ent.unwrap());

            return true;
        }
    }

    println!("This should be basically unreachable");
    false
}
