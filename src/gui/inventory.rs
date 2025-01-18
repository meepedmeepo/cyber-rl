use std::cmp::{max, min};

use bracket_lib::{color, prelude::BTerm};

use crate::{InContainer, Item, Name, State};

pub fn draw_inventory(state: &mut State, ctx: &mut BTerm)
{
    let mut items = Vec::new();

    for (_id,(_item, _in_container,name)) in 
        state.world.query::<(&Item, &InContainer,&Name)>()
            .iter().filter(|ent| ent.1.1.owner == state.player_ent
            .expect("Couldn't find player entity to query inventory"))
    {
        items.push(name.clone());
    }

    let height = max(5, items.len() +4);
    
    
    //let height = min(38,max(15,items.len()*3));
    ctx.draw_box_double(22, 10, 35, height, bracket_lib::color::WHITE, bracket_lib::color::BLACK);
    ctx.print_centered_at(35, 11, "Inventory");
    let mut y = 13;
    let mut index : u8 = 97;
    for item in items.iter()
    {
        ctx.print_color(23, y,color::WHITE,color::BLACK, format!("{}.) {}",index as char,item.name.clone()));
        y+=1;
        index += 1;
    }
}




