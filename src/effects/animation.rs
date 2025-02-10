use bracket_lib::prelude::{BTerm, Point};
use hecs::Entity;
use macroquad::time::get_frame_time;

use crate::{projectile::{projectile_system::*, ProjectileType}, Creator, Hidden, Map, ProgramState, Projectile, State};

use super::{add_effect, Particle, ANIMATIONQUEUE};


#[derive(Clone, PartialEq)]
pub struct Animation
{
    pub particle : Particle,
    pub path : Vec<Point>,
    pub index : usize,
    pub step_time : f32,
    pub current_step_time : f32,
    pub creator : Entity,
}

pub fn run_animation_queue(state : &mut State)
{
    //spawns animations added from effect queue
    for (anim, ranged) in ANIMATIONQUEUE.lock().unwrap().iter()
    {
        state.world.spawn((anim.clone(), ProjectileType::Missile, ProjectileUpdated{}, Projectile{damage: ranged.damage}));
    }
    //clears animation queue list
    ANIMATIONQUEUE.lock().unwrap().clear();

    let mut anim_to_delete = Vec::new();
    let mut proj_to_update = Vec::new();
    //updates animations in the animation queue
    for (id, anim) in state.world.query_mut::<&mut Animation>()
    {
        anim.current_step_time -= get_frame_time()*1000.;

        if anim.current_step_time < 0.
        {
            if anim.index < anim.path.len() - 1
            {
                proj_to_update.push(id);
                anim.index += 1;
                anim.current_step_time = anim.step_time;
                let idx = state.map.xy_idx(anim.path[anim.index].x, anim.path[anim.index].y) as i32;

                add_effect(None, super::EffectType::Particle { glyph: anim.particle.glyph.clone(), fg: anim.particle.fg
                , bg: anim.particle.bg, lifetime: anim.particle.lifetime+25. }
                , super::Targets::Tile { tile_idx: idx });
            }
            else 
            {
                anim_to_delete.push(id);
            }
        }
    }
    for proj in proj_to_update.iter()
    {
        let _ = state.world.insert_one(*proj, ProjectileUpdated{});
    }
    for ent in anim_to_delete.iter()
    {
        let _ = state.world.despawn(*ent);
    }

    projectile_system(state);

    let mut q = state.world.query::<&Animation>();
    let query = q.into_iter().collect::<Vec<_>>();
    
    if query.len() > 0
    {
        state.current_state = ProgramState::PlayAnimation;
    }
    else 
    {
        state.current_state = ProgramState::Ticking;    
    }
}