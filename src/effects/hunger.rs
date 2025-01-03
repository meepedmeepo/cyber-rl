use bracket_lib::color::{BLACK, ORANGE, RGB};
use hecs::Entity;

use crate::{hunger::HungerLevel, State};

use super::{add_effect, entity_position, EffectSpawner, EffectType, Targets};

pub fn restore_hunger(state : &mut State, feed : &EffectSpawner, target: Entity)
{
    let query = state.world.query_one_mut::<&mut HungerLevel>(target);
    
    match query
    {
        Ok(pools) => 
        {
            
            if let EffectType::Feed{amount} = feed.effect_type
            {
                pools.nutrition.restore(amount);

                add_effect(None,
                     EffectType::Particle { glyph: bracket_lib::terminal::to_char(227 as u8),//glyph is π
                      fg: RGB::named(BLACK), bg: RGB::named(ORANGE),
                      lifetime: 200. }, Targets::Tile{tile_idx : entity_position(state, target).unwrap()});
            }
            
            
        }
        Err(_) => {}
    }
}