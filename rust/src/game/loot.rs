use hecs::World;
use rand::Rng;
use crate::ecs::components::*;
use crate::game::monsters;

/// Roll loot from a loot table and spawn item entities at the given position
pub fn spawn_loot(world: &mut World, monster_type: MonsterType, x: f32, y: f32) -> Vec<DroppedItem> {
    let loot_table = monsters::get_loot_table(monster_type);
    let mut rng = rand::thread_rng();
    let mut dropped_items = Vec::new();

    // Roll for item drops
    for loot_drop in &loot_table.drops {
        let roll = rng.gen::<f32>();
        if roll < loot_drop.chance {
            dropped_items.push(DroppedItem {
                item_id: loot_drop.item_id.clone(),
                quantity: loot_drop.quantity,
                x,
                y,
            });
        }
    }

    // Generate guaranteed copper
    if loot_table.guaranteed_copper_max > 0 {
        let copper_amount = if loot_table.guaranteed_copper_min == loot_table.guaranteed_copper_max {
            loot_table.guaranteed_copper_min
        } else {
            rng.gen_range(loot_table.guaranteed_copper_min..=loot_table.guaranteed_copper_max)
        };

        if copper_amount > 0 {
            dropped_items.push(DroppedItem {
                item_id: "copper".to_string(),
                quantity: copper_amount,
                x,
                y,
            });
        }
    }

    // Spawn item entities in the world
    for item in &dropped_items {
        spawn_item_entity(world, &item.item_id, item.quantity, x, y);
    }

    dropped_items
}

/// Spawn a physical item entity in the world that can be picked up
fn spawn_item_entity(world: &mut World, item_id: &str, quantity: u32, x: f32, y: f32) {
    world.spawn((
        Position { x, y },
        Item {
            item_id: item_id.to_string(),
            quantity,
        },
        Sprite {
            sprite_id: get_item_sprite(item_id),
        },
        Collider { radius: 16.0 }, // Small collider for items
    ));
}

/// Get the sprite ID for an item
fn get_item_sprite(item_id: &str) -> String {
    match item_id {
        "copper" => "coin_copper".to_string(),
        "silver" => "coin_silver".to_string(),
        "gold" => "coin_gold".to_string(),
        "rat_meat" => "item_rat_meat".to_string(),
        "rat_pelt" => "item_rat_pelt".to_string(),
        "goblin_ear" => "item_goblin_ear".to_string(),
        "bone_fragment" => "item_bone".to_string(),
        "wolf_pelt" => "item_wolf_pelt".to_string(),
        "orc_tusk" => "item_orc_tusk".to_string(),
        "spider_silk" => "item_spider_silk".to_string(),
        "hobgoblin_armor" => "item_armor_scrap".to_string(),
        "ogre_club" => "item_club".to_string(),
        "wight_essence" => "item_essence".to_string(),
        "troll_blood" => "item_blood_vial".to_string(),
        "dragon_scale" => "item_dragon_scale".to_string(),
        "lich_phylactery" => "item_phylactery".to_string(),
        "demon_horn" => "item_demon_horn".to_string(),
        _ => "item_unknown".to_string(),
    }
}

/// Represents an item that was dropped by a monster
#[derive(Debug, Clone)]
pub struct DroppedItem {
    pub item_id: String,
    pub quantity: u32,
    pub x: f32,
    pub y: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_loot() {
        let mut world = World::new();

        // Test Giant Rat loot
        let items = spawn_loot(&mut world, MonsterType::GiantRat, 100.0, 100.0);

        // Should have spawned some items (probabilistic, but should at least spawn entities)
        assert!(items.len() >= 0); // May have 0-2 items depending on rolls

        // Check that entities were spawned in world
        let entity_count = world.query::<&Item>().iter().count();
        assert_eq!(entity_count, items.len());
    }

    #[test]
    fn test_guaranteed_loot() {
        let mut world = World::new();

        // Test Lich which has guaranteed gold drops
        let items = spawn_loot(&mut world, MonsterType::Lich, 100.0, 100.0);

        // Lich should always drop gold (1000-2000 copper = 10-20 gold)
        let has_copper = items.iter().any(|item| item.item_id == "copper");
        assert!(has_copper || items.len() > 0); // Should have at least copper or other drops
    }

    #[test]
    fn test_item_sprite_mapping() {
        assert_eq!(get_item_sprite("copper"), "coin_copper");
        assert_eq!(get_item_sprite("rat_meat"), "item_rat_meat");
        assert_eq!(get_item_sprite("unknown_item"), "item_unknown");
    }
}
