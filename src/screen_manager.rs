use std::sync::{LazyLock, Mutex};
use new_egui_macroquad::egui::{self as egui};
use hecs::Entity;
use crate::{ProgramState, State};
use crate::gui::mqui::{ItemWindow, ItemWindowMode};



pub static MANAGER : LazyLock<Mutex<ScreenManager>> = LazyLock::new(|| Mutex::new
    (ScreenManager { current_menu: None }));

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MenuType {Pickup, Drop, Unequip, Inventory}


pub struct MenuScreen 
{ 
    pub window : ItemWindow,
    pub response : Option<Vec<Entity>>, 
    pub exit_called : bool, 
    pub menu_type : MenuType,
}

pub struct ScreenManager
{
    pub current_menu : Option<MenuScreen>
}


impl MenuScreen
{
    fn show(&mut self,ctx : &egui::Context, state : &mut State) -> Option<Vec<Entity>>
    {
        (self.response, self.exit_called) = self.window.show(ctx, state);

        match &self.response
        {
            Some(res) =>
            {
                Some(res.clone())
            }

            None => 
            {
                None
            }
        }
    }

    pub fn new(contents: Vec<(Entity, bool)>, title : String, mode : ItemWindowMode, menu_type : MenuType) ->MenuScreen
    {
        MenuScreen{window: ItemWindow::default_with_type(contents, title, mode),
            response : None, exit_called : false, menu_type}
    }

    
}


impl ScreenManager
{
    pub fn show(&mut self, ctx : &egui::Context, state : &mut State)
    {
        match self.current_menu
        {
            Some(ref mut menu) =>
            {
                let res = menu.show(ctx, state);

                if res.is_some()
                {
                    state.current_state = ProgramState::AwaitingMenu { response: Some(res.unwrap()), menu_type: menu.menu_type };

                    self.current_menu = None;
                }else if menu.exit_called == true
                {
                    //std::mem::drop(menu);

                    self.current_menu = None;
                    state.current_state = ProgramState::AwaitingInput;
                }else 
                {
                    
                }

            }
            None =>
            {

            }
        }
    }


    pub fn create_menu(&mut self, contents : Vec<(Entity,bool)>, title : String, mode : ItemWindowMode, menu_type : MenuType, state : &mut State)
    {
        self.current_menu = Some(MenuScreen::new(contents, title, mode, menu_type));
        
        state.current_state = ProgramState::AwaitingMenu { response: None, menu_type};
    }
}