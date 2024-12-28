use crate::{ statistics::Pools, EquipmentDirty, Equippable, Equipped, State, Wearable};



pub fn run(state :&mut State)
{
    let mut ents_to_recalculate = Vec::new();
    for (ent, _eq) in state.world.query_mut::<&EquipmentDirty>()
    {
        ents_to_recalculate.push(ent);
    }

    for ent in ents_to_recalculate.iter()
    {
        let mut ac_bonus = 0;
        let armour = state.world.query::<(&Equipped,&Wearable)>()
            .iter().filter(|(_id,(eq, wearable))| eq.owner == *ent)
            .for_each(|(_id,(eq, wearable))|
        {
            ac_bonus+= wearable.ac_bonus;
        });
        {
        let pool =state.world.query_one_mut::<&mut Pools>(*ent).expect("Couldn't find Pools for entity to recalculate
            equipment ac bonus");
        
        pool.armour_class.bonuses = ac_bonus;

        pool.armour_class.total = pool.armour_class.base + pool.armour_class.bonuses;
        }

        state.world.remove_one::<EquipmentDirty>(*ent)
            .expect("Couldn't remove EquipmentDirty from entity");

    }
}