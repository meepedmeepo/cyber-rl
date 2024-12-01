use super::{State,Map,Position,BlocksTiles};

pub struct MapIndexingSystem
{}

impl MapIndexingSystem
{
    pub fn run(state : &mut State)
    {
        state.map.populate_blocked();

        for (_id,(pos, _blocks)) in state.world.query::<(&Position,&BlocksTiles)>().iter()
        {
            let idx = Map::xy_id(pos.x, pos.y);
            state.map.blocked[idx] = true;
        }
    }
}