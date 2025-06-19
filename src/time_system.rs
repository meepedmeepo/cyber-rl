use crate::{
    components::{EffectDuration, StatusEffect},
    State,
};

pub fn time_system(state: &mut State) {
    state.turn_number += 1;

    let mut effects_to_despawn = Vec::new();

    for (ent, (status, duration)) in state
        .world
        .query_mut::<(&StatusEffect, &mut EffectDuration)>()
    {
        duration.rounds -= 1;

        if duration.rounds <= 0 {
            effects_to_despawn.push((ent, status.target));
        }
    }

    for (effect, _target) in effects_to_despawn.iter() {
        let _ = state.world.despawn(*effect);
    }
}
