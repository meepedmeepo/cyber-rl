use std::collections::HashMap;

use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize)]
pub struct FactionInfo
{
    pub name : String,
    pub responses : HashMap<String, String>
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Reaction {
    Ignore, Attack, Flee
}