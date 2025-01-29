use bracket_lib::random::RandomNumberGenerator;
use town_builder::starting_town;

use super::{random_map_builder, BuilderChain};

mod town_builder;
mod utils;


const MAX_WIDTH : i32 = 120;
const MAX_HEIGHT : i32 = 90;


pub fn level_generator(new_depth: i32) -> BuilderChain
{
    match new_depth
    {
        0 => { return starting_town()}
        _ => 
        {
            let mut rng = RandomNumberGenerator::new();
            let width = rng.range(50, MAX_WIDTH + 1);
            let height = rng.range(40, MAX_HEIGHT + 1);

            return random_map_builder(new_depth, width, height)
            
        }

        
    }
}