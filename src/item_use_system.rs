
use crate::effects::{add_effect, get_aoe_tiles, EffectType, Targets};
use crate::{ AoE, Map,  State, WantsToUseItem};



pub fn run(state : &mut State)
{
    let mut entities_to_use_items = Vec::new();
    
    for (ent, item) in state.world.query_mut::<&WantsToUseItem>()
    {
        entities_to_use_items.push((ent, item.item, item.target));
    }

    for (entity, item, point) in entities_to_use_items.iter()
    {
    
        match point
        {
            Some(target) =>
            {
                match state.world.query_one_mut::<&AoE>(*item)
                {
                    Ok( aoe) => 
                    {
                        let range = aoe.radius;
                        
                        let tiles = get_aoe_tiles(state, range, *target);

                        add_effect(Some(*entity), EffectType::ItemUse { item: *item }, Targets::Tiles {tiles: tiles.clone() });
                    }

                    Err(_) =>
                    {
                        add_effect(Some(*entity), EffectType::ItemUse { item: *item },
                             Targets::Tile { tile_idx: Map::xy_id(target.x, target.y) as i32 });
                    }
                }
                
            }

            //self targeted
            None => 
            {
                add_effect(Some(*entity), EffectType::ItemUse { item: *item }, Targets::Single { target: *entity });
            }
            
        }

        state.world.remove_one::<WantsToUseItem>(*entity).unwrap();

        
    }

   


}