use hecs::Entity;

use crate::{
    effects::{EffectSpawner, EffectType},
    spawns::spawning_system::spawn_effect_entity,
    State,
};

pub fn spawn_status_effect(state: &mut State, effect: &EffectSpawner, target: Entity) {
    //
    if let EffectType::StatusEffect { effects, duration } = effect.effect_type.clone() {
        spawn_effect_entity(state, effects, duration, effect.creator.unwrap(), target);
    }
}
