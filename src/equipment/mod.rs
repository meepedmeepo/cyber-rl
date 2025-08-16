use hecs::Entity;

///This is spawned in ECS by an item as a container for effects that
/// happen when an entity hits a target with an attack.
pub struct HitEffect {
    pub owner: Entity,
    pub source: Entity,
}

pub struct DamageRecieveEffect {
    pub owner: Entity,
    pub source: Entity,
}

pub struct DeathEffect {
    pub owner: Entity,
    pub source: Entity,
}

///This entity is spawned in world as a container for effect components
/// given by equipment.
pub struct EquipEffect {
    pub owner: Entity,
    pub source: Entity,
}

pub struct Duration {
    pub time_to_live: i32,
}

pub struct Useable {}
