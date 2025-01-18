use bracket_lib::prelude::{BTerm, Point};

use crate::{Map, State};

use super::{add_effect, Particle, ANIMATIONQUEUE};



pub struct Animation
{
    pub  particle : Particle,
    pub path : Vec<Point>,
    pub index : usize,
    pub step_time : f32,
    pub current_step_time : f32,
}

pub fn run_animation_queue(state : &mut State, ctx : &mut BTerm)
{
    let mut anim_to_delete = Vec::new();
    for (id, anim) in state.world.query_mut::<&mut Animation>()
    {
        anim.current_step_time -= ctx.frame_time_ms;

        if anim.current_step_time <= 0.
        {
            if anim.index < anim.path.len() - 1
            {
                anim.index += 1;
                anim.current_step_time = anim.step_time;

                let idx = Map::xy_id(anim.path[anim.index].x, anim.path[anim.index].y) as i32;

                add_effect(None, super::EffectType::Particle { glyph: anim.particle.glyph, fg: anim.particle.fg
                , bg: anim.particle.bg, lifetime: anim.particle.lifetime }
                , super::Targets::Tile { tile_idx: idx });

            }
            else 
            {
                anim_to_delete.push(id);
            }
        }
    }

    for ent in anim_to_delete.iter()
    {
        let _ = state.world.despawn(*ent);
    }
}