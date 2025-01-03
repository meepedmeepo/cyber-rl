use bracket_lib::color::{BLACK, RED, RGB};
use hecs::Entity;

use crate::statistics::Pools;

use super::{add_effect, entity_position, EffectSpawner, EffectType, State, Targets};


pub fn inflict_damage(state : &mut State, damage : &EffectSpawner, target: Entity)
{
    let query =state.world.query_one_mut::<&mut Pools>(target);
    
    match query
    {
        Ok(pools) => 
        {
            
            if let EffectType::Damage{amount} = damage.effect_type
            {
                pools.hitpoints.damage(amount);

                add_effect(None,
                     EffectType::Particle { glyph: '!',
                      fg: RGB::named(BLACK), bg: RGB::named(RED),
                      lifetime: 200. }, Targets::Tile{tile_idx : entity_position(state, target).unwrap()});
            }
            
            
        }
        Err(_) => {}
    }
}