use bracket_lib::prelude::BTerm;
use hecs::Entity;

use crate::{Position, Renderable, State};

use super::ParticleLifetime;



pub fn spawn_system(state: &mut State)
{

    for particle in state.particle_builder.requests.iter()
    {
        let _p = 
            state.world.spawn((Position{x : particle.x, y : particle.y},
            Renderable{glyph : particle.glyph, fg : particle.fg, bg : particle.bg, order: 7},
            ParticleLifetime{lifetime : particle.lifetime}));
    }
    state.particle_builder.requests.clear();
}

pub fn update(state: &mut State, ctx : &mut BTerm)
{
    let mut particles_to_despawn : Vec<Entity> = Vec::new();
    for (particle, lifetime) in state.world.query_mut::<&mut ParticleLifetime>()
    {
        lifetime.lifetime -= ctx.frame_time_ms;

        if lifetime.lifetime < 0.
        {
            particles_to_despawn.push(particle);
        }
    }

    for particle in particles_to_despawn.iter()
    {
        state.world.despawn(*particle).expect("Couldn't despawn particle!");
    }
}