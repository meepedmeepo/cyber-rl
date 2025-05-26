use crate::{Player, State};

use super::MyTurn;

pub fn default_move_ai_system(state: &mut State) {
    let mut turn_done = Vec::new();

    for (ent, _turn) in state.world.query_mut::<&MyTurn>().without::<&Player>() {
        turn_done.push(ent);
    }

    for done in turn_done.iter() {
        let _ = state.world.remove_one::<MyTurn>(*done);
    }
}
