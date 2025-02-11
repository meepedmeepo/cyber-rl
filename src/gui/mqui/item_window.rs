use hecs::Entity;
use macroquad::window::{screen_height, screen_width};
use new_egui_macroquad::egui::{self as egui, Layout, RichText, ScrollArea};

use crate::{components::Name, State};

#[derive(Debug,PartialEq, Eq)]
pub enum ItemWindowType{Pickup, Drop, Unequip}

#[derive(Debug,PartialEq, Eq)]
pub enum ItemWindowMode {Single, Multiple}


pub struct ItemWindow
{
    pub contents : Vec<(Entity,bool)>,
    pub title : String,
    pub dimensions : (f32, f32),
    pub mode : ItemWindowMode,
}

impl ItemWindow
{
    pub fn default(contents : Vec<(Entity, bool)>) -> ItemWindow
    {
        ItemWindow {contents, title : "Inventory".to_string(), dimensions: (200.,200.), mode : ItemWindowMode ::Single}
    }

    pub fn new <T : Into<(f32,f32)>>(contents : Vec<(Entity, bool)>, title : String, dimensions : T, mode: ItemWindowMode) -> ItemWindow
    {
        ItemWindow {contents,title,dimensions : dimensions.into(),mode}
    }

    pub fn show(&mut self, ctx : &egui::Context, state : &State) -> Option<Vec<Entity>>
    {
        let mut target : Option<Vec<Entity>> = None;
        egui::Window::new(self.title.clone())
            .title_bar(false)
            .default_pos(self.calculate_window_pos())
            .movable(false)
            .fixed_size(self.dimensions)
            .show(ctx, |ui|
            {
                //Draws title
                ui.centered_and_justified(|ui| ui.label(RichText::new(self.title.clone())
                    .heading()));

                //Draws grid of items
                ui.with_layout(Layout::top_down_justified(egui::Align::Min)
                ,|ui|{
                    let text_style = egui::TextStyle::Body;
                    let num_rows = self.contents.len();
                    let row_height = ui.text_style_height(&text_style);
                    ScrollArea::vertical()
                        .show_rows(ui, row_height, num_rows, |ui, rows|
                        {
                            for (i,row) in rows.enumerate()
                            {
                                let c = 65u8 + i as u8;
                                if ui.button(format!("{}.) {}",c as char
                                    , state.world.get::<&Name>(self.contents[i].0).unwrap().name.clone())
                                ).clicked()
                                {
                                    if self.mode == ItemWindowMode::Single
                                    {
                                        target = Some(vec![self.contents[i].0.clone();1]);
                                        break;
                                    }else if self.mode == ItemWindowMode::Multiple
                                    {
                                        self.contents[i].1 = !self.contents[i].1;
                                    }
                                };

                            }
                        });

                        if self.mode == ItemWindowMode::Multiple
                        {
                            if ui.button("Execute!").clicked()
                            {
                                target = Some(self.contents.iter().filter_map(|(ent, selected)|
                            {
                                if *selected
                                {
                                    Some(*ent)
                                }
                                else {
                                    None
                                }
                            }).collect())
                            }
                        }
                });
            });

        target
    }

    fn calculate_window_pos(&self) -> (f32,f32)
    {
        ((screen_width()/2.) - (self.dimensions.0/2.),(screen_height()/2.) - (self.dimensions.1/2.))
    }
}

