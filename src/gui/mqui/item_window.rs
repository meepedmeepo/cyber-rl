use hecs::Entity;
use macroquad::{input::{is_key_down, KeyCode}, window::{screen_height, screen_width}};
use new_egui_macroquad::egui::{self as egui, Button, Color32, Layout, RichText, ScrollArea};

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

    pub fn default_with_type(contents : Vec<(Entity, bool)>, title : String, mode: ItemWindowMode) -> ItemWindow
    {
        ItemWindow{contents, title, dimensions : (200.,200.), mode}
    }

    pub fn new <T : Into<(f32,f32)>>(contents : Vec<(Entity, bool)>, title : String, dimensions : T, mode: ItemWindowMode) -> ItemWindow
    {
        ItemWindow {contents,title,dimensions : dimensions.into(),mode}
    }

    pub fn show(&mut self, ctx : &egui::Context, state : &State) -> (Option<Vec<Entity>>, bool)
    {
        let mut target : Option<Vec<Entity>> = None;
        let should_close = is_key_down(KeyCode::Escape);
        egui::Window::new(self.title.clone())
            .title_bar(false)
            .default_pos(self.calculate_window_pos())
            .movable(false)
            .fixed_size(self.dimensions)
            .show(ctx, |ui|
            {
                //Draws title
                //ui.centered_and_justified(|ui| ));
                
                ui.label(RichText::new(self.title.clone())
                    .heading());

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
                                let mut col = Color32::DARK_GRAY;

                                if self.contents[i].1 
                                {
                                    col = Color32::GREEN;
                                }

                                let c = 65u8 + i as u8;

                                if ui.add(Button::new(format!("{}.) {}",c as char
                                    , state.world.get::<&Name>(self.contents[i].0).unwrap().name.clone())
                                ).fill(col)).clicked()
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

        (target,should_close)
    }

    fn calculate_window_pos(&self) -> (f32,f32)
    {
        ((screen_width()/2.) - (self.dimensions.0/2.),(screen_height()/2.) - (self.dimensions.1/2.))
    }
}

