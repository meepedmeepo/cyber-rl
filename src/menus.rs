use std::i32;

use bracket_lib::prelude::{BTerm, VirtualKeyCode};
use bracket_lib::terminal::console;
use hecs::Entity;
use macroquad::input::{get_last_key_pressed, KeyCode};

use crate::gui::draw_pickup_menu;
use crate::{Equippable, InContainer, Item, Map, Name, ProgramState, RangedTargetting, State, WantsToEquipItem, WantsToPickupItem, WantsToUseItem};

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
    pub fn menu_input( state : &mut State) -> inventory_state
    {
        match get_last_key_pressed()
        {
            Some(key) =>
            {	
				match key
				{
					KeyCode::Escape => {return inventory_state::Cancel;}
				_ =>
				{
				let mut item_target : Option<Entity> = None;
				let mut item_list = Vec::new();

				for (_id,(_item, _in_container, _name)) in
					state.world.query::<(&Item, &InContainer,&Name)>()
        				.iter().filter(|ent| ent.1.1.owner == state.player_ent
        		    	.expect("Couldn't find player entity to query inventory"))
				{
					item_list.push(_id);
				}

				match item_list.get(key_to_option(key) as usize)
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

pub fn key_to_option(key : KeyCode) -> i32
{
	match key
	{
		KeyCode::A => 0,
		KeyCode::B => 1,
		KeyCode::C => 2,
		KeyCode::D => 3,
		KeyCode::E => 4,
		KeyCode::F => 5,
		KeyCode::G => 6,
		KeyCode::H => 7,
		KeyCode::I => 8,
		KeyCode::J => 9,
		KeyCode::K => 10,
		KeyCode::L => 11,
		KeyCode::M => 12,
		KeyCode::N => 13,
		KeyCode::O => 14,
		KeyCode::P => 15,
		KeyCode::Q => 16,
		KeyCode::R => 17,
		KeyCode::S => 18,
		KeyCode::T => 19,
		KeyCode::U => 20,
		KeyCode::V => 21,
		KeyCode::W => 22,
		KeyCode::X => 23,
		KeyCode::Y => 24,
		KeyCode::Z=> 25,
		_ => i32::MIN,
	}
}

pub enum MenuSelections
{
	NoInput,
	Cancel,
	ToggleSelected,
	Execute
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuType
{
	PickupItem,
	DropItem,
	UnequipItem
}

//type defs for function pointers so I can use dynamic dispatch to not have to repeat as much menu code and keep ProgramStates tidier
type MenuFunction = dyn FnMut (&mut State,  &mut Vec<(Entity, bool)>) -> MenuSelections;
type MenuDrawFunction = dyn FnMut ( &mut BTerm, Vec<(Entity, bool)>, &mut State) -> ();

pub fn select_menu_functions(menu : MenuType) -> (Box<MenuFunction>, Box<MenuDrawFunction>)
{
	match menu
	{
		MenuType::PickupItem => 
		{
			let  func: Box<MenuFunction> = Box::new(menu_input);
			let draw_func: Box<MenuDrawFunction> = Box::new(draw_pickup_menu);
			return (func, draw_func);
		}
		//TODO: REMOVE THIS
		_ => {(Box::new(menu_input), Box::new(draw_pickup_menu))}
	}
}

pub fn menu_input(state : &mut State, items : &mut Vec<(Entity, bool)>) -> MenuSelections
{

	match get_last_key_pressed()
	{
		Some(key) => 
		{
			match key
			{
				KeyCode::Enter => {return MenuSelections::Execute;}
				KeyCode::Escape => {return MenuSelections::Cancel;}
				_ => {}
			}

			match items.get_mut( key_to_option(key )as usize)
			{
				Some(target) =>
				{
					target.1 = !target.1;
					return MenuSelections::ToggleSelected;
				}
				None => { console::log("Invalid menu item!"); return MenuSelections::NoInput;}
			} 
		}

		None => {MenuSelections::NoInput}
	}
}