
use macroquad::color::RED;
use new_egui_macroquad::egui::{self as egui, Color32, Frame, Layout, Widget};
use crate::{statistics::Pools, State};




pub fn right_panel(ctx : &egui::Context, state : &State )
{
    let pools = state.world.get::<&Pools>(state.player_ent.unwrap()).unwrap();


    egui::SidePanel::right("status_pane;")
        .exact_width(350.)
        .show(ctx, |ui|
        {
            ui.with_layout(Layout::top_down_justified(egui::Align::Center), |ui|
        {
            ui.
            ui.add(egui::ProgressBar::new((pools.hitpoints.current_value/pools.hitpoints.max_value) as f32)
                .text(format!("{} / {} HP", pools.hitpoints.current_value, pools.hitpoints.max_value))
                .fill(Color32::RED)
                );

                
        });
        });
}