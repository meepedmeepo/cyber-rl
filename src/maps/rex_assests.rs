use std::fs::File;

use bracket_lib::prelude::XpFile;




pub struct RexAssests 
{
    pub menu : XpFile
}

impl  RexAssests 
{
    
    pub fn new() -> RexAssests
    {
        let mut  fs = File::open("rex.xp").expect("msg");
        RexAssests
        {
            menu: XpFile::read(&mut fs).expect("msg")
        }
    }    
}