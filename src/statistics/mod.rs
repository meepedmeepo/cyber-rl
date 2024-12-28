use super::Attribute;

pub enum StatisticEffect
{
    Strength, Dexterity, Toughness, Intelligence, MentalFortitude, ArmourClass,
}

pub struct BaseStatistics
{
    pub strength : Attribute,
    pub dexterity : Attribute,
    pub toughness : Attribute,
    pub intelligence : Attribute,
    pub mental_fortitude : Attribute,
}

pub struct Pools
{
    pub hitpoints: StatPool,
    pub exp: i32,
    pub level : i32,
    pub armour_class : i32
    
}

pub struct StatPool
{
    current_value: i32,
    max_value: i32,
}

impl StatPool
{
    pub fn new(max_value: i32) -> StatPool
    {
        StatPool
        {
            max_value: max_value,
            current_value: max_value
        }
    }

    pub fn restore(&mut self, value: i32)
    {
        self.current_value = std::cmp::min(self.max_value, self.current_value + value);
    }

    pub fn damage(&mut self, value: i32)
    {
        self.current_value = std::cmp::max(0, self.current_value - value);
    }
}

