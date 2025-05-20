use std::clone;

use bracket_lib::prelude::Point;
use hecs::Entity;

use crate::{
    components::{AoE, Creator},
    effects::{add_effect, get_aoe_tiles, EffectType, Targets},
    State,
};

pub struct WantsToInteract {
    pub machine: Entity,
    pub target: Option<Point>,
}

///fires off interaction events for all entities with WantsToInteract Component set
pub fn interaction_system(state: &mut State) {
    let ents_to_interact = state
        .world
        .query_mut::<&WantsToInteract>()
        .into_iter()
        .map(|ent| (ent.0.clone(), ent.1.machine, ent.1.target))
        .collect::<Vec<(Entity, Entity, Option<Point>)>>();

    for (ent, machine, target) in ents_to_interact.iter() {
        match *target {
            Some(target) => match state.world.query_one_mut::<&AoE>(*machine) {
                Ok(aoe) => {
                    let range = aoe.radius;

                    let tiles = get_aoe_tiles(state, range, target);

                    add_effect(
                        Some(*ent),
                        EffectType::InteractMachine { machine: *machine },
                        Targets::Tiles {
                            tiles: tiles.clone(),
                        },
                    );
                }

                Err(_) => {
                    add_effect(
                        Some(*ent),
                        EffectType::InteractMachine { machine: *machine },
                        Targets::Tile {
                            tile_idx: state.map.xy_idx(target.x, target.y) as i32,
                        },
                    );
                }
            },

            //self targetted
            None => {
                add_effect(
                    Some(*ent),
                    EffectType::InteractMachine { machine: *machine },
                    Targets::Single { target: *ent },
                );
            }
        }

        let _ = state.world.remove_one::<WantsToInteract>(*ent);
    }
}
