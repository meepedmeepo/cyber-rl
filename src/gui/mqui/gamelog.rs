use macroquad::window::screen_height;
use new_egui_macroquad::egui as egui;

use crate::gamelog::GameLog;

pub fn bottom_panel(ctx : &egui::Context, gamelog : &GameLog)
{
    egui::TopBottomPanel::bottom("gamelog")
        .exact_height(screen_height()/3.)
        .show(ctx, |ui| {
            //ui.label("Cunt");
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);

            let num_rows = gamelog.len();
            egui::ScrollArea::vertical()
                .drag_to_scroll(true)
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show_rows(ui, row_height, num_rows, |ui, rows| 
                {
                    for row in rows
                    {
                        let text = gamelog.entries[row].clone();

                        ui.label(text);
                    }
                })
                    //ui.colored_label(color, text)
                    //ui.text_edit_singleline("/");
                });
}