use bracket_lib::prelude::{BEvent, BTerm, VirtualKeyCode, INPUT};

use crate::State;

pub fn display_input_text(state : &mut State, ctx: &mut BTerm, text: &Vec<char>, x: i32, y : i32)
{
    let text = text.iter().collect::<String>();

    ctx.print(x, y, format!("/{}", text));
}
pub fn get_input_text(state : &mut State, ctx: &mut BTerm, text : &mut Vec<char>) -> bool
{
    let mut input = INPUT.lock();
    let mut enter_pressed = false;
    let mut quit = false;
    //BEvent::KeyboardInput { key: (), scan_code: (), pressed: () }

    input.for_each_message(|event| {
        if let BEvent::Character{c} = event
        {
            if !c.is_whitespace() && !c.is_control()
            {
            text.push(c);
            }
        }
        if let BEvent::KeyboardInput { key: VirtualKeyCode::Return, pressed: false, ..  } = event
        {
            enter_pressed = true;
        }
        if let BEvent::KeyboardInput { key: VirtualKeyCode::Back, pressed: true, .. } = event
        {
            let _ = text.pop();
        }
        if let BEvent::CloseRequested = event
        {
            quit = true
        }
    });
    if quit
    {
        ctx.quit();
    }
    if enter_pressed
    {
        return true;
    }
    else
    {
        false
    }
    
}