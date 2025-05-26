use crate::{go_down_stairs, ProgramState, State};

use super::{EffectSpawner, EffectType};

pub fn player_decend_floor(state: &mut State, effect: &EffectSpawner) {
    //
    if let EffectType::PlayerDecendFloor { to_descend } = effect.effect_type {
        //for now only will ever go down one floor ooofies
        state.current_state = ProgramState::Ticking;
        go_down_stairs(state)
    }
}
