use std::arch::x86_64;

use crate::{networks::{ControlNode, NodeOwned}, BlocksVisibility, Map, Player, State};


use super::{FoV, Position};
use bracket_lib::prelude::{field_of_view,Point};
//use hecs::World;
pub struct VisibilitySystem
{}

impl VisibilitySystem
{
    pub fn run(state: &mut State)
    {
        state.map.view_blocked.clear();
        for (id,(block_pos, _block)) in state.world.query_mut::<(&Position, &BlocksVisibility)>()
        {
            let idx = state.map.xy_idx(block_pos.x, block_pos.y);
            state.map.view_blocked.insert(idx);
        }

        for(_id ,(fov,pos, player )) in state.world.query_mut::<(&mut FoV,&Position, Option<&Player>)>()
        {
            if fov.dirty
            {
                fov.dirty = false;
                fov.visible_tiles.clear();
                fov.visible_tiles = field_of_view(Point::new(pos.x,pos.y), fov.range, &state.map);
                fov.visible_tiles.retain(|p| p.x >= 0 && p.x < state.map.map_width && p.y >= 0 && p.y < state.map.map_height );

            //let p: Option<&Player> = state.world.entity(_id).
            match player
            {
                Some(_p) =>
                {
                    for rev in state.map.visible_tiles.iter_mut(){*rev = false;}
                    for vis in fov.visible_tiles.iter() 
                    {
                        let idx = state.map.xy_idx(vis.x, vis.y);
                        state.map.visible_tiles[idx] = true;
                        state.map.revealed_tiles[idx] = true;
                    }
                },
                None => continue,

            }
            }
        }

        for (_id, (_node, fov, _owned)) in 
            state.world.query_mut::<(&ControlNode, &FoV, &NodeOwned)>().into_iter()
            .filter(|(_id,(_node, _fov, owned))| owned.owner == state.player_ent.unwrap())
        {
            for tile in fov.visible_tiles.iter()
            {
                let idx = state.map.xy_idx(tile.x, tile.y);

                state.map.visible_tiles[idx] = true;
                state.map.revealed_tiles[idx] = true;
            }
        }
    }
}
        
    

