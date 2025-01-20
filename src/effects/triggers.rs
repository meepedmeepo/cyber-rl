use bracket_lib::prelude::{console, Point};
use hecs::Entity;

use crate::{gamelog, Consumable, DamageEffect, GivesFood, HealingEffect, Hidden, Map, Position, Projectile, RangedWeapon, State, MAPWIDTH};

use super::{add_effect, animation::Animation, EffectType, ParticleAnimation, ParticleBurst, ParticleLine, Targets, ANIMATIONQUEUE};


pub fn ranged_trigger(creator : Option<Entity>, item : Entity, targets : &Targets, state : &mut State)
{
    event_trigger(creator, item, targets, state);
}

pub fn item_trigger(creator : Option<Entity>, item : Entity, targets : &Targets, state : &mut State)
{
    //fires off effect
    event_trigger(creator, item, targets, state);

    //despawns entity if it was consumable
    if state.world.get::<&Consumable>(item).is_ok()
    {
        if state.world.despawn(item).is_err()
        {
            console::log("Couldn't despawn consumable item after use!");
        }
    }
}

#[allow(dead_code)]
pub fn entry_trigger_fire(creator : Option<Entity>, prop: Entity, targets : &Targets, state : &mut State)
{
    state.game_log.add_log("Trap fired!".to_string());
    event_trigger(creator, prop, targets, state);
}

fn event_trigger(creator : Option<Entity>, item : Entity, targets : &Targets, state : &mut State)
{
    //do .get on item for different Components and then execute relevant code you nerdd!!!!!!


    if let Ok(damage) = state.world.get::<&DamageEffect>(item)
    {
        add_effect(creator, EffectType::Damage { amount: damage.damage_amount }, targets.clone());
    }

    if let Ok(heal) = state.world.get::<&HealingEffect>(item)
    {
        add_effect(creator, EffectType::Healing { amount: heal.healing_amount }, targets.clone());
    }

    if let Ok(food) = state.world.get::<&GivesFood>(item)
    {
        add_effect(creator, EffectType::Feed { amount: food.amount}, targets.clone());
    }

    if let Ok(p) = state.world.get::<&ParticleBurst>(item)
    {
        add_effect(creator, EffectType::Particle { glyph: p.particle.glyph, fg: p.particle.fg
            , bg: p.particle.bg, lifetime: p.particle.lifetime }, targets.clone());
    }

    if let Ok(p) = state.world.get::<&ParticleLine>(item)
    {
        if let Some(source) = creator
        {
            let pl = *p;

            if let Ok(source_pos) = state.world.get::<&Position>( source)
            {
                let start_pos = *source_pos;

                let mut end_pos = Point::zero();

                if let Targets::Tile{tile_idx} = *targets
                {
                    end_pos = Point::new(tile_idx % MAPWIDTH, tile_idx / MAPWIDTH);
                }
                else if let Targets::Tiles { tiles } = targets.clone()
                {
                    end_pos = Point::new(tiles[0] % MAPWIDTH, tiles[0] / MAPWIDTH);
                }
                if end_pos != Point::zero()
                {
                    //TODO: change this so that there is a staggered appearance and dissappearance of the particles!
                    let line = bracket_lib::geometry::Bresenham::new(Point{x:start_pos.x,y: start_pos.y}, end_pos);
                    let tile_vec =line.skip(1).map(|point| Map::xy_id(point.x, point.y) as i32).collect::<Vec<_>>();

                    add_effect(creator, EffectType::Particle { glyph: pl.particle.glyph, fg: pl.particle.fg
                        , bg: pl.particle.bg, lifetime: pl.particle.lifetime }
                        , Targets::Tiles { tiles: tile_vec.clone() });
                }
            }
        }
    }

    if let Ok(p) = state.world.get::<&ParticleAnimation>(item)
    {
        if let Some(ent) = creator
        {
            if let Ok(pos) = state.world.get::<&Position>(ent)
            {
                let start_pos = Into::<Point>::into(*pos);
                let mut end_pos = Point::zero();

                if let Targets::Tile { tile_idx } = targets
                {
                    end_pos.x = tile_idx % MAPWIDTH;
                    end_pos.y = tile_idx / MAPWIDTH;
                }
                if let Targets::Tiles { tiles } = targets
                {
                    end_pos.x = tiles[0] % MAPWIDTH;
                    end_pos.y = tiles[0] / MAPWIDTH;
                }

                if end_pos != Point::zero()
                {


                    let path = bracket_lib::geometry::BresenhamInclusive::new(start_pos, end_pos).skip(1).collect::<Vec<_>>();

                    let anim = Animation{step_time: p.particle.lifetime-20., particle: p.particle.clone(), path: path,
                        index: 0, current_step_time : p.particle.lifetime-20., creator : creator.expect("No projectile creator") };
                    
                    //std::mem::drop(p);
                    //std::mem::drop(pos);
                    ANIMATIONQUEUE.lock().unwrap().push((anim, Projectile{damage:
                        state.world.get::<&RangedWeapon>(item)
                        .expect("Couldn't get RangedWeapon component!").damage}));
                    //return;
                }
            }
        }
    }

}