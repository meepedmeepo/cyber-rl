use bracket_lib::prelude::{BTerm, Point};
use queues::{queue, Queue, IsQueue};

use crate::{damage_system::DamageSystem, Position, Renderable, State};

use super::{Projectile, ProjectileType};




pub fn spawn_projectiles(state : &mut State)
{
    for projectile in state.projectile_builder.requests.iter()
    {
        //let proj = 
            //Projectile{frame_time: projectile.frame_time, path: projectile.path.}
        let mut path = projectile.path.clone();
        //path.reverse();

        let mut projectile_path: Queue<Point> = queue![];
        for p in path.iter()
        {
            projectile_path.add(p.clone()).unwrap();
        }

        state.world.spawn((Projectile{frame_time : projectile.frame_time, current_frame_time: projectile.frame_time,
            path: projectile_path, dmg : projectile.dmg}, 
            Renderable::new(projectile.glyph, projectile.fg, projectile.bg, projectile.order), projectile.projectile_type));

        //let queue: Queue<Point> = queue![projectile.path.];
    }

    state.projectile_builder.requests.clear();
}


pub fn update_projectiles(state : &mut State, ctx: &mut BTerm)
{
    let mut projectiles_to_update = Vec::new();
    let mut projectiles_to_despawn = Vec::new();
    for (ent, (projectile, projectile_type)) in 
        state.world.query_mut::<(&mut Projectile, &ProjectileType)>()
    {
        projectile.current_frame_time -= ctx.frame_time_ms;
        
        if projectile.current_frame_time < 0.
        {
            match projectile.path.peek()
            {
                Ok(_) =>
                {
                    
                    projectile.current_frame_time = projectile.frame_time;
                    //fix this crashing by unwrapping null -- change above to peek and only remove if not last??
                    projectiles_to_update.push((ent, projectile.path.peek().unwrap(), *projectile_type, projectile.dmg));
                    match projectile.path.remove()
                    {
                        Ok(_) =>{}
                        Err(_) =>
                        {
                            projectiles_to_despawn.push(ent);
                        }
                    }
                }
                Err(_) =>
                {
                    projectiles_to_despawn.push(ent);
                    continue;
                }
            }
        }
    }


    for (ent, new_location, proj_type, dmg) in projectiles_to_update.iter()
    {
       let _proj = state.world
            .insert_one(*ent, Position{ x: new_location.x, y: new_location.y });

        let hits = state.map.get_mob_entities_at_position(state, *new_location);

        match proj_type
        {
            ProjectileType::Beam => {}
            ProjectileType::Missile =>
            {
                if hits.len() > 0
                {
                    projectiles_to_despawn.push(*ent);
                }

            }
            _ => {}
        }

        for target in hits.iter()
        {
            DamageSystem::mark_for_damage(state, *target, *dmg);
        }

    }


    for ent in projectiles_to_despawn.iter()
    {
        state.world.despawn(*ent).expect("Couldn't despawn projectile entity!");
    }


}

