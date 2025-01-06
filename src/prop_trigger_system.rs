use bracket_lib::prelude::Point;

use crate::{effects::{self, add_effect, EffectType, Targets}, AoE, Hidden, SingleActivation, State, Trigger, Triggered, MAPWIDTH};




pub fn run(state: &mut State)
{
    let mut query = state.world.query::<&Triggered>();
    
    let triggered_props = query.into_iter()
        .map(|(ent,trig)| (ent, *trig)).collect::<Vec<_>>();
    
    std::mem::drop(query);

    for (prop, target) in triggered_props.iter()
    {
        if state.world.get::<&SingleActivation>(*prop).is_ok()
        {
            let _ = state.world.remove_one::<Trigger>(*prop);
        }

        if let Ok(aoe) = state.world.query_one_mut::<&AoE>(*prop)
        {
            let radius = aoe.radius;

            let _ = aoe;

            add_effect(Some(*prop), EffectType::PropTriggered { prop: *prop },
                Targets::Tiles { tiles: effects::get_aoe_tiles(state, radius,
                Point::new(target.idx % MAPWIDTH, target.idx / MAPWIDTH)) });

        } else
        {
            add_effect(Some(*prop), EffectType::PropTriggered { prop: *prop }, Targets::Tile { tile_idx: target.idx });
        }

        let _ = state.world.remove_one::<Triggered>(*prop);
        //let _ = state.world.remove_one::<Hidden>(*prop);
    }

}