use super::Attribute;

pub enum StatisticEffect
{
    Strength, Dexterity, Toughness, Intelligence, MentalFortitude, ArmourClass,
}

pub struct BaseStatistics
{
    strength : Attribute,
    dexterity : Attribute,
    toughness : Attribute,
    intelligence : Attribute,
    mental_fortitue : Attribute,
}

pub struct DerivedStatistics
{
    armour_class : Attribute,
    max_hitpoints : i32,
    //carry_capacity : Attribute,
    //max_stamina : i32,
    //max_mana : i32
}

impl DerivedStatistics
{
    pub fn new(stats : &BaseStatistics)-> DerivedStatistics
    {
        DerivedStatistics
        {
            armour_class: Attribute::new(10 + stats.dexterity.get_modifier()),
            max_hitpoints: 30 + stats.toughness.get_modifier()  
        }
    }
}

