
use macroquad::{color::RED, time::get_fps};
use new_egui_macroquad::egui::{self as egui, Color32, Frame, Layout, Widget};
use crate::{hunger::HungerLevel, statistics::{calculate_xp_from_level, get_xp_from_current_level, BaseStatistics, Pools}, State};




pub fn right_panel(ctx : &egui::Context, state : &State )
{
    let pools = state.world.get::<&Pools>(state.player_ent.unwrap()).unwrap();


    egui::SidePanel::right("status_pane;")
        .exact_width(350.)
        .resizable(false)
        .show_separator_line(false)
        //.frame(egui::Frame::rounding(egui::Frame::none(), 1.))
        .show(ctx, |ui|
        {
            ui.with_layout(Layout::top_down_justified(egui::Align::Center), |ui|
        {
            //ui.
            ui.columns(3, |cols| 
            {
                cols[0].label(format!("Depth {}",state.map.depth));
                cols[1].label(format!("Turn {}",state.turn_number));
                cols[2].label(format!("FPS: {}", get_fps()));
            });


            ui.add(egui::ProgressBar::new(((pools.hitpoints.current_value as f32/pools.hitpoints.max_value as f32)))
                .text(format!("{} / {} HP", pools.hitpoints.current_value, pools.hitpoints.max_value))
                .fill(Color32::RED)
                );

            let xp = pools.exp;
            let cl = calculate_xp_from_level(pools.level);

            let nl = calculate_xp_from_level(pools.level+1);

            ui.add(egui::ProgressBar::new((xp as f32-cl as f32)/(nl as f32-cl as f32))
            .text(format!("Level {}:{} / {} XP",pools.level, xp-cl, nl-cl))
            .fill(Color32::BLUE));


            std::mem::drop(pools);
            let hng = state.world.get::<&HungerLevel>(state.player_ent.unwrap()).unwrap();

            ui.add(egui::ProgressBar::new(hng.nutrition.current_value as f32/hng.nutrition.max_value as f32)
                .text(format!("Hunger"))
                .fill(Color32::from_hex("#ee913a").unwrap()));


            std::mem::drop(hng);

            let stats = state.world.get::<&BaseStatistics>(state.player_ent.unwrap()).unwrap();

            ui.columns(2, |cols| 
                {
                    cols[0].label(format!("Strength: {} {}",stats.strength.total, stats.strength.get_modifier()));
                    cols[1].label(format!("Dexterity: {} {}",stats.dexterity.total, stats.dexterity.get_modifier()));
                });

            ui.columns(2, |cols| 
                {
                    cols[0].label(format!("Toughness: {} {}",stats.toughness.total, stats.toughness.get_modifier()));
                    cols[1].label(format!("Intelligence: {} {}",stats.intelligence.total, stats.intelligence.get_modifier()));
                });

            ui.columns(1, |cols| 
                {
                    cols[0].label(format!("Mental Fortitude: {} {}",stats.mental_fortitude.total, stats.mental_fortitude.get_modifier()));
                });
        });
        });
}