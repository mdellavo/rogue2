use hecs::{World, Entity};
use crate::ecs::components::*;
use rand::Rng;

const FLEE_HP_THRESHOLD: f32 = 0.25; // Flee when below 25% HP

/// Update all monster AI behaviors (simplified version for MVP)
pub fn update_monster_ai(world: &mut World, _delta_time: f32) {
    // Collect entity data first to avoid borrow checker issues
    let mut entities_data: Vec<(Entity, AIState, f32, f32, f32, Option<Entity>)> = Vec::new();

    for (entity, (ai, health, pos, _spawn)) in
        world.query_mut::<(&AIController, &Health, &Position, &SpawnPoint)>()
    {
        entities_data.push((
            entity,
            ai.state,
            health.current as f32 / health.max as f32, // HP percentage
            pos.x,
            pos.y,
            ai.target,
        ));
    }

    // Now process AI logic without holding any borrows
    for (entity, _state, _hp_percent, _x, _y, _target) in entities_data {
        // Simple AI: just exist for now
        // More complex AI will be added when integrated with game loop
        if let Ok(ai) = world.get::<&mut AIController>(entity) {
            // Keep monsters in idle state for now
            if ai.state == AIState::Dead {
                continue;
            }
        }
    }
}

/// Move monster towards a target
pub fn move_towards_target(
    world: &mut World,
    entity: Entity,
    target_x: f32,
    target_y: f32,
    _delta_time: f32,
) {
    // Query for components separately to work around borrow checker
    let (pos_x, pos_y, speed_value) = {
        if let Ok(mut query) = world.query_one::<(&Position, &MovementSpeed)>(entity) {
            if let Some((pos, speed)) = query.get() {
                (pos.x, pos.y, speed.pixels_per_second)
            } else {
                return;
            }
        } else {
            return;
        }
    };

    // Now update velocity with the calculated values
    if let Ok(mut vel) = world.get::<&mut Velocity>(entity) {
        let dx = target_x - pos_x;
        let dy = target_y - pos_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 1.0 {
            // Normalize and apply speed
            vel.dx = (dx / distance) * speed_value;
            vel.dy = (dy / distance) * speed_value;
        } else {
            vel.dx = 0.0;
            vel.dy = 0.0;
        }
    }
}

/// Patrol around spawn point
pub fn patrol_behavior(world: &mut World, entity: Entity, delta_time: f32) {
    let (spawn_x, spawn_y) = {
        if let Ok(mut query) = world.query_one::<&SpawnPoint>(entity) {
            if let Some(spawn) = query.get() {
                (spawn.x, spawn.y)
            } else {
                return;
            }
        } else {
            return;
        }
    };

    // Calculate random patrol point near spawn
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let distance = rng.gen_range(5.0..10.0) * 32.0; // 5-10 tiles

    let patrol_x = spawn_x + angle.cos() * distance;
    let patrol_y = spawn_y + angle.sin() * distance;

    // Move towards patrol point
    move_towards_target(world, entity, patrol_x, patrol_y, delta_time);
}

/// Chase a target entity
pub fn chase_behavior(world: &mut World, entity: Entity, target: Entity, delta_time: f32) {
    if let Some((target_x, target_y)) = get_position(world, target) {
        move_towards_target(world, entity, target_x, target_y, delta_time);
    }
}

/// Flee from target back to spawn
pub fn flee_behavior(world: &mut World, entity: Entity, delta_time: f32) {
    let (spawn_x, spawn_y) = {
        if let Ok(mut query) = world.query_one::<&SpawnPoint>(entity) {
            if let Some(spawn) = query.get() {
                (spawn.x, spawn.y)
            } else {
                return;
            }
        } else {
            return;
        }
    };

    move_towards_target(world, entity, spawn_x, spawn_y, delta_time);
}

/// Find the nearest enemy (player) within detection range
pub fn find_nearest_enemy(world: &World, entity: Entity) -> Option<Entity> {
    let (my_x, my_y, detection_distance) = {
        let mut query = world.query_one::<(&Position, &DetectionRange)>(entity).ok()?;
        let (pos, detection) = query.get()?;
        (pos.x, pos.y, detection.tiles * 32.0)
    };

    let mut nearest: Option<(Entity, f32)> = None;

    // Find nearest player
    for (other_entity, (other_pos, _player)) in world.query::<(&Position, &Player)>().iter() {
        if other_entity == entity {
            continue;
        }

        let dx = other_pos.x - my_x;
        let dy = other_pos.y - my_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= detection_distance {
            if let Some((_, nearest_dist)) = nearest {
                if distance < nearest_dist {
                    nearest = Some((other_entity, distance));
                }
            } else {
                nearest = Some((other_entity, distance));
            }
        }
    }

    nearest.map(|(e, _)| e)
}

/// Check if target is in attack range
pub fn is_in_attack_range(world: &World, entity: Entity, target: Entity) -> bool {
    const MELEE_RANGE: f32 = 1.5 * 32.0; // 1.5 tiles

    if let Some((my_pos, target_pos)) = get_positions(world, entity, target) {
        let dx = target_pos.0 - my_pos.0;
        let dy = target_pos.1 - my_pos.1;
        let distance = (dx * dx + dy * dy).sqrt();
        distance <= MELEE_RANGE
    } else {
        false
    }
}

/// Check if can see target (within detection range)
pub fn can_see_target(world: &World, entity: Entity, target: Entity) -> bool {
    let detection_distance = if let Ok(mut query) = world.query_one::<&DetectionRange>(entity) {
        if let Some(detection) = query.get() {
            detection.tiles * 32.0
        } else {
            return false;
        }
    } else {
        return false;
    };

    if let Some((my_pos, target_pos)) = get_positions(world, entity, target) {
        let dx = target_pos.0 - my_pos.0;
        let dy = target_pos.1 - my_pos.1;
        let distance = (dx * dx + dy * dy).sqrt();
        return distance <= detection_distance;
    }

    false
}

/// Check if monster should flee
pub fn should_flee(health: &Health) -> bool {
    let hp_percent = health.current as f32 / health.max as f32;
    hp_percent < FLEE_HP_THRESHOLD
}

/// Get position of an entity
fn get_position(world: &World, entity: Entity) -> Option<(f32, f32)> {
    let mut query = world.query_one::<&Position>(entity).ok()?;
    let pos = query.get()?;
    Some((pos.x, pos.y))
}

/// Get positions of two entities
fn get_positions(world: &World, entity1: Entity, entity2: Entity) -> Option<((f32, f32), (f32, f32))> {
    let pos1 = get_position(world, entity1)?;
    let pos2 = get_position(world, entity2)?;
    Some((pos1, pos2))
}

/// Call nearby allies when attacked (pack behavior)
pub fn call_for_help(world: &mut World, entity: Entity, attacker: Entity) {
    // Get monster position and type
    let (my_x, my_y, monster_type) = {
        if let Ok(mut query) = world.query_one::<(&Position, &Monster)>(entity) {
            if let Some((pos, monster)) = query.get() {
                (pos.x, pos.y, monster.monster_type)
            } else {
                return;
            }
        } else {
            return;
        }
    };

    // Only pack creatures call for help
    if !is_pack_creature(monster_type) {
        return;
    }

    const CALL_RANGE: f32 = 15.0 * 32.0; // 15 tiles

    // Collect allies to alert
    let mut allies_to_alert = Vec::new();

    for (ally_entity, (ally_pos, ally_monster)) in world.query::<(&Position, &Monster)>().iter() {
        if ally_entity == entity {
            continue;
        }

        // Check if same monster type
        if ally_monster.monster_type != monster_type {
            continue;
        }

        // Check if in range
        let dx = ally_pos.x - my_x;
        let dy = ally_pos.y - my_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= CALL_RANGE {
            allies_to_alert.push(ally_entity);
        }
    }

    // Alert allies
    for ally_entity in allies_to_alert {
        if let Ok(mut ai) = world.get::<&mut AIController>(ally_entity) {
            ai.state = AIState::Chase;
            ai.target = Some(attacker);
            ai.chase_timer = 0.0;
        }
    }
}

/// Check if monster is a pack creature
fn is_pack_creature(monster_type: MonsterType) -> bool {
    matches!(
        monster_type,
        MonsterType::GiantRat
            | MonsterType::Goblin
            | MonsterType::Wolf
            | MonsterType::OrcWarrior
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_flee() {
        let high_hp = Health {
            current: 80,
            max: 100,
        };
        let low_hp = Health {
            current: 20,
            max: 100,
        };

        assert!(!should_flee(&high_hp));
        assert!(should_flee(&low_hp));
    }

    #[test]
    fn test_pack_creature() {
        assert!(is_pack_creature(MonsterType::GiantRat));
        assert!(is_pack_creature(MonsterType::Wolf));
        assert!(!is_pack_creature(MonsterType::Ogre));
    }
}
