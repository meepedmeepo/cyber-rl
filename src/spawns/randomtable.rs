use bracket_lib::prelude::console;



pub struct RandomEntry
{
    name: String,
    weight: i32
}

impl RandomEntry
{
    pub fn new<S:ToString>(name: S, weight: i32) -> RandomEntry
    {
        RandomEntry{name: name.to_string(), weight}
    }
}

#[derive(Default)]
pub struct RandomTable
{
    entries: Vec<RandomEntry>,
    total_weight: i32
}

impl RandomTable
{
    pub fn new() -> RandomTable
    {
        RandomTable{entries: Vec::new(), total_weight: 0}
    }

    pub fn add<S:ToString>(mut self, name: S, weight: i32) -> RandomTable
    {
        self.total_weight += weight;
        self.entries.push(RandomEntry::new(name.to_string(), weight));
        self
    }

    pub fn roll(&self, rng: &mut bracket_lib::random::RandomNumberGenerator) -> String
    {
        if self.total_weight == 0 {return "zero weight".to_string();}
        
        let mut roll = rng.roll_dice(1, self.total_weight)-1;
        //console::log(roll);
        let mut index :usize = 0;

        while roll >= 0
        {
            if roll <= self.entries[index].weight
            {
                return self.entries[index].name.clone();
            }

            roll -= self.entries[index].weight;
            index += 1;
        }

        "none".to_string()
    }
}