use bracket_lib::terminal::*;
use bracket_lib::color;
use crate::{Player, Statistics};

use super::{State};

pub fn draw_ui(state :&mut State, ctx: &mut BTerm)
{
    ctx.draw_box(0, 42, 79, 7,
         bracket_lib::color::WHITE, bracket_lib::color::BLACK);
    for (_id,(player,stats)) in
     state.world.query_mut::<(&Player,&Statistics)>()
    {
        let health = format!("HP: {} / {} ",stats.hp,stats.max_hp);
        ctx.print_color(16, 44, color::YELLOW, color::BLACK, &health);
        ctx.draw_bar_horizontal(28, 44, 51, stats.hp, stats.max_hp,
             color::RED, color::BLACK);
    }
}