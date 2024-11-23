#[derive(PartialEq,Copy,Clone)]
pub struct Rect
{
    x1 : i32,
    x2 : i32,
    y1 : i32,
    y2 : i32,
}

impl Rect
{
    ////Constructor for the rect struct 
    pub fn _new(x1:i32,x2:i32,y1:i32,y2:i32) -> Rect
    {
        Rect
        {
            x1,
            x2,
            y1,
            y2,
        }
    }

}