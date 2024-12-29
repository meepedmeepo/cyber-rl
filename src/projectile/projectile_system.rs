use bracket_lib::prelude::{console, BTerm, Point};
use queues::{queue, Queue, IsQueue};

use crate::{damage_system::DamageSystem, statistics::BaseStatistics, Position, Renderable, State, Name};

use super::{Projectile, ProjectileType};




pub fn spawn_projectiles(state : &mut State)
{
    for projectile in state.projectile_builder.requests.iter()
    {
        //let proj = 
            //Projectile{frame_time: projectile.frame_time, path: projectile.path.}
        let path = projectile.path.clone();
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
                

            }
            _ => {}
        }
        for target in hits.iter()
        {
            let query= 
                state.world.query_one_mut::<(&Name, &BaseStatistics)>(*target)
                .expect("Couldn't get name or stats of entity attempting to dodge missile!");

            let name = query.0.clone();
            let stats = *query.1;

            let mut roll = state.rng.roll_dice(1, 20);
            roll += stats.dexterity.get_modifier();

            if roll < 15
            {
                DamageSystem::mark_for_damage(state, *target, *dmg);
                
                let msg = format!("{} was hit by missile",name.name.clone());
                console::log(msg.clone());
                state.game_log.add_log(msg);

                if *proj_type == ProjectileType::Missile
                {
                    projectiles_to_despawn.push(*ent);
                }
            }
            else
            {
                let msg = format!("{} dodged missile!", name.name.clone());
                console::log(msg.clone());
                state.game_log.add_log(msg);
            }
        }

    }


    for ent in projectiles_to_despawn.iter()
    {
        state.world.despawn(*ent).expect("Couldn't despawn projectile entity!");
    }


}

