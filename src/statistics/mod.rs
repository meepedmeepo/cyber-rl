use bracket_lib::random::DiceType;
use hecs::Entity;

use crate::{gamelog::DEBUGLOG, State};

use super::Attribute;
mod leveling;
mod skills;
mod stat_calculation_system;
pub use leveling::*;
pub use skills::*;
pub use stat_calculation_system::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    Strength,
    Dexterity,
    Toughness,
    Intelligence,
    MentalFortitude,
}

impl StatType {
    pub fn from_string<S: Into<String>>(value: S) -> Self {
        let val = value.into();
        match val.as_str() {
            "strength" => StatType::Strength,
            "dexterity" => StatType::Dexterity,
            "toughness" => StatType::Toughness,
            "intelligence" => StatType::Intelligence,
            "mentalfortitude" => StatType::MentalFortitude,
            _ => panic!("{} is not a valid StatType", val),
        }
    }
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
    pub fn get_stat(&self, stat: StatType) -> Attribute {
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

    pub fn change_stat_bonus(&mut self, stat: StatType, bonus: i32) {
        match stat {
            StatType::Strength => {
                self.strength.bonuses += bonus;
            }
            StatType::Dexterity => {
                self.dexterity.bonuses += bonus;
            }
            StatType::Intelligence => {
                self.intelligence.bonuses += bonus;
            }
            StatType::Toughness => {
                self.toughness.bonuses += bonus;
            }
            StatType::MentalFortitude => {
                self.mental_fortitude.bonuses += bonus;
            }
        }
    }

    pub fn reset_stat_bonuses(&mut self) {
        self.dexterity.bonuses = 0;
        self.intelligence.bonuses = 0;
        self.mental_fortitude.bonuses = 0;
        self.strength.bonuses = 0;
        self.toughness.bonuses = 0;
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

pub fn stat_check(
    stat: StatType,
    entity: Entity,
    state: &mut State,
    difficulty_class: i32,
) -> bool {
    let entity_stats = state
        .world
        .get::<&BaseStatistics>(entity)
        .expect("Stat check ran against entity that doesn't have stats.");

    let roll = state.rng.roll_dice(1, 20);

    let stat_mod = entity_stats.get_stat(stat).get_modifier();

    let total = roll + stat_mod;
    let res = total >= difficulty_class;

    let check_status_msg;

    if res {
        check_status_msg = "passed";
    } else {
        check_status_msg = "failed";
    }

    let msg = format!(
        "Stat check {}: stat : {} ({}) + roll ({}) = total ({})",
        check_status_msg,
        stat.to_string(),
        stat_mod,
        roll,
        total
    );

    DEBUGLOG.add_log(msg.clone());
    state.game_log.add_log(msg);

    res
}
