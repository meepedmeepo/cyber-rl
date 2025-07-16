use crate::{
    components::{AIQuips, FoV, Name},
    State,
};

pub fn quip_system(state: &mut State) {
    let mut quipcomp_to_remove = Vec::new();
    for (ent, (quips, fov, name)) in state.world.query_mut::<(&mut AIQuips, &FoV, &Name)>() {
        if fov.visible_tiles.contains(&state.player_pos) && state.rng.roll_dice(1, 15) == 15 {
            let index;
            if quips.quips.len() == 0 {
                continue;
            } else if quips.quips.len() == 1 {
                index = 0;
            } else {
                index = state.rng.random_slice_index(&quips.quips).unwrap();
            }

            state.game_log.add_log(format!(
                "The {} says {}",
                name.name.clone(),
                quips.quips[index]
            ));

            quips.quips.swap_remove(index);

            if quips.quips.len() == 0 {
                quipcomp_to_remove.push(ent);
            }
        }
    }

    for ent in quipcomp_to_remove.iter() {
        let _ = state.world.remove_one::<AIQuips>(*ent);
    }
}
