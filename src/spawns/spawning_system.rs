use core::panic;
use std::collections::{HashMap, HashSet};

use crate::ai::Energy;
use crate::components::{
    Consumable, EffectDuration, EffectSpawnerPrefab, Interactable, StatusEffect,
};
use crate::map_indexing::SPATIAL_INDEX;
use crate::raws::{get_spawn_table_for_depth, SpawnType, RAWS};
use crate::{
    raws::RawMaster, DamageEffect, HealingEffect, Item, Name, Position, RangedTargetting,
    Renderable, State,
};
use crate::{EquipmentSlot, Equippable, Equipped, InContainer, Map, TileType, Usable};
use bracket_lib::prelude::{console, Rect};
use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::Point;
use hecs::{Entity, EntityBuilder};

use super::randomtable::RandomTable;

pub const MAXMOBS: i32 = 6;
pub enum EntityType {
    Item,
    Mob,
    Prop,
}

pub fn spawn_effect_entity(
    state: &mut State,
    effects: HashMap<String, String>,
    duration: Option<i32>,
    item: Entity,
    target: Entity,
) {
    let mut eb = EntityBuilder::new();

    eb.add(StatusEffect {
        source: item,
        target,
    });

    if duration.is_some() {
        eb.add(EffectDuration {
            rounds: duration.unwrap(),
        });
    }

    eb = RawMaster::add_effects_comps(eb, effects);

    state.world.spawn(eb.build());
}

/// TODO: add checks for if there is already an item equipped in that slot
pub fn spawn_item_equipped(state: &mut State, item_name: &String, target: Entity) {
    let mut item_builder = RawMaster::spawn_named_item(
        &RAWS.lock().unwrap(),
        hecs::EntityBuilder::new(),
        &item_name,
        SpawnType::Equipped { target },
    );

    let mut slot: Option<EquipmentSlot> = None;
    match item_builder {
        Some(mut build_box) => {
            let query = build_box.get::<&Equippable>();
            match query {
                Some(equippable) => {
                    slot = Some(equippable.slot);

                    build_box.add(Equipped {
                        owner: target,
                        slot: slot.expect("Couldn't get slot"),
                    });

                    state.world.spawn(build_box.build());
                }

                None => {
                    panic!(
                        "Can't spawn and equip item {} as it isn't equippable",
                        build_box.get::<&Name>().expect("Can't get item name!").name
                    )
                }
            }
        }

        None => {
            panic!("No entity builder found!");
        }
    }
}

pub fn spawn_item_in_backpack(state: &mut State, item_name: &String, owner: Entity) {
    let mut item_builder = RawMaster::spawn_named_item(
        &RAWS.lock().unwrap(),
        hecs::EntityBuilder::new(),
        &item_name,
        SpawnType::InBackpack,
    );

    match item_builder {
        Some(mut builder) => {
            builder.add(InContainer { owner });
            let ent = state.world.spawn(builder.build());
        }
        None => console::log(format!(
            "Could spawn {} in backpack as no item with that name exists!",
            item_name
        )),
    }
}

pub fn spawn_entity(
    state: &mut State,
    spawn: &(&usize, &String),
    x: i32,
    y: i32,
    ent_type: EntityType,
) {
    match ent_type {
        EntityType::Item => {
            let item_res = RawMaster::spawn_named_item(
                &RAWS.lock().unwrap(),
                hecs::EntityBuilder::new(),
                &spawn.1,
                SpawnType::AtPosition { x, y },
            );
            match item_res {
                Some(mut item) => {
                    state.world.spawn(item.build());
                }

                None => {
                    bracket_lib::terminal::console::log(format!(
                        "Can't find item entity named {}",
                        &spawn.1
                    ));
                }
            }
        }

        EntityType::Mob => {
            let (mob_res, equip_list) = RawMaster::spawn_named_mob(
                &RAWS.lock().unwrap(),
                hecs::EntityBuilder::new(),
                &spawn.1,
                SpawnType::AtPosition { x, y },
            );
            match mob_res {
                Some(mut mob) => {
                    //gives random energy so not every mob processes on same tick!
                    mob.add(Energy {
                        value: state.rng.range(-120, 71),
                    });

                    let mob_ent = state.world.spawn(mob.build());
                    for eq in equip_list.iter() {
                        spawn_item_equipped(state, eq, mob_ent);
                    }
                    let idx = state.map.xy_idx(x, y);
                    SPATIAL_INDEX
                        .lock()
                        .unwrap()
                        .set_tile_blocked_by_entity(idx);
                }

                None => {
                    bracket_lib::terminal::console::log(format!(
                        "Can't find mob entity named {}",
                        &spawn.1
                    ));
                }
            }
        }

        EntityType::Prop => {
            let prop_res = RawMaster::spawn_named_prop(
                &RAWS.lock().unwrap(),
                hecs::EntityBuilder::new(),
                &spawn.1,
                SpawnType::AtPosition { x, y },
            );

            match prop_res {
                Some(mut prop) => {
                    if prop.has::<EffectSpawnerPrefab>() {
                        {
                            prop.add(Interactable);
                            let prefab = prop.get::<&EffectSpawnerPrefab>().unwrap().clone();

                            let entity = state.world.spawn(prop.build());

                            let mut spawner_builder = RawMaster::spawn_effect_spawner(
                                &RAWS.lock().unwrap(),
                                hecs::EntityBuilder::new(),
                                entity,
                                prefab,
                            );

                            let _spawner = state.world.spawn(spawner_builder.build());

                            let _ = state.world.remove_one::<EffectSpawnerPrefab>(entity);

                            return;
                        }
                    }
                    state.world.spawn(prop.build());
                }

                None => {
                    bracket_lib::terminal::console::log(format!(
                        "Can't find prop entity named {}",
                        &spawn.1
                    ));
                }
            }
        }
    }
}

fn room_table(state: &mut State) -> RandomTable {
    get_spawn_table_for_depth(&RAWS.lock().unwrap(), state.map.depth)
}

pub fn roll_spawn_table(depth: i32) -> (String, EntityType) {
    let mob_names = RAWS.lock().unwrap().get_mob_name_list();
    let item_names = RAWS.lock().unwrap().get_item_name_list();
    let prop_names = RAWS.lock().unwrap().get_prop_name_list();

    let table = get_spawn_table_for_depth(&RAWS.lock().unwrap(), depth);

    let mut rng = RandomNumberGenerator::new();
    let name = table.roll(&mut rng);
    let entity_type = get_entity_type(&name);

    (name.clone(), entity_type)
}

pub fn get_entity_type(name: &String) -> EntityType {
    let mob_names = RAWS.lock().unwrap().get_mob_name_list();
    let item_names = RAWS.lock().unwrap().get_item_name_list();
    let prop_names = RAWS.lock().unwrap().get_prop_name_list();

    let mut entity_type = EntityType::Mob;

    if mob_names.contains(&name) {
        entity_type = EntityType::Mob;
    } else if item_names.contains(&name) {
        entity_type = EntityType::Item;
    } else if prop_names.contains(&name) {
        entity_type = EntityType::Prop;
    } else {
        panic!(
            "{} is not a valid item, mob or prop name so can't be spawned",
            name
        );
    }

    entity_type
}

pub fn spawn_room(room: Rect, depth: i32, spawn_list: &mut Vec<(usize, String)>, map: &Map) {
    let mut num_mobs = 0;
    let mut num_items = 0;
    let mut ent_type = EntityType::Mob;

    let mut rng = RandomNumberGenerator::new();

    let mut attempts = 20;

    let mut num_spawns = rng.range(0, MAXMOBS + 1);

    let mut spawn_points: HashSet<usize> = HashSet::new();

    while attempts > 0 && num_spawns > 0 {
        let (name, _) = roll_spawn_table(depth);

        let pos_set = room.point_set();
        let point = pos_set.iter().next().unwrap();
        let pos = map.xy_idx(point.x, point.y);
        if !spawn_points.contains(&pos) && map.map[pos] != TileType::Wall {
            //spawn_entity(state, &(&0, &name), point.x, point.y, ent_type);
            spawn_points.insert(pos);
            spawn_list.push((pos, name));
            num_spawns -= 1;
        } else {
            {
                attempts -= 1;
            }
        }
    }
}

pub fn spawn_region(area: &[usize], map_depth: i32, spawn_list: &mut Vec<(usize, String)>) {
    let mut areas = Vec::from(area);

    let mut rng = RandomNumberGenerator::new();

    let mut attempts = 20;

    let mut num_spawns = std::cmp::min(rng.range(0, MAXMOBS + 1), area.len() as i32);

    let mut spawn_points: HashMap<usize, String> = HashMap::new();

    for _i in 0..num_spawns {
        let array_index = if areas.len() == 1 {
            0usize
        } else {
            (rng.roll_dice(1, areas.len() as i32) - 1) as usize
        };

        let map_idx = areas[array_index];
        areas.remove(array_index);
        let (name, _) = roll_spawn_table(map_depth);
        spawn_points.insert(map_idx, name);
    }

    for (idx, name) in spawn_points.iter() {
        spawn_list.push((*idx, name.clone()));
        //spawn_entity(state, &(&0usize, name), *idx as i32  % map.map_width, *idx as i32 / map.map_width, ent_type);
    }
}
