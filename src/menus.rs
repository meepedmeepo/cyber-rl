use bracket_lib::prelude::BTerm;
use bracket_lib::terminal::console;
use hecs::Entity;

use crate::{ItemContainer, State, WantsToUseItem};

pub struct InventoryMenu
{}
pub enum inventory_state
{
    Cancel,
    Selected,
    None,
}
impl InventoryMenu
{
    pub fn menu_input(ctx : &BTerm, state : &mut State) -> inventory_state
    {
        match ctx.key
        {
            Some(key) =>
            {	
				match key
				{
				bracket_lib::terminal::VirtualKeyCode::Escape => {return inventory_state::Cancel;}
				
				_ =>
				{
				let mut item_target : Option<Entity> = None;
                //TODO: add
				let item_list = 
				state.world.query_one_mut::<&ItemContainer>(state.player_ent
					.expect("Couldn't find player to query inventory!"))
					.expect("Couldn't find player ItemContainer to query inventory!");
				match item_list.items.get(bracket_lib::terminal::letter_to_option(key) as usize)
				{
					Some(p) => { item_target = Some(*p); }

					None => 
					{
						console::log("Invalid inventory menu selection!");
						return inventory_state::None;
					}
				}
				match item_target
				{
					Some(selected_item) =>
					{
						state.world.insert_one(state.player_ent
							.expect("Couldn't find player to insert WantsToUseItem component!"),
							 WantsToUseItem {item: selected_item})
							 .expect("Couldn't insert WantsToUseItem onto player!");
						return inventory_state::Selected;
					}
					None =>
					{
						console::log("Key doesn't correspond with any item in inventory!");
						return inventory_state::None;
					}
				}
			}
			}
            }
            None => {return inventory_state::None;}
        }

        inventory_state::None
    }

}