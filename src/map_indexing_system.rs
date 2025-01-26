use bracket_lib::prelude::console;

use crate::Trigger;

use super::{State,Map,Position,BlocksTiles};

pub struct MapIndexingSystem
{}

impl MapIndexingSystem
{
    pub fn run(state : &mut State)
    {
        state.map.populate_blocked();
        state.map.reset_tile_contents();
        state.map.props.clear();

        for (_id,(pos, _blocks,trig)) in 
            state.world.query::<(&Position,Option<&BlocksTiles>, Option<&Trigger>)>().iter()
        {
            let idx = state.map.xy_idx(pos.x, pos.y);
            state.map.tile_contents[idx].push(_id);
            match _blocks
            {
                Some(_p) => {state.map.blocked[idx] = true;}
                None => {}
            }

            match trig
            {
                Some(_) =>
                {
                    state.map.props.insert(idx as i32, _id);
                    //console::log(format!("after insertion {} on map",state.map.props.len()));
                }
                None => {}
            }  
        }
    }
}