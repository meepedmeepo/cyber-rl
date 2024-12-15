use bracket_lib::prelude::BTerm;
use bracket_lib::terminal::console;
use hecs::Entity;

use crate::{Equippable, InContainer, Item, Name, ProgramState, RangedTargetting, State, WantsToEquipItem, WantsToUseItem};

pub struct InventoryMenu
{}
pub enum inventory_state
{
    Cancel,
    Selected,
    None,
	TargetedItem {item : Entity, range: i32},
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
				// let item_list = 
				// state.world.query_one_mut::<&ItemContainer>(state.player_ent
				// 	.expect("Couldn't find player to query inventory!"))
				// 	.expect("Couldn't find player ItemContainer to query inventory!");
				let mut item_list = Vec::new();
				for (_id,(_item, _in_container, _name)) in state.world.query::<(&Item, &InContainer,&Name)>()
        			.iter().filter(|ent| ent.1.1.owner == state.player_ent
        			.expect("Couldn't find player entity to query inventory"))
				{
					item_list.push(_id);
				}
				match item_list.get(bracket_lib::terminal::letter_to_option(key) as usize)
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
						let is_ranged = 
						state.world.get::<&RangedTargetting>(
							Option::expect(item_target, "Couldn't find item target!"));
						match is_ranged
						{
							Ok(ref ranged) =>
							 {
								return inventory_state::TargetedItem { item: item_target
									.expect("Couldn't find item target!")
									, range: ranged.range };
							 }
							Err(_) =>
							{
								std::mem::drop(is_ranged);
								let is_equippable = 
									state.world.get::<&Equippable>(selected_item);
								match is_equippable
								{
									Ok(ref equip) =>
									{
										let slot = equip.slot;
										
										std::mem::drop(is_equippable);

										state.world.insert_one(state.player_ent
											.expect("Couldn't find player to insert WantsToEquipItem component"),
										 	WantsToEquipItem{item : selected_item, slot : slot})
											.expect("Couldn't insert WantsToEquipItem onto player entity!");
										
										return inventory_state::Selected;
									}
									Err(_) =>
									{
										std::mem::drop(is_equippable);

										state.world.insert_one(state.player_ent
											.expect("Couldn't find player to insert WantsToUseItem component!"),
											 WantsToUseItem {item: selected_item, target: None})
											 .expect("Couldn't insert WantsToUseItem onto player!");
										return inventory_state::Selected;
									}
								}
								
							}
						}
						
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

        //inventory_state::None
    }

}