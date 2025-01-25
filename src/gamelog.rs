


pub struct GameLog
{
    entries : Vec<String>,
    pub index : usize,
}

impl GameLog
{
    pub fn new() ->GameLog
    {
        GameLog
        {
            entries: Vec::new(),
            index: 0,
        }
    }

    pub fn add_log(&mut self, msg: String)
    {
        self.entries.push(". ".to_string() + &msg);
        //self.index+=1;
    }

    pub fn view_log(&self,num_entries: usize) -> Vec<String>
    {
        self.entries.iter().rev().skip(self.index).take(num_entries ).map(|s|s.clone()).collect::<Vec<String>>()
    }



}