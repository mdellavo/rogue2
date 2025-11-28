use hecs::{World, Entity};
use log::info;

use crate::ecs::components::*;
use super::character::progression;

/// Award experience points to a character entity
pub fn award_xp(
    world: &mut World,
    entity: Entity,
    base_xp: u32,
) -> Result<(), String> {
    // Get character data
    let (species, old_level, old_xp) = {
        let mut query = world
            .query_one::<&Character>(entity)
            .map_err(|_| "Entity missing required components")?;
        let character = query.get().ok_or("Failed to get components")?;

        (character.species, character.level, character.experience)
    };

    // Calculate XP with species bonus
    let new_xp = progression::add_xp(old_xp, base_xp, species);

    // Check if leveled up
    let new_level = progression::level_from_xp(new_xp);
    let did_level_up = new_level > old_level;

    // Update character XP and level
    if let Ok(mut character) = world.get::<&mut Character>(entity) {
        character.experience = new_xp;
        character.level = new_level;
    }

    if did_level_up {
        info!("Character leveled up from {} to {}!", old_level, new_level);

        // Apply level up bonuses
        apply_level_up_bonuses(world, entity, old_level, new_level)?;
    }

    Ok(())
}

/// Apply level up bonuses to a character
fn apply_level_up_bonuses(
    world: &mut World,
    entity: Entity,
    old_level: u32,
    new_level: u32,
) -> Result<(), String> {
    let levels_gained = new_level - old_level;

    // Apply HP bonus for each level
    for _ in 0..levels_gained {
        if let Ok(mut health) = world.get::<&mut Health>(entity) {
            let old_max = health.max;

            // Apply +5 HP per level
            health.max += 5;

            // Heal the difference (so leveling up heals you)
            let hp_gained = health.max - old_max;
            health.current += hp_gained;

            info!(
                "Level up! Max HP increased by {} (now {}/{})",
                hp_gained, health.current, health.max
            );
        }
    }

    // Future: Allow player to choose stat increases
    // For now, stats remain the same

    Ok(())
}

/// Calculate XP reward for defeating an entity based on its level and class
pub fn calculate_xp_reward(target_level: u32, _target_class: CharacterClass) -> u32 {
    // Base XP scales with level
    let base_xp = match target_level {
        1 => 50,
        2 => 100,
        3 => 200,
        4 => 400,
        5 => 800,
        _ => 800 + (target_level - 5) * 200,
    };

    // Future: Different classes could give different XP
    // For now, all enemies give the same based on level
    base_xp
}

/// Handle entity death - award XP to the killer
pub fn handle_entity_death(
    world: &mut World,
    killed_entity: Entity,
    killer_entity: Entity,
) -> Result<(), String> {
    // Get info about the killed entity
    let (target_level, target_class) = {
        let mut query = world
            .query_one::<&Character>(killed_entity)
            .map_err(|_| "Killed entity has no character")?;
        let character = query.get().ok_or("Failed to get character")?;
        (character.level, character.class)
    };

    // Calculate XP reward
    let xp_reward = calculate_xp_reward(target_level, target_class);

    // Award XP to killer
    info!("Awarding {} XP to killer", xp_reward);
    award_xp(world, killer_entity, xp_reward)?;

    Ok(())
}

/// Check if an entity should be removed due to death
pub fn should_despawn_on_death(world: &World, entity: Entity) -> bool {
    if let Ok(health) = world.get::<&Health>(entity) {
        health.current <= 0
    } else {
        false
    }
}

/// Get a character's current XP progress toward next level
pub fn get_xp_progress(current_xp: u32, current_level: u32) -> (u32, u32, f32) {
    let current_level_xp = progression::xp_for_level(current_level);
    let next_level_xp = progression::xp_for_level(current_level + 1);
    let xp_into_level = current_xp - current_level_xp;
    let xp_needed = next_level_xp - current_level_xp;
    let progress = xp_into_level as f32 / xp_needed as f32;

    (xp_into_level, xp_needed, progress)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xp_calculation() {
        // Level 1 enemy gives 50 XP
        assert_eq!(calculate_xp_reward(1, CharacterClass::Fighter), 50);

        // Level 3 enemy gives 200 XP
        assert_eq!(calculate_xp_reward(3, CharacterClass::Wizard), 200);

        // Level 10 enemy gives more
        assert!(calculate_xp_reward(10, CharacterClass::Fighter) > 800);
    }

    #[test]
    fn test_xp_progress() {
        // At 150 XP, level 1
        let (into, needed, progress) = get_xp_progress(150, 1);
        assert_eq!(into, 150);
        assert_eq!(needed, 300);
        assert!((progress - 0.5).abs() < 0.01); // 50% progress

        // At 600 XP, level 2 (300 XP into level 2, need 600 more)
        let (into, needed, _) = get_xp_progress(600, 2);
        assert_eq!(into, 300);
        assert_eq!(needed, 600);
    }
}
