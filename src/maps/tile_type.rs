
#[derive(PartialEq,Clone, Copy, Debug, Hash, Eq)]
pub enum TileType
{
    Floor, Wall, DownStairs, Road, Footpath, Concrete, MetalGrate, RustedMetalFloor
}

pub fn tile_walkable(tt : TileType) -> bool
{
    match tt
    {
        TileType::Footpath | TileType::Concrete | TileType::DownStairs | TileType::Floor | TileType::MetalGrate | TileType::Road
            | TileType::RustedMetalFloor
            => true,
        _ => false
    }
}

pub fn tile_opaque(tt : TileType) -> bool
{
    match tt
    {
        TileType::Wall => true,
        _ => false
    }
}

pub fn tile_cost(tt : TileType) -> f32
{
    match tt
    {  
        TileType::Footpath => 0.9,
        TileType::MetalGrate => 1.1,
        TileType::Road => 0.8,
        _ => 1.0
    }
}