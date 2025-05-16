use new_egui_macroquad::egui::{self, RichText};

use crate::{dev_console::Terminal, gamelog::DEBUGLOG};

pub struct DevConsole<'a> {
    current_cmd: String,
    terminal: &'a mut Terminal,
}

impl DevConsole<'_> {
    pub fn new<'a>(terminal: &'a mut Terminal) -> DevConsole<'a> {
        DevConsole {
            current_cmd: String::from(""),
            terminal,
        }
    }
    pub fn show(&mut self, ctx: &super::egui::Context, state: &super::State) {
        egui::Window::new("dev console").show(ctx, |ui| {
            ui.label(RichText::new("Dev Console:").heading());

            let logs = DEBUGLOG.get_log_guard();

            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = logs.entries.len();

            //view debug log
            egui::ScrollArea::vertical()
                .drag_to_scroll(true)
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show_rows(ui, row_height, num_rows, |ui, rows| {
                    for row in rows {
                        let text = logs.entries[row].clone();

                        ui.label(text);
                    }
                });

            //input new command
            let response = ui.add(egui::TextEdit::singleline(&mut self.current_cmd));

            if response.changed() {
                //todo
            }
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.terminal.set_cmd(self.current_cmd.clone());
                self.terminal.run_cmd();
            }
        });
    }
}
