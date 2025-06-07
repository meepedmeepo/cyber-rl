use bracket_lib::random::DiceType;

use super::Attribute;
mod leveling;
mod skills;
pub use leveling::*;
pub use skills::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    Strength,
    Dexterity,
    Toughness,
    Intelligence,
    MentalFortitude,
}

impl std::fmt::Display for StatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StatType::Strength => write!(f, "Strength"),
            StatType::Dexterity => write!(f, "Dexterity"),
            StatType::Toughness => write!(f, "Toughness"),
            StatType::Intelligence => write!(f, "Intelligence"),
            StatType::MentalFortitude => write!(f, "Mental Fortitude"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BaseStatistics {
    pub strength: Attribute,
    pub dexterity: Attribute,
    pub toughness: Attribute,
    pub intelligence: Attribute,
    pub mental_fortitude: Attribute,
}
impl BaseStatistics {
    pub fn roll_stats(num_dice: i32) -> BaseStatistics {
        let mut rng = bracket_lib::random::RandomNumberGenerator::new();
        let dice_pool = DiceType::new(num_dice, 6, 1);

        BaseStatistics {
            strength: Attribute::new(rng.roll(dice_pool)),
            dexterity: Attribute::new(rng.roll(dice_pool)),
            toughness: Attribute::new(rng.roll(dice_pool)),
            intelligence: Attribute::new(rng.roll(dice_pool)),
            mental_fortitude: Attribute::new(rng.roll(dice_pool)),
        }
    }
    pub fn get_stat(self, stat: StatType) -> Attribute {
        match stat {
            StatType::Strength => {
                return self.strength;
            }
            StatType::Dexterity => {
                return self.dexterity;
            }
            StatType::Intelligence => {
                return self.intelligence;
            }
            StatType::Toughness => {
                return self.toughness;
            }
            StatType::MentalFortitude => {
                return self.mental_fortitude;
            }
        }
    }
}
#[derive(Clone, Copy)]
pub struct Pools {
    pub hitpoints: StatPool,
    pub exp: i32,
    pub level: i32,
    pub armour_class: Attribute,
    pub hit_die: DiceType,
}
#[derive(Clone, Copy)]
pub struct StatPool {
    pub current_value: i32,
    pub max_value: i32,
}

impl StatPool {
    pub fn new(max_value: i32) -> StatPool {
        StatPool {
            max_value: max_value,
            current_value: max_value,
        }
    }

    pub fn restore(&mut self, value: i32) {
        self.current_value = std::cmp::min(self.max_value, self.current_value + value);
    }

    pub fn damage(&mut self, value: i32) {
        self.current_value = std::cmp::max(0, self.current_value - value);
    }
}
