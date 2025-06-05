use bracket_lib::prelude::console;

use crate::{map_indexing::SPATIAL_INDEX, Trigger};

use super::{BlocksTiles, Position, State};

pub struct MapIndexingSystem {}

impl MapIndexingSystem {
    pub fn run(state: &mut State) {
        let mut spatial_map = SPATIAL_INDEX.lock().unwrap();

        spatial_map.reset(state.map.populate_blocked());

        for (id, (pos, blocks, trig)) in state
            .world
            .query::<(&Position, Option<&BlocksTiles>, Option<&Trigger>)>()
            .iter()
        {
            let idx = state.map.xy_idx(pos.x, pos.y);
            spatial_map.add_tile_content(idx, id);
            match blocks {
                Some(_) => {
                    spatial_map.set_tile_blocked_by_entity(idx);
                }
                None => {}
            }

            match trig {
                Some(_) => {
                    spatial_map.insert_prop(idx as i32, id);
                    //console::log(format!("after insertion {} on map",state.map.props.len()));
                }
                None => {}
            }
        }
    }
}
