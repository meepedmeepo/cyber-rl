use crate::{
    effects::door::toggle_door, map_indexing::SPATIAL_INDEX, particles::ParticleBuilder,
    Projectile, State,
};
use bracket_lib::{color::RGB, prelude::FontCharType};
use damage::{heal_damage, inflict_damage};
use hecs::Entity;
use hunger::restore_hunger;
use movement::player_decend_floor;
use std::{collections::VecDeque, sync::Mutex};
use triggers::{
    command_trigger, entry_trigger_fire, interact_trigger, item_trigger, ranged_trigger,
};

mod animation;
mod damage;
mod door;
mod hunger;
mod movement;
mod particles;
mod targetting;
mod triggers;
pub use animation::*;
pub use particles::*;
pub use targetting::*;

lazy_static! {
    pub static ref EFFECTQUEUE: Mutex<VecDeque<EffectSpawner>> = Mutex::new(VecDeque::new());
    pub static ref ANIMATIONQUEUE: Mutex<Vec<(Animation, Projectile)>> = Mutex::new(Vec::new());
}

#[derive(Debug, PartialEq)]
pub enum EffectType {
    Damage {
        amount: i32,
    },
    Particle {
        glyph: String,
        fg: RGB,
        bg: RGB,
        lifetime: f32,
    },
    ItemUse {
        item: Entity,
    },
    InteractMachine {
        machine: Entity,
    },
    ConsoleCommand {
        command: crate::raws::scripting::Command,
    },
    Healing {
        amount: i32,
    },
    Feed {
        amount: i32,
    },
    ParticleLine {
        glyph: String,
        fg: RGB,
        bg: RGB,
        lifetime: f32,
    },
    PropTriggered {
        prop: Entity,
    },
    ParticleProjectile {
        glyph: String,
        fg: RGB,
        bg: RGB,
        lifetime: f32,
        step_time: f32,
    },
    RangedFire {
        item: Entity,
    },
    PlayerDecendFloor {
        to_descend: u32,
    },
    ToggleDoor,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Targets {
    Single { target: Entity },
    Area { targets: Vec<Entity> },
    Tile { tile_idx: i32 },
    Tiles { tiles: Vec<i32> },
    TargetList { targets: Vec<Entity> },
}

pub struct EffectSpawner {
    pub creator: Option<Entity>,
    pub effect_type: EffectType,
    pub targets: Targets,
}

pub fn add_effect(creator: Option<Entity>, effect_type: EffectType, targets: Targets) {
    EFFECTQUEUE.lock().unwrap().push_back(EffectSpawner {
        creator,
        effect_type,
        targets,
    });
}

pub fn run_effect_queue(state: &mut State) {
    loop {
        let effect = EFFECTQUEUE.lock().unwrap().pop_front();
        if let Some(effect) = effect {
            target_applicator(state, &effect);
        } else {
            break;
        }
    }
}

fn target_applicator(state: &mut State, effect: &EffectSpawner) {
    if let EffectType::ItemUse { item } = effect.effect_type {
        //items are handled seperately here because they can be consumable so it requires slightly different handling to despawn
        //after use

        item_trigger(effect.creator, item, &effect.targets, state);
    } else if let EffectType::InteractMachine { machine } = effect.effect_type {
        interact_trigger(effect.creator, machine, &effect.targets, state);
    } else if let EffectType::PropTriggered { prop } = effect.effect_type {
        entry_trigger_fire(effect.creator, prop, &effect.targets, state);
    } else if let EffectType::RangedFire { item } = effect.effect_type {
        ranged_trigger(effect.creator, item, &effect.targets, state);
    } else if let EffectType::ConsoleCommand { command } = &effect.effect_type {
        command_trigger(command.clone(), state);
    } else {
        match &effect.targets {
            Targets::Tile { tile_idx } => affect_tile(state, effect, *tile_idx),
            Targets::Tiles { tiles } => tiles
                .iter()
                .for_each(|tile_idx| affect_tile(state, effect, *tile_idx)),
            Targets::Single { target } => affect_entity(state, effect, *target),
            Targets::TargetList { targets } => targets
                .iter()
                .for_each(|target| affect_entity(state, effect, *target)),
            _ => todo!(),
        }
    }
}

fn tile_effect_hits_entities(effect: &EffectType) -> bool {
    match effect {
        EffectType::Damage { .. } => return true,
        EffectType::Healing { .. } => return true,
        EffectType::Feed { .. } => return true,
        _ => false,
    }
}

fn affect_tile(state: &mut State, effect: &EffectSpawner, tile_idx: i32) {
    if tile_effect_hits_entities(&effect.effect_type) {
        let contents = SPATIAL_INDEX
            .lock()
            .unwrap()
            .get_tile_contents(tile_idx as usize);

        contents
            .iter()
            .for_each(|target| affect_entity(state, effect, *target));
    }

    match &effect.effect_type {
        EffectType::Particle {
            glyph,
            fg,
            bg,
            lifetime,
        } => {
            spawn_particle(state, glyph.clone(), *fg, *bg, *lifetime, tile_idx);
        }
        _ => {}
    }
}

fn affect_entity(state: &mut State, effect: &EffectSpawner, target: Entity) {
    match effect.effect_type {
        EffectType::Damage { .. } => inflict_damage(state, effect, target),
        EffectType::Healing { .. } => heal_damage(state, effect, target),
        EffectType::Feed { .. } => restore_hunger(state, effect, target),
        EffectType::PlayerDecendFloor { .. } => player_decend_floor(state, effect),
        EffectType::ToggleDoor => toggle_door(state, effect, target),
        _ => {}
    }
}
