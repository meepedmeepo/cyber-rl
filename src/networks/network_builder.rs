use crate::State;

use super::{NetworkMap, Root, RootNode};




fn generate_network(state : &mut State)
{
    let root = state.world.spawn((RootNode{difficulty: 2}, Root{}));
    let mut res = NetworkMap::new(root);

    if res.is_err() {panic!("Couldn't create root node of network!");}

    state.network_map = res.unwrap();
}

fn spawn_rootnodes(state : &mut State)
{
    
}