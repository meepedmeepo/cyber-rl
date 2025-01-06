use bracket_lib::prelude::console;
use hecs::Entity;

use crate::{gamelog, Consumable, DamageEffect, GivesFood, HealingEffect, State};

use super::{add_effect, EffectType, ParticleBurst, Targets};




pub fn item_trigger(creator : Option<Entity>, item : Entity, targets : &Targets, state : &mut State)
{
    //fires off effect
    event_trigger(creator, item, targets, state);

    //despawns entity if it was consumable
    if state.world.get::<&Consumable>(item).is_ok()
    {
        if state.world.despawn(item).is_err()
        {
            console::log("Couldn't despawn consumable item after use!");
        }
    }
}

#[allow(dead_code)]
pub fn entry_trigger_fire(creator : Option<Entity>, prop: Entity, targets : &Targets, state : &mut State)
{
    state.game_log.add_log("Trap fired!".to_string());
    event_trigger(creator, prop, targets, state);
}

fn event_trigger(creator : Option<Entity>, item : Entity, targets : &Targets, state : &mut State)
{
    //do .get on item for different Components and then execute relevant code you nerdd!!!!!!


    if let Ok(damage) = state.world.get::<&DamageEffect>(item)
    {
        add_effect(creator, EffectType::Damage { amount: damage.damage_amount }, targets.clone());
    }

    if let Ok(heal) = state.world.get::<&HealingEffect>(item)
    {
        add_effect(creator, EffectType::Healing { amount: heal.healing_amount }, targets.clone());
    }

    if let Ok(food) = state.world.get::<&GivesFood>(item)
    {
        add_effect(creator, EffectType::Feed { amount: food.amount}, targets.clone());
    }

    if let Ok(p) = state.world.get::<&ParticleBurst>(item)
    {
        add_effect(creator, EffectType::Particle { glyph: p.particle.glyph, fg: p.particle.fg
            , bg: p.particle.bg, lifetime: p.particle.lifetime }, targets.clone());
    }

}