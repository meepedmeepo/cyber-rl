use bracket_lib::prelude::console;
use hecs::Entity;

use crate::{Consumable, State};

use super::Targets;




pub fn item_trigger(creator : Option<Entity>, item : Entity, targets : &Targets, state : &mut State)
{
    //fires off effect
    event_trigger(creator, item, targets, state);

    //despawns entity if it was consumable
    if state.world.get::<&Consumable>(item).is_ok()
    {
        if state.world.despawn(item).is_err()
        {
            console::log("Couldn't despawn consumable item after use!");
        }
    }
}

fn event_trigger(creator : Option<Entity>, item : Entity, targets : &Targets, state : &mut State)
{
    //do .get on item for different Components and then execute relevant code you nerdd!!!!!!1
}