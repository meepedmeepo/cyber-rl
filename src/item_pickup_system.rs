use bracket_lib::prelude::console;
use hecs::{Entity, World};
use crate::{Item, ItemContainer, Position, WantsToPickupItem};

use super::{State};

pub fn run(state : &mut State)
{
    let mut pickup_tags_to_remove: Vec<Entity> =  Vec::new();
    let mut items_to_pickup = Vec::new();
    
    for (ent,(pickup,container)) in state.world.query_mut::<(&WantsToPickupItem,Option<&ItemContainer>)>()
    {
  
        match container
        {
            Some(_p) => 
            {
                items_to_pickup.push((ent,pickup.item));
                pickup_tags_to_remove.push(ent);
            }
            None =>
            {
                console::log("Can't pickup item as entity doesn't have an ItemContainer component!");
                pickup_tags_to_remove.push(ent);
            }

        }
    }
    
    
    for (ent,pickup) in items_to_pickup.into_iter()
    {
        //state.world.get::<&ItemContainer>(item.0).expect("Couldn't get ItemContainer component of an entity that should have one!").items.push(item.1.item);
        let container = state.world.query_one_mut::<&mut ItemContainer>(ent).expect("Couldn't get ItemContainer component of an entity that should have one");
        container.items.push(pickup);
        state.world.remove_one::<Position>(pickup).expect("Couldn't remove Position component from item entity.");
        //console::log("Item picked up!");
    }
    
    
    for tags in pickup_tags_to_remove.iter()
    {
        state.world.remove_one::<WantsToPickupItem>(*tags).expect("Couldn't remove WantsToPickupItem component from entity!");
    }

    
    }

