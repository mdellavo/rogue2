use hecs::{World, Entity};
use std::time::Instant;
use crate::ecs::components::*;
use crate::game::monsters::MonsterTemplate;

const REGULAR_RESPAWN_SECONDS: u64 = 300; // 5 minutes
const BOSS_RESPAWN_HOURS: u32 = 24; // 24 hours

/// Spawn point manager tracks all spawn points and their respawn timers
pub struct SpawnPointManager {
    pub spawn_points: Vec<SpawnPointData>,
}

#[derive(Debug, Clone)]
pub struct SpawnPointData {
    pub id: u64,
    pub monster_type: MonsterType,
    pub x: f32,
    pub y: f32,
    pub is_boss: bool,
    pub respawn_cooldown_seconds: u64,
    pub last_spawn_time: Option<Instant>,
    pub spawned_entity: Option<Entity>,
}

impl SpawnPointManager {
    pub fn new() -> Self {
        Self {
            spawn_points: Vec::new(),
        }
    }

    /// Add a spawn point to the manager
    pub fn add_spawn_point(
        &mut self,
        monster_type: MonsterType,
        x: f32,
        y: f32,
        is_boss: bool,
    ) -> u64 {
        let id = self.spawn_points.len() as u64;
        let respawn_cooldown = if is_boss {
            BOSS_RESPAWN_HOURS as u64 * 3600 // Convert hours to seconds
        } else {
            REGULAR_RESPAWN_SECONDS
        };

        self.spawn_points.push(SpawnPointData {
            id,
            monster_type,
            x,
            y,
            is_boss,
            respawn_cooldown_seconds: respawn_cooldown,
            last_spawn_time: None,
            spawned_entity: None,
        });

        id
    }

    /// Update all spawn points and spawn monsters if ready
    pub fn update(&mut self, world: &mut World) {
        let now = Instant::now();

        for spawn_point in &mut self.spawn_points {
            // Check if monster is still alive
            if let Some(entity) = spawn_point.spawned_entity {
                // Check if entity still exists and is not dead
                if let Ok(mut query) = world.query_one::<&AIController>(entity) {
                    if let Some(ai) = query.get() {
                        if ai.state != AIState::Dead {
                            continue; // Monster still alive
                        }
                    }
                } else {
                    // Entity no longer exists
                    spawn_point.spawned_entity = None;
                }
            }

            // Check if ready to respawn
            let should_spawn = if let Some(last_spawn) = spawn_point.last_spawn_time {
                now.duration_since(last_spawn).as_secs() >= spawn_point.respawn_cooldown_seconds
            } else {
                true // Never spawned before
            };

            if should_spawn {
                let entity = spawn_monster(
                    world,
                    spawn_point.monster_type,
                    spawn_point.x,
                    spawn_point.y,
                    spawn_point.is_boss,
                );
                spawn_point.spawned_entity = Some(entity);
                spawn_point.last_spawn_time = Some(now);
            }
        }
    }

    /// Clear all spawn points (useful for map transitions)
    pub fn clear(&mut self) {
        self.spawn_points.clear();
    }
}

impl Default for SpawnPointManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawn a monster entity in the world at the given position
pub fn spawn_monster(
    world: &mut World,
    monster_type: MonsterType,
    x: f32,
    y: f32,
    is_boss: bool,
) -> Entity {
    let template = MonsterTemplate::get(monster_type);

    let mut builder = hecs::EntityBuilder::new();

    builder.add(Position { x, y });
    builder.add(Velocity { dx: 0.0, dy: 0.0 });
    builder.add(Health {
        current: template.hp,
        max: template.hp,
    });
    builder.add(Stats {
        str: template.stats.str,
        dex: template.stats.dex,
        con: template.stats.con,
        int: template.stats.int,
        wis: template.stats.wis,
        cha: template.stats.cha,
    });
    builder.add(Monster {
        monster_type: template.monster_type,
        level: template.level,
        xp_reward: template.xp_reward,
    });
    builder.add(ArmorClass {
        value: template.ac,
    });
    builder.add(MovementSpeed {
        pixels_per_second: template.speed,
    });
    builder.add(AttackSpeed {
        cooldown_ms: template.attack_cooldown_ms,
    });
    builder.add(DetectionRange {
        tiles: template.detection_range_tiles,
    });
    builder.add(SpawnPoint {
        x,
        y,
        max_distance: 10.0 * 32.0, // 10 tiles from spawn
    });
    builder.add(AIController::default());
    builder.add(Sprite {
        sprite_id: template.sprite_id.to_string(),
    });
    builder.add(Collider { radius: 16.0 });
    builder.add(Cooldowns::default());
    builder.add(StatusEffects::default());

    // Add monster traits
    if !template.traits.is_empty() {
        builder.add(MonsterTraits {
            traits: template.traits.clone(),
        });
    }

    // Add regeneration if monster has that trait
    if template.traits.contains(&MonsterTrait::Regeneration) {
        builder.add(Regeneration {
            hp_per_second: (template.hp as f32 * 0.05) as i32, // 5% HP per second
            disabled_until: None,
        });
    }

    // Mark as boss if applicable
    if is_boss {
        builder.add(Boss {
            respawn_cooldown_hours: BOSS_RESPAWN_HOURS,
        });
    }

    world.spawn(builder.build())
}

/// Despawn a monster entity (when it dies)
pub fn despawn_monster(world: &mut World, entity: Entity) {
    if world.despawn(entity).is_ok() {
        // Successfully despawned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_point_manager() {
        let mut manager = SpawnPointManager::new();

        let id1 = manager.add_spawn_point(MonsterType::GiantRat, 100.0, 100.0, false);
        let id2 = manager.add_spawn_point(MonsterType::Goblin, 200.0, 200.0, false);

        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(manager.spawn_points.len(), 2);
    }

    #[test]
    fn test_spawn_monster() {
        let mut world = World::new();

        let entity = spawn_monster(&mut world, MonsterType::GiantRat, 100.0, 100.0, false);

        // Check that monster has required components
        assert!(world.get::<&Position>(entity).is_ok());
        assert!(world.get::<&Health>(entity).is_ok());
        assert!(world.get::<&Monster>(entity).is_ok());
        assert!(world.get::<&AIController>(entity).is_ok());

        // Check position
        if let Ok(pos) = world.get::<&Position>(entity) {
            assert_eq!(pos.x, 100.0);
            assert_eq!(pos.y, 100.0);
        };
    }

    #[test]
    fn test_boss_spawn() {
        let mut world = World::new();

        let entity = spawn_monster(&mut world, MonsterType::Lich, 100.0, 100.0, true);

        // Check that boss has Boss component
        assert!(world.get::<&Boss>(entity).is_ok());

        // Check boss respawn cooldown
        if let Ok(boss) = world.get::<&Boss>(entity) {
            assert_eq!(boss.respawn_cooldown_hours, 24);
        };
    }

    #[test]
    fn test_respawn_cooldown() {
        let mut manager = SpawnPointManager::new();

        let _id = manager.add_spawn_point(MonsterType::GiantRat, 100.0, 100.0, false);

        // Check respawn cooldown was set correctly
        assert_eq!(manager.spawn_points[0].respawn_cooldown_seconds, 300); // 5 minutes

        // Boss respawn
        let _boss_id = manager.add_spawn_point(MonsterType::Lich, 200.0, 200.0, true);
        assert_eq!(manager.spawn_points[1].respawn_cooldown_seconds, 24 * 3600); // 24 hours
    }

    #[test]
    fn test_initial_spawn() {
        let mut world = World::new();
        let mut manager = SpawnPointManager::new();

        manager.add_spawn_point(MonsterType::GiantRat, 100.0, 100.0, false);

        // Update should spawn monsters on first tick
        manager.update(&mut world);

        // Check that monster was spawned
        assert!(manager.spawn_points[0].spawned_entity.is_some());
        assert!(manager.spawn_points[0].last_spawn_time.is_some());

        // Check that entity exists in world
        let entity = manager.spawn_points[0].spawned_entity.unwrap();
        assert!(world.get::<&Monster>(entity).is_ok());
    }
}
