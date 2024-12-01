use super::{State,Map,Position,BlocksTiles};

pub struct MapIndexingSystem
{}

impl MapIndexingSystem
{
    pub fn run(state : &mut State)
    {
        state.map.populate_blocked();
        state.map.reset_tile_contents();

        for (_id,(pos, _blocks)) in state.world.query::<(&Position,Option<&BlocksTiles>)>().iter()
        {
            let idx = Map::xy_id(pos.x, pos.y);
            state.map.tile_contents[idx].push(_id);
            match _blocks
            {
                Some(_p) => {state.map.blocked[idx] = true;}
                None => {continue;}
            }
            
        }
    }
}