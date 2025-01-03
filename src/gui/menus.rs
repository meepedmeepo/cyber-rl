use bracket_lib::{color::{BLACK, GREEN, RGB, WHITE, YELLOW}, prelude::{BTerm, Point}};
use hecs::Entity;

use crate::{menus::MenuType, Name, State};



//TODO: try to remove reference to state from this function to keep a seperation of GUI vs business logic
pub fn draw_pickup_menu(ctx : &mut BTerm, items: Vec<(Entity, bool)>, state : &mut State)
{
    let mut menu_content = Vec::new();
    
    let mut index: u8 = 97;
    for item in items.iter()
    {
        let name =state.world.query_one_mut::<&Name>(item.0).unwrap();
        menu_content.push((format!("{}.) {}", index as char, name.name.clone()), item.1));
        
        index += 1;
    }

    draw_menu_list(ctx, &menu_content, "Pickup Item:", Point::new(22, 10),
         35, RGB::named(WHITE), RGB::named(BLACK), RGB::named(GREEN));
}

pub fn menu_theme(menu : MenuType) -> (&'static str, RGB, RGB) 
{
    match menu
    {
        MenuType::PickupItem =>
        {
            return ("Pickup Items:", RGB::named(WHITE), RGB::named(GREEN))
        }

        MenuType::DropItem => 
        {
            return ("Drop Items:", RGB::named(WHITE), RGB::named(GREEN))
        }

        MenuType::UnequipItem =>
        {
            return ("Unequip Items:", RGB::named(WHITE), RGB::named(GREEN))
        }
    }
}

pub fn draw_menu_custom(ctx : &mut BTerm, items: &Vec<(Entity, bool)>, title: &str, text_colour: RGB,
     highlight: RGB, state: &mut State)
{
    let mut menu_content = Vec::new();
    
    let mut index: u8 = 97;
    for item in items.iter()
    {
        let name = state.world.query_one_mut::<&Name>(item.0).unwrap();
        menu_content.push((format!("{}.) {}", index as char, name.name.clone()), item.1));
        
        index += 1;
    }

    draw_menu_list(ctx, &menu_content, title , Point::new(22, 10),
         35, text_colour, RGB::named(BLACK), highlight);
}


pub fn draw_menu_list(ctx : &mut BTerm, content: &Vec<(String, bool)>, title : &str, pos : Point, width : i32, fg : RGB
        , bg : RGB, highlight: RGB)
{
    let height = std::cmp::max(content.len() + 4, 5);
    ctx.draw_box_double(pos.x, pos.y, width, height, fg, bg);

    let centre_x = pos.x + (width/2);
    
    ctx.print_color_centered_at(centre_x, pos.y + 1, YELLOW, bg, title);

    let mut current_y =  pos.y + 3;

    for (ln, is_selected) in content.iter()
    {
        if *is_selected
        {
            ctx.print_color( pos.x + 1, current_y, fg, highlight, ln.clone());
        } else
        {
            ctx.print_color( pos.x + 1, current_y, fg, bg, ln.clone());
        }
        current_y += 1;
    }
}