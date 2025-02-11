mod gamelog;
mod status_panel;
mod item_window;

pub use item_window::*;
use new_egui_macroquad::egui as egui;
use status_panel::right_panel;
use crate::State;

pub fn ui_layout(ctx : &egui::Context, state : &State)
{
    gamelog::bottom_panel(ctx,&state.game_log);
    right_panel(ctx, state);


}