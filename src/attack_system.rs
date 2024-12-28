use bracket_lib::{color::{BLACK, RGB, WHITE}, terminal::console};
use hecs::{World, Entity};
use crate::{damage_system::DamageSystem, statistics::{BaseStatistics, Pools}, EquipmentSlot, Equippable, Equipped, Position, Weapon, WeaponStat};

use super::{State, Attack, Name, TakeDamage};
pub struct AttackSystem
{}

impl AttackSystem
{
    pub fn add_attack(attacker : hecs::Entity, target : hecs::Entity, state : &mut State)
    {
        //console::log("Inserting attack component");
        state.world.insert_one(attacker, Attack{target: target})
        .expect("Failed at trying to insert attack component!\n");
    }

    /// TODO: this will handle the logic of checking if an attack hits but for now just directly creates suffer TakeDamage
    pub fn run(state : &mut State)
    {
        let mut attackers: Vec<(Entity, Position)> = Vec::new();
        let mut defenders_to_damage : Vec<(Entity, Entity, BaseStatistics)> = Vec::new();
        
        for (_id,(attack,_name, stats, pos)) 
        in state.world.query::<(&mut Attack,&Name,&BaseStatistics, &Position)>().iter()
        {
            attackers.push((_id, *pos));
            defenders_to_damage.push((attack.target, _id, stats.clone()));
        }

        for (target,attacker, atkstats) in defenders_to_damage
        {
            
            let query = state.world.query_one_mut::<(&BaseStatistics, &Pools)>(target).expect("");
            let stats = query.0.clone();
            let pools = query.1.clone();
            std::mem::drop(query);

            let weapons = state.world.query::<(&Equipped, &Weapon)>()
                .iter()
                .filter(|(_ent,(equip, _wep))|
                 equip.owner == attacker && equip.slot == EquipmentSlot::MainHand)
                 .map(|ent| *ent.1.1)
                 .collect::<Vec<_>>();
            if weapons.len() < 1
            {
                //TODO: IMPLEMENT NATURAL WEPS FOR NPCS AND UNARMED FOR PLAYER
            }

            let mut to_hit_bonus = 0;
            let mut dmg_bonus = 0;
            let mut dmg_die = 0;

            to_hit_bonus += weapons[0].to_hit_bonus;
            dmg_bonus += weapons[0].dmg_bonus;
            dmg_die += weapons[0].damage_die;

            match weapons[0].uses_statistic
            {
                WeaponStat::Strength => 
                {
                    dmg_bonus += atkstats.strength.get_modifier();
                    to_hit_bonus += atkstats.strength.get_modifier();
                }
                WeaponStat::Dexterity => 
                {
                    dmg_bonus += atkstats.dexterity.get_modifier();
                    to_hit_bonus += atkstats.dexterity.get_modifier();
                }
            }

            let natural_roll = state.rng.roll_dice(1, 20);
            let mut hit = false;
            if natural_roll == 0
            {
                hit = false;
            } else if natural_roll == 20
            {
                hit = true;
            }else 
            {
                let atk_roll = natural_roll + to_hit_bonus;
                if atk_roll >= pools.armour_class
                {
                    hit = true;
                }
            }

            if hit
            {
                //TODO: allow for multiple damage dice to be rolled at a time by adding it to the weapon struct 
                let dmg = state.rng.roll_dice(1, dmg_die) + dmg_bonus;
                DamageSystem::mark_for_damage(state, target, dmg);
            }

            
            //DamageSystem::mark_for_damage(state, target, dmg);
        }


        for (entity, pos) in attackers.iter()
        {
            state.world.remove_one::<Attack>(*entity)
                .expect("Couldn't remove Attack component from the attacker");

            state.particle_builder.request(pos.x, pos.y,
                 RGB::named(WHITE), RGB::named(BLACK), '/', 50., None);
        }
    }



}