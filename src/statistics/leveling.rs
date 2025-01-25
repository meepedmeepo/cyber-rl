use bracket_lib::color::{ORANGE, RGB, WHITE};

use crate::State;

use super::{BaseStatistics, Pools};

const LEVEL_SCALING_FACTOR: f32 = 0.17;


pub fn level_up(stats: &BaseStatistics, pools : &mut Pools
    , rng : &mut bracket_lib::random::RandomNumberGenerator)
{
    let mut hp = rng.roll(pools.hit_die);
    hp += stats.toughness.get_modifier();

    pools.hitpoints.max_value += hp;
    pools.hitpoints.current_value = pools.hitpoints.max_value;
    
    pools.level += 1;
}


pub fn monster_xp_drop( level : i32) -> i32
{
    level*30
}

pub fn calculate_xp_from_level(level : i32) -> i32
{
    let base_xp =  level as f32/LEVEL_SCALING_FACTOR;

    base_xp.powf(2.) as i32
}

pub fn calculate_level_from_xp(xp: i32) -> i32
{
    let sqrt_xp = f32::sqrt(xp as f32); 
    let level = LEVEL_SCALING_FACTOR*sqrt_xp;

    level as i32
}

pub fn xp_to_next_level(level : i32, xp: i32) -> i32
{
    calculate_xp_from_level(level+1) - xp
}

pub fn check_level_up(state : &mut State)
{
    let (stats, pools) =state.world.query_one_mut::<(&BaseStatistics, &mut Pools)>
        (state.player_ent.expect("Couldn't find player to check level!")).unwrap();

    if xp_to_next_level(pools.level, pools.exp) <= 0
    {
        level_up(stats, pools, &mut state.rng);

        state.particle_builder
            .request(state.player_pos.x, state.player_pos.y, RGB::named(WHITE), RGB::named(ORANGE)
            , '+', 350., Some(state.player_ent.unwrap()));
    }
}

///used for UI level progress bar! the first i32 is level progress, the 2nd i32 is xp value of the next level
pub fn get_xp_from_current_level(state: &mut State)-> (i32,i32)
{
    let pools = state.world.query_one_mut::<&Pools>(state.player_ent.unwrap()).unwrap();

    let current_level = calculate_xp_from_level(pools.level);
    let xp_progress = pools.exp - current_level;

    let xp_next_level = calculate_xp_from_level(pools.level+1) - current_level;

    (xp_progress, xp_next_level)

}