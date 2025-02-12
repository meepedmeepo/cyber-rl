use std::i32;

use bracket_lib::prelude::{BTerm, VirtualKeyCode};
use bracket_lib::terminal::console;
use hecs::Entity;
use macroquad::input::{get_last_key_pressed, KeyCode};

use crate::gui::draw_pickup_menu;
use crate::{Equippable, InContainer, Item, Map, Name, ProgramState, RangedTargetting, State, WantsToEquipItem, WantsToPickupItem, WantsToUseItem};

//TODO: REMOVE THIS FILE COMPLETELY :D 



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