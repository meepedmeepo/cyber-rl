use new_egui_macroquad::egui as egui;

use crate::{camera, components::Name, State};



pub fn show_tooltip_window(ctx : &egui::Context, state : &mut State, x : i32, y : i32)
{
    let (min_x, _max_x, min_y, _max_y) =camera::get_screen_bounds(state);
    let entities = state.map.tile_contents[state.map.xy_idx(x, y)].clone();
    let mut name = Vec::new();

    for ent in entities.iter()
    {
        if let Ok(n) = state.world.query_one_mut::<&Name>(*ent)
        {
            name.push(n.name.clone());
        }
        
    }
    if name.len() > 0
    {
        let (screen_x, screen_y) = state.renderer.canvas.get_tile_screen_pos(x-min_x, y-min_y);
        egui::Window::new("Tile Contents")
            .title_bar(false)
            .current_pos((screen_x as f32 + 70., screen_y as f32 + 16.))
            .max_width(100.)
            .resizable(false)
            .movable(false)
            .show(ctx, |ui|
            {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui|
                {
                    egui::ScrollArea::vertical().show(ui, |ui|
                        {
                            for i in name.iter()
                            {
                                ui.label(i);
                            }
                        });
                });
                
            });
    }
}