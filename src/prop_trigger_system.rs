use crate::{State, Triggered};




pub fn run(state: &mut State)
{
    let triggered_props = state.world.query::<&Triggered>().iter().collect::<Vec<_>>();
    

}