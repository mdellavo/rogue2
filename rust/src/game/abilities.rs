use std::time::Instant;
use hecs::{World, Entity};
use log::info;

use crate::ecs::components::*;
use super::character::abilities::ClassAbility;

/// Check if an ability is on cooldown
pub fn is_ability_on_cooldown(cooldowns: &Cooldowns, ability: ClassAbility) -> bool {
    if let Some(last_used) = cooldowns.ability {
        let elapsed_ms = last_used.elapsed().as_millis() as u64;
        let cooldown_ms = ability.cooldown_ms();
        elapsed_ms < cooldown_ms
    } else {
        false
    }
}

/// Activate a class ability for an entity
pub fn activate_ability(
    world: &mut World,
    entity: Entity,
    ability: ClassAbility,
) -> Result<(), String> {
    // First check cooldown and get character info
    let (character_class, is_on_cooldown, health_max, int_stat) = {
        let mut query = world
            .query_one::<(&Character, &Cooldowns, &Health, &Stats)>(entity)
            .map_err(|_| "Entity missing required components")?;
        let (character, cooldowns, health, stats) = query.get().ok_or("Failed to get components")?;

        (
            character.class,
            is_ability_on_cooldown(cooldowns, ability),
            health.max,
            stats.int,
        )
    };

    // Check if ability is on cooldown
    if is_on_cooldown {
        return Err(format!("{} is on cooldown", ability.name()));
    }

    // Apply ability effect based on type
    match ability {
        ClassAbility::SecondWind => {
            // Fighter: Heal 25% max HP
            let heal_amount = (health_max as f32 * 0.25) as i32;

            // Apply healing
            if let Ok(mut health) = world.get::<&mut Health>(entity) {
                health.current = (health.current + heal_amount).min(health.max);
                info!(
                    "{:?} used Second Wind, healed for {} (now {}/{})",
                    character_class, heal_amount, health.current, health.max
                );
            }
        }
        ClassAbility::SneakAttack => {
            // Rogue: Double damage on next attack (handled in combat system)
            // We'll add a buff component for this in the future
            info!("{:?} activated Sneak Attack", character_class);
        }
        ClassAbility::HealingWord => {
            // Cleric: Heal ally 30% max HP
            // For now, heal self
            let heal_amount = (health_max as f32 * 0.30) as i32;

            // Apply healing
            if let Ok(mut health) = world.get::<&mut Health>(entity) {
                health.current = (health.current + heal_amount).min(health.max);
                info!(
                    "{:?} used Healing Word, healed for {} (now {}/{})",
                    character_class, heal_amount, health.current, health.max
                );
            }
        }
        ClassAbility::MagicMissile => {
            // Wizard: 3 projectiles dealing 1d4+INT damage each
            // This will be handled by a projectile system
            let base_damage = (1 + 4) / 2; // Average of 1d4
            let int_mod = (int_stat - 10) / 2;
            let damage_per_missile = base_damage + int_mod;
            let total_damage = damage_per_missile * 3;
            info!(
                "{:?} cast Magic Missile for {} total damage",
                character_class, total_damage
            );
        }
        ClassAbility::HuntersMark => {
            // Ranger: +1d6 damage for 30 seconds
            // This will be handled by a buff system
            info!("{:?} activated Hunter's Mark", character_class);
        }
        ClassAbility::Rage => {
            // Barbarian: +2 damage, 50% damage reduction for 10 seconds
            // This will be handled by a buff system
            info!("{:?} activated Rage", character_class);
        }
    }

    // Set cooldown
    if let Ok(mut cooldowns) = world.get::<&mut Cooldowns>(entity) {
        cooldowns.ability = Some(Instant::now());
    }

    Ok(())
}

/// Get remaining cooldown time in milliseconds
pub fn get_ability_cooldown_remaining(cooldowns: &Cooldowns, ability: ClassAbility) -> u64 {
    if let Some(last_used) = cooldowns.ability {
        let elapsed_ms = last_used.elapsed().as_millis() as u64;
        let cooldown_ms = ability.cooldown_ms();
        if elapsed_ms < cooldown_ms {
            cooldown_ms - elapsed_ms
        } else {
            0
        }
    } else {
        0
    }
}

/// Check if an attack is off cooldown
pub fn is_attack_ready(cooldowns: &Cooldowns, attack_speed: &AttackSpeed) -> bool {
    if let Some(last_attack) = cooldowns.attack {
        let elapsed_ms = last_attack.elapsed().as_millis() as u64;
        elapsed_ms >= attack_speed.cooldown_ms
    } else {
        true
    }
}

/// Perform a melee attack
pub fn perform_attack(
    world: &mut World,
    attacker_entity: Entity,
    target_entity: Entity,
) -> Result<(), String> {
    // First, collect all data we need from attacker
    let (attacker_class, attacker_str, _attack_speed_ms, is_ready) = {
        let mut query = world
            .query_one::<(&Character, &Stats, &AttackSpeed, &Cooldowns)>(attacker_entity)
            .map_err(|_| "Attacker missing required components")?;
        let (character, stats, attack_speed, cooldowns) = query.get().ok_or("Failed to get attacker components")?;

        let is_ready = is_attack_ready(cooldowns, attack_speed);

        (character.class, stats.str, attack_speed.cooldown_ms, is_ready)
    };

    // Check if attack is ready
    if !is_ready {
        return Err("Attack is on cooldown".to_string());
    }

    // Collect target data
    let (target_ac_value, target_species, _target_max_hp) = {
        let mut query = world
            .query_one::<(&ArmorClass, &Character, &Health)>(target_entity)
            .map_err(|_| "Target missing required components")?;
        let (ac, character, health) = query.get().ok_or("Failed to get target components")?;
        (ac.value, character.species, health.max)
    };

    // Calculate attack roll (simplified - should use random d20)
    let str_mod = (attacker_str - 10) / 2;
    let attack_roll = 10 + str_mod; // Simplified: assuming roll of 10

    info!(
        "{:?} attacks with roll {} vs AC {}",
        attacker_class, attack_roll, target_ac_value
    );

    // Check if attack hits
    if attack_roll >= target_ac_value {
        // Calculate damage (1d6 + STR modifier for basic attack)
        let base_damage = (1 + 6) / 2; // Average of 1d6
        let damage = base_damage + str_mod.max(0);

        // Apply racial trait damage resistance
        let damage = super::character::traits::apply_damage_resistance(damage, target_species);

        // Apply damage to target
        if let Ok(mut health) = world.get::<&mut Health>(target_entity) {
            health.current = (health.current - damage).max(0);
            info!(
                "{:?} hits for {} damage! Target: {}/{}",
                attacker_class, damage, health.current, health.max
            );
        }
    } else {
        info!("{:?} misses!", attacker_class);
    }

    // Set attack cooldown
    if let Ok(mut cooldowns) = world.get::<&mut Cooldowns>(attacker_entity) {
        cooldowns.attack = Some(Instant::now());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_cooldown() {
        let mut cooldowns = Cooldowns::default();
        let ability = ClassAbility::SecondWind;

        // Should not be on cooldown initially
        assert!(!is_ability_on_cooldown(&cooldowns, ability));
        assert_eq!(get_ability_cooldown_remaining(&cooldowns, ability), 0);

        // After using, should be on cooldown
        cooldowns.ability = Some(Instant::now());
        assert!(is_ability_on_cooldown(&cooldowns, ability));
        assert!(get_ability_cooldown_remaining(&cooldowns, ability) > 0);
    }

    #[test]
    fn test_attack_cooldown() {
        let mut cooldowns = Cooldowns::default();
        let attack_speed = AttackSpeed { cooldown_ms: 1000 };

        // Should be ready initially
        assert!(is_attack_ready(&cooldowns, &attack_speed));

        // After attacking, should be on cooldown
        cooldowns.attack = Some(Instant::now());
        // Immediately after, should still be on cooldown
        // (This might be flaky if the test runs very slowly)
        assert!(!is_attack_ready(&cooldowns, &attack_speed));
    }
}
