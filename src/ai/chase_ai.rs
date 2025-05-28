use bracket_lib::prelude::Point;
use hecs::Entity;

use crate::{statistics::BaseStatistics, Position, State};

use super::MyTurn;

pub struct InCombat {
    pub target: Entity,
}

pub struct Chasing {
    pub target: Entity,
}

pub struct LastKnownPosition {
    pub pos: Point,
}

pub fn chase_ai_system(state: &mut State) {
    let mut turn_done = Vec::new();
    let mut chases_to_initiate = Vec::new();

    for (entity, (_, combat, pos, stats, chase)) in state.world.query_mut::<(
        &MyTurn,
        &InCombat,
        &mut Position,
        &BaseStatistics,
        Option<&Chasing>,
    )>() {
        //

        if let Some(chasing) = chase {
            //continue chasing

            turn_done.push(entity);
        } else {
            //initiate a chase
            chases_to_initiate.push((entity, combat.target));
        }
    }

    for (ent, target) in chases_to_initiate.iter() {
        let _ = state.world.insert_one(*ent, Chasing { target: *target });
    }

    //ends turn for all enemies that made a movement to chase
    for ent in turn_done.iter() {
        let _ = state.world.remove_one::<MyTurn>(*ent);
    }
}
