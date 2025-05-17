use crate::gui::mqui::{show_tooltip_window, DevConsole, ItemWindow, ItemWindowMode};
use crate::gui::TargettingMode;
use crate::{camera, ProgramState, State};
use hecs::Entity;
use new_egui_macroquad::egui::{self as egui};
use std::sync::{LazyLock, Mutex};

pub static MANAGER: LazyLock<Mutex<ScreenManager>> = LazyLock::new(|| {
    Mutex::new(ScreenManager {
        current_menu: None,
        tooltip_active: false,
    })
});

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MenuType {
    Pickup,
    Drop,
    Unequip,
    Inventory,
}

pub struct MenuScreen {
    pub window: ItemWindow,
    pub response: Option<Vec<Entity>>,
    pub exit_called: bool,
    pub menu_type: MenuType,
}

pub struct ScreenManager {
    pub current_menu: Option<MenuScreen>,
    pub tooltip_active: bool,
}

impl MenuScreen {
    fn show(&mut self, ctx: &egui::Context, state: &mut State) -> Option<Vec<Entity>> {
        (self.response, self.exit_called) = self.window.show(ctx, state);

        match &self.response {
            Some(res) => Some(res.clone()),

            None => None,
        }
    }

    pub fn new(
        contents: Vec<(Entity, bool)>,
        title: String,
        mode: ItemWindowMode,
        menu_type: MenuType,
    ) -> MenuScreen {
        MenuScreen {
            window: ItemWindow::default_with_type(contents, title, mode),
            response: None,
            exit_called: false,
            menu_type,
        }
    }
}

impl ScreenManager {
    pub fn show(&mut self, ctx: &egui::Context, state: &mut State, console: &mut DevConsole) {
        console.show(ctx, state);

        if self.tooltip_active {
            if let TargettingMode::Keyboard { cursor_pos } = state.target_mode {
                let (min_x, _max_x, min_y, _max_y) = camera::get_screen_bounds(state);

                show_tooltip_window(ctx, state, cursor_pos.x + min_x, cursor_pos.y + min_y);
            }
        }
        match self.current_menu {
            Some(ref mut menu) => {
                let res = menu.show(ctx, state);

                if res.is_some() {
                    state.current_state = ProgramState::AwaitingMenu {
                        response: Some(res.unwrap()),
                        menu_type: menu.menu_type,
                    };

                    self.current_menu = None;
                } else if menu.exit_called == true {
                    //std::mem::drop(menu);

                    self.current_menu = None;
                    state.current_state = ProgramState::AwaitingInput;
                } else {
                }
            }
            None => {}
        }
    }

    pub fn create_menu(
        &mut self,
        contents: Vec<(Entity, bool)>,
        title: String,
        mode: ItemWindowMode,
        menu_type: MenuType,
        state: &mut State,
    ) {
        self.current_menu = Some(MenuScreen::new(contents, title, mode, menu_type));

        state.current_state = ProgramState::AwaitingMenu {
            response: None,
            menu_type,
        };
    }
}
