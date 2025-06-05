use bracket_lib::prelude::console;
use hecs::Entity;

use crate::{
    components,
    effects::{self, add_effect, EffectType, Targets},
    statistics::BaseStatistics,
    utils::get_mob_entities_at_position,
    Name, Position, RangedWeapon, Renderable, State,
};

use super::{Projectile, ProjectileType};

pub struct ProjectileUpdated {}

pub fn projectile_system(state: &mut State) {
    let mut proj_to_update = Vec::new();
    let mut proj_to_despawn: Vec<Entity> = Vec::new();

    for (id, (proj_type, _updated, anim, proj)) in state.world.query_mut::<(
        &ProjectileType,
        &ProjectileUpdated,
        &effects::Animation,
        &components::Projectile,
    )>() {
        let pos = anim.path[anim.index];

        proj_to_update.push((id, *proj_type, pos, anim.creator.clone(), proj.damage));
    }

    for (proj, proj_type, pos, creator, damage) in proj_to_update.iter() {
        let _ = state.world.remove_one::<ProjectileUpdated>(*proj);

        let hits = get_mob_entities_at_position(state, *pos);

        if hits.len() < 1 {
            continue;
        }

        let query = state
            .world
            .query_one_mut::<(&Name, &BaseStatistics)>(hits[0])
            .expect("Couldn't get name or stats of entity attempting to dodge missile!");

        let name = query.0.clone();
        let stats = *query.1;

        let mut roll = state.rng.roll_dice(1, 20);
        roll += stats.dexterity.get_modifier();

        if roll < 15 {
            //make a way of adding the original creator of the projectile to this
            add_effect(
                Some(*creator),
                EffectType::Damage {
                    amount: state.rng.roll(*damage),
                },
                Targets::Single { target: hits[0] },
            );

            let msg = format!("{} was hit by missile", name.name.clone());
            console::log(msg.clone());
            state.game_log.add_log(msg);

            if *proj_type == ProjectileType::Missile {
                proj_to_despawn.push(*proj);
            }
        } else {
            let msg = format!("{} dodged missile!", name.name.clone());
            console::log(msg.clone());
            state.game_log.add_log(msg);
        }
    }

    for proj in proj_to_despawn.iter() {
        let _ = state.world.despawn(*proj);
    }
}
