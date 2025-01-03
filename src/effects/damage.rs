use bracket_lib::color::{BLACK, GREEN, RED, RGB};
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

pub fn heal_damage(state : &mut State, heal : &EffectSpawner, target: Entity)
{
    let query =state.world.query_one_mut::<&mut Pools>(target);
    
    match query
    {
        Ok(pools) => 
        {
            
            if let EffectType::Healing{amount} = heal.effect_type
            {
                pools.hitpoints.restore(amount);

                add_effect(None,
                     EffectType::Particle { glyph: bracket_lib::terminal::to_char(3 as u8),//glyph is ♥︎
                      fg: RGB::named(BLACK), bg: RGB::named(GREEN),
                      lifetime: 200. }, Targets::Tile{tile_idx : entity_position(state, target).unwrap()});
            }
            
            
        }
        Err(_) => {}
    }
}