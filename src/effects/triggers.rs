use bracket_lib::prelude::{Point, console};
use hecs::Entity;

use crate::{
    Consumable, DamageEffect, GivesFood, HealingEffect, Hidden, Map, Position, Projectile,
    RangedWeapon, State,
    components::{DescendFloors, Door, GrantsStatus},
    events::Event,
    gamelog,
    raws::RawMaster,
};

use super::{
    ANIMATIONQUEUE, EffectType, ParticleAnimation, ParticleBurst, ParticleLine, Targets,
    add_effect, animation::Animation,
};

pub fn ranged_trigger(creator: Option<Entity>, item: Entity, targets: &Targets, state: &mut State) {
    event_trigger(creator, item, targets, state);
}

pub fn item_trigger(creator: Option<Entity>, item: Entity, targets: &Targets, state: &mut State) {
    //fires off effect
    event_trigger(creator, item, targets, state);

    //despawns entity if it was consumable
    if state.world.get::<&Consumable>(item).is_ok() {
        if state.world.despawn(item).is_err() {
            console::log("Couldn't despawn consumable item after use!");
        }
    }
}

pub fn event_message_trigger(e: Event, creator: Option<Entity>, listener: Entity) {
    //correctly select targets here
}

pub fn interact_trigger(
    creator: Option<Entity>,
    interactable: Entity,
    targets: &Targets,
    state: &mut State,
) {
    //fires off effect from interacting with machine
    state.game_log.add_log(String::from("Machine used!"));
    event_trigger(creator, interactable, targets, state);
}

///Activates effects caused by running console commands
///todo: currently only works for player targetted commands and doesn't take any arguments
pub fn command_trigger(command: crate::raws::scripting::Command, state: &mut State) {
    let mut builder = hecs::EntityBuilder::new();
    builder = RawMaster::add_effects_comps(builder, command.consumable.effects.clone());
    let ent = state.world.spawn(builder.build());

    if command.target == "self" {
        event_trigger(
            None,
            ent,
            &Targets::Single {
                target: state.player_ent.unwrap(),
            },
            state,
        );

        let _ = state.world.despawn(ent);
    }
}

#[allow(dead_code)]
pub fn entry_trigger_fire(
    creator: Option<Entity>,
    prop: Entity,
    targets: &Targets,
    state: &mut State,
) {
    state.game_log.add_log("Trap fired!".to_string());
    event_trigger(creator, prop, targets, state);
}

fn event_trigger(creator: Option<Entity>, item: Entity, targets: &Targets, state: &mut State) {
    //do .get on item for different Components and then execute relevant code you nerdd!!!!!!

    if let Ok(door) = state.world.get::<&Door>(item) {
        //attempts to toggle door state
        //
        //add_effect(creator, EffectType::ToggleDoor, targets);
    }

    if let Ok(status) = state.world.get::<&GrantsStatus>(item) {
        add_effect(
            Some(item),
            EffectType::StatusEffect {
                effects: status.effects.clone(),
                duration: status.duration,
            },
            Targets::Single {
                target: creator.unwrap(),
            },
        );
    }

    if let Ok(damage) = state.world.get::<&DamageEffect>(item) {
        add_effect(
            creator,
            EffectType::Damage {
                amount: damage.damage_amount,
            },
            targets.clone(),
        );
    }

    if let Ok(heal) = state.world.get::<&HealingEffect>(item) {
        add_effect(
            creator,
            EffectType::Healing {
                amount: heal.healing_amount,
            },
            targets.clone(),
        );
    }

    if let Ok(food) = state.world.get::<&GivesFood>(item) {
        add_effect(
            creator,
            EffectType::Feed {
                amount: food.amount,
            },
            targets.clone(),
        );
    }

    if let Ok(p) = state.world.get::<&ParticleBurst>(item) {
        add_effect(
            creator,
            EffectType::Particle {
                glyph: p.particle.glyph.clone(),
                fg: p.particle.fg,
                bg: p.particle.bg,
                lifetime: p.particle.lifetime,
            },
            targets.clone(),
        );
    }

    if let Ok(p) = state.world.get::<&DescendFloors>(item) {
        add_effect(
            creator,
            EffectType::PlayerDecendFloor {
                to_descend: p.num_floors,
            },
            targets.clone(),
        );
    }

    if let Ok(p) = state.world.get::<&ParticleLine>(item) {
        if let Some(source) = creator {
            let pl = &p.clone();

            if let Ok(source_pos) = state.world.get::<&Position>(source) {
                let start_pos = *source_pos;

                let mut end_pos = Point::zero();

                if let Targets::Tile { tile_idx } = *targets {
                    end_pos = Point::new(
                        tile_idx % state.map.map_width,
                        tile_idx / state.map.map_width,
                    );
                } else if let Targets::Tiles { tiles } = targets.clone() {
                    end_pos = Point::new(
                        tiles[0] % state.map.map_width,
                        tiles[0] / state.map.map_width,
                    );
                }
                if end_pos != Point::zero() {
                    //TODO: change this so that there is a staggered appearance and dissappearance of the particles!
                    let line = bracket_lib::geometry::Bresenham::new(
                        Point {
                            x: start_pos.x,
                            y: start_pos.y,
                        },
                        end_pos,
                    );
                    let tile_vec = line
                        .skip(1)
                        .map(|point| state.map.xy_idx(point.x, point.y) as i32)
                        .collect::<Vec<_>>();

                    add_effect(
                        creator,
                        EffectType::Particle {
                            glyph: pl.particle.glyph.clone(),
                            fg: pl.particle.fg,
                            bg: pl.particle.bg,
                            lifetime: pl.particle.lifetime,
                        },
                        Targets::Tiles {
                            tiles: tile_vec.clone(),
                        },
                    );
                }
            }
        }
    }

    if let Ok(p) = state.world.get::<&ParticleAnimation>(item) {
        if let Some(ent) = creator {
            if let Ok(pos) = state.world.get::<&Position>(ent) {
                let start_pos = Into::<Point>::into(*pos);
                let mut end_pos = Point::zero();

                if let Targets::Tile { tile_idx } = targets {
                    end_pos.x = tile_idx % state.map.map_width;
                    end_pos.y = tile_idx / state.map.map_width;
                }
                if let Targets::Tiles { tiles } = targets {
                    end_pos.x = tiles[0] % state.map.map_width;
                    end_pos.y = tiles[0] / state.map.map_width;
                }

                if end_pos != Point::zero() {
                    let path = bracket_lib::geometry::BresenhamInclusive::new(start_pos, end_pos)
                        .skip(1)
                        .collect::<Vec<_>>();

                    let anim = Animation {
                        step_time: p.particle.lifetime - 20.,
                        particle: p.particle.clone(),
                        path: path,
                        index: 0,
                        current_step_time: p.particle.lifetime - 20.,
                        creator: creator.expect("No projectile creator"),
                    };

                    //std::mem::drop(p);
                    //std::mem::drop(pos);
                    ANIMATIONQUEUE.lock().unwrap().push((
                        anim,
                        Projectile {
                            damage: state
                                .world
                                .get::<&RangedWeapon>(item)
                                .expect("Couldn't get RangedWeapon component!")
                                .damage,
                        },
                    ));
                    //return;
                }
            }
        }
    }
}
