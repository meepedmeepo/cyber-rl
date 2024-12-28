use crate::{ Equippable, Equipped, State};



pub fn run(state :&mut State)
{
    // let mut stats_to_calc = Vec::new();
    // for (_id,(stats,cstats)) 
    //     in state.world.query::<(&Statistics,&CombatStats)>().iter()
    //     .filter(|ent| ent.1.1.is_dirty())
    // {
    //     stats_to_calc.push((_id,*cstats));
    // }

    // for (ent, cstats) in stats_to_calc.iter_mut()
    // {
    //     cstats.defence.bonuses = 0;
    //     cstats.power.bonuses = 0;

    //     for (_id, (_equipped,equippable)) 
    //         in state.world.query::<(&Equipped,&Equippable)>()
    //         .iter().filter(|item|item.1.0.owner == *ent)
    //     {
    //         cstats.defence.bonuses += equippable.defence_bonus;
    //         cstats.power.bonuses += equippable.power_bonus;
    //     }

    //     cstats.defence.total = cstats.defence.base + cstats.defence.bonuses;
    //     cstats.power.total = cstats.power.base + cstats.power.bonuses;
    //     cstats.defence.dirty = false;
    //     cstats.power.dirty = false;
    // }

    // for (ent, cstats) in stats_to_calc.iter()
    // {
    //     state.world.insert_one(*ent, *cstats)
    //         .expect("Couldn't insert new CombatStats component");
    // }
}