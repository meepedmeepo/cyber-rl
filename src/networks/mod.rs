use std::collections::HashSet;
use hecs::Entity;

mod network_map;
mod network_builder;

pub use network_map::*;
pub use network_builder::*;

pub struct Root {}
pub struct RootNode
{
    pub difficulty : i32,
}

//This component indicates the owner of the node this component is attached to
pub struct ParentNode
{
    pub ent : Entity
}

pub struct ControlNode
{
    pub level : i32,
}

pub struct FileServer
{}

pub struct Records
{
    pub content : HashSet<String>,
}

pub struct NodeOwned
{
    pub owner : Entity,
}
//internal representation of possible hacking commands
pub enum Command
{
    Infect,
    Disable,
    Enable,
    ListNodes
}