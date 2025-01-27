use bracket_lib::{color::{BLACK, GREEN, RED, RGB}, prelude::{console, to_cp437}};
use hecs::Entity;

use crate::{statistics::Pools, Name};

use super::{add_effect, entity_position, EffectSpawner, EffectType, State, Targets};


pub fn inflict_damage(state : &mut State, damage : &EffectSpawner, target: Entity)
{
    let query =state.world.query_one_mut::<(&mut Pools, &Name)>(target);

    let mut ent_name = "".to_string();
    let mut dmg_num = 0;
    
    match query
    {
        Ok((pools, name)) => 
        {
            if let EffectType::Damage{amount} = damage.effect_type
            {

                pools.hitpoints.damage(amount);

                ent_name = name.name.clone();
                dmg_num = amount;

                add_effect(None,
                    EffectType::Particle { glyph: to_cp437('!'),
                    fg: RGB::named(BLACK), bg: RGB::named(RED),
                    lifetime: 200. }, Targets::Tile{tile_idx : entity_position(state, target).unwrap()});
            }
        }
        Err(_) => {}
    }
    if ent_name != ""
    {
        let msg = format!("{} took {} damage!",ent_name, dmg_num);
        state.game_log.add_log(msg.clone());
        console::log(msg);
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
                    EffectType::Particle { glyph: 2665u16,//glyph should be changed to ♥︎
                    fg: RGB::named(BLACK), bg: RGB::named(GREEN),
                    lifetime: 200. }, Targets::Tile{tile_idx : entity_position(state, target).unwrap()});
            }
        }
        Err(_) => {}
    }
}