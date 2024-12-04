use bracket_lib::terminal::*;
use bracket_lib::color;
use crate::ItemContainer;
use crate::Renderable;
use crate::{Player, Statistics,Name,Item};
use super::State;
use std::cmp::{max,min};

pub fn draw_ui(state :&mut State, ctx: &mut BTerm)
{
    ctx.draw_box(0, 42, 79, 7,
         bracket_lib::color::WHITE, bracket_lib::color::BLACK);
    for (_id,(_player,stats)) in
     state.world.query_mut::<(&Player,&Statistics)>()
    {
        let health = format!("HP: {} / {} ",stats.hp,stats.max_hp);
        ctx.print_color(16, 44, color::WHITE, color::BLACK, &health);
        ctx.draw_bar_horizontal(28, 44, 51, stats.hp, stats.max_hp,
             color::RED, color::BLACK);
    }
}


pub fn draw_inventory(state: &mut State, ctx: &mut BTerm)
{
    let mut items_to_search = Vec::new();
    {
        let itemlist = state.world.query_one_mut::<&ItemContainer>(Option::expect(state.player_ent, "Couldn't find player")).expect("Couldn't find item container on player!");
        for i in itemlist.items.iter()
        {
            items_to_search.push(*i);
        }
    }
     let mut items = Vec::new();
    // for (_id,(_item, name,graphic)) in state.world.query_mut::<(&Item,&Name,&Renderable)>().into_iter().filter(|x| x.0 == Option::expect(state.player_ent, "Couldn't find player!"))
    // {
    //     items.push((name,graphic));
    // }
    for item in items_to_search
    {
        let item_info = state.world.query_one_mut::<&Name>(item).expect("Couldn't find item name for displaying inventory!");
        items.push(item_info.clone());
    }


    let height = min(38,max(15,items.len()*3));
    ctx.draw_box(22, 10, 35, height, bracket_lib::color::WHITE, bracket_lib::color::BLACK);
    ctx.print_centered_at(35, 11, "Inventory");
    let mut y = 13;
    let mut index : u8 = 97;
    for item in items.iter()
    {
        ctx.print_color(23, y,color::WHITE,color::BLACK, format!("{}.) {}",index as char,item.name.clone()));
        y+=2;
        index += 1;
    }
}