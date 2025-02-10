use bracket_lib::prelude::{BTerm, Point};
use hecs::Entity;
use macroquad::time::get_frame_time;

use crate::{Position, Renderable, State};

use super::{ParticleFollowEntity, ParticleLifetime};



pub fn spawn_system(state: &mut State)
{

    for particle in state.particle_builder.requests.iter()
    {
        let p = 
            state.world.spawn((Position{x : particle.x, y : particle.y},
            Renderable{glyph : particle.glyph.clone(), fg : particle.fg, bg : particle.bg, order: 7},
            ParticleLifetime{lifetime : particle.lifetime}));
        match particle.target
        {
            Some(target) => 
            {
                state.world.insert_one(p, ParticleFollowEntity{target})
                    .expect("Couldn't insert ParticleFollowEntity component onto particle");
            }
            None =>{}
        }
    }
    state.particle_builder.requests.clear();
}

pub fn update(state: &mut State)
{
    let mut particles_to_update_position : Vec<(Entity, Entity, Point)> = Vec::new();
    let mut particles_to_despawn : Vec<Entity> = Vec::new();
    for (particle, (lifetime, target)) in
        state.world.query_mut::<(&mut ParticleLifetime, Option<&ParticleFollowEntity>)>()
    {
        let mut culled = false;
        lifetime.lifetime -= get_frame_time()*1000.;

        if lifetime.lifetime < 0.
        {
            particles_to_despawn.push(particle);
            culled = true;
        }
        if !culled
        {
            match target
            {
                Some(ent) =>
                {
                    particles_to_update_position.push((particle, ent.target, Point::zero()));
                }
                None => {}
            }
        }
    }

    for (particle, target, pos) in particles_to_update_position.iter_mut()
    {
        {
            let query = state.world.query_one_mut::<&Position>(*target);

            match query
            {
                Ok(point) =>
                {
                    pos.x = point.x;
                    pos.y = point.y;
                }
                Err(_) => 
                {
                    particles_to_despawn.push(*particle);
                    continue;
                }
            }
        }

        let particle_pos = 
            state.world.query_one_mut::<&mut Position>(*particle)
            .expect("Couldn't get particle position to update");
        
        particle_pos.x = pos.x;
        particle_pos.y = pos.y; 
    }

    for particle in particles_to_despawn.iter()
    {
        state.world.despawn(*particle).expect("Couldn't despawn particle!");
    }
}