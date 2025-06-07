use hecs::Entity;

use crate::{
    components::{Door, Password, PasswordProtected},
    effects::EffectSpawner,
    interaction::{close_door, open_door},
    State,
};

pub fn toggle_door(state: &mut State, effect: &EffectSpawner, target: Entity) {
    let is_open;
    {
        let mut query = state
            .world
            .query_one::<(&Door, Option<&PasswordProtected>)>(target)
            .unwrap();

        let (door, locked) = query.get().unwrap();

        is_open = door.open;

        if locked.is_some() {
            let mut query = state.world.query::<&Password>();

            let res = query
                .iter()
                .filter(|(_, pass)| {
                    pass.controls_target == target && pass.owner == effect.creator.unwrap()
                })
                .next();

            if res.is_none() {
                state
                    .game_log
                    .add_log(String::from("Door is tightly locked."));
                return;
            }
        }
    }

    if is_open {
        close_door(state, effect.creator.unwrap(), target);
    } else {
        open_door(state, effect.creator.unwrap(), target);
    }
}
