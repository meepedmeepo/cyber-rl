mod dev_console;
mod gamelog;
mod item_window;
mod status_panel;
mod tooltip_window;

pub use dev_console::*;
pub use item_window::*;
pub use tooltip_window::*;

use crate::State;
use new_egui_macroquad::egui;
use status_panel::right_panel;

pub fn ui_layout(ctx: &egui::Context, state: &State) {
    gamelog::bottom_panel(ctx, &state.game_log);
    right_panel(ctx, state);
}
