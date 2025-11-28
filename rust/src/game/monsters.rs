use crate::ecs::components::*;

/// Monster template containing all stats and properties
#[derive(Debug, Clone)]
pub struct MonsterTemplate {
    pub monster_type: MonsterType,
    pub name: &'static str,
    pub level: u32,
    pub xp_reward: u32,
    pub hp: i32,
    pub ac: i32,
    pub stats: Stats,
    pub speed: f32,
    pub attack_damage_dice: (u32, u32), // (num_dice, die_size)
    pub attack_damage_bonus: i32,
    pub attack_cooldown_ms: u64,
    pub detection_range_tiles: f32,
    pub traits: Vec<MonsterTrait>,
    pub sprite_id: &'static str,
}

impl MonsterTemplate {
    /// Get template for a specific monster type
    pub fn get(monster_type: MonsterType) -> Self {
        match monster_type {
            MonsterType::GiantRat => Self::giant_rat(),
            MonsterType::Goblin => Self::goblin(),
            MonsterType::Skeleton => Self::skeleton(),
            MonsterType::Wolf => Self::wolf(),
            MonsterType::OrcWarrior => Self::orc_warrior(),
            MonsterType::Zombie => Self::zombie(),
            MonsterType::GiantSpider => Self::giant_spider(),
            MonsterType::Hobgoblin => Self::hobgoblin(),
            MonsterType::Ogre => Self::ogre(),
            MonsterType::Wight => Self::wight(),
            MonsterType::Troll => Self::troll(),
            MonsterType::YoungDragon => Self::young_dragon(),
            MonsterType::Lich => Self::lich(),
            MonsterType::DemonLord => Self::demon_lord(),
        }
    }

    // Level 1 Monsters

    fn giant_rat() -> Self {
        Self {
            monster_type: MonsterType::GiantRat,
            name: "Giant Rat",
            level: 1,
            xp_reward: 50,
            hp: 7,
            ac: 12,
            stats: Stats { str: 8, dex: 15, con: 11, int: 2, wis: 10, cha: 4 },
            speed: 200.0,
            attack_damage_dice: (1, 4),
            attack_damage_bonus: 0,
            attack_cooldown_ms: 1000,
            detection_range_tiles: 12.0,
            traits: vec![MonsterTrait::PackCreature],
            sprite_id: "giant_rat",
        }
    }

    fn goblin() -> Self {
        Self {
            monster_type: MonsterType::Goblin,
            name: "Goblin",
            level: 1,
            xp_reward: 50,
            hp: 7,
            ac: 13,
            stats: Stats { str: 8, dex: 14, con: 10, int: 10, wis: 8, cha: 8 },
            speed: 180.0,
            attack_damage_dice: (1, 4),
            attack_damage_bonus: 1,
            attack_cooldown_ms: 1000,
            detection_range_tiles: 15.0,
            traits: vec![MonsterTrait::PackCreature],
            sprite_id: "goblin",
        }
    }

    fn skeleton() -> Self {
        Self {
            monster_type: MonsterType::Skeleton,
            name: "Skeleton",
            level: 1,
            xp_reward: 50,
            hp: 13,
            ac: 13,
            stats: Stats { str: 10, dex: 14, con: 15, int: 6, wis: 8, cha: 5 },
            speed: 160.0,
            attack_damage_dice: (1, 6),
            attack_damage_bonus: 0,
            attack_cooldown_ms: 1200,
            detection_range_tiles: 18.0,
            traits: vec![MonsterTrait::UndeadResilience],
            sprite_id: "skeleton",
        }
    }

    fn wolf() -> Self {
        Self {
            monster_type: MonsterType::Wolf,
            name: "Wolf",
            level: 1,
            xp_reward: 50,
            hp: 11,
            ac: 13,
            stats: Stats { str: 12, dex: 15, con: 12, int: 3, wis: 12, cha: 6 },
            speed: 240.0,
            attack_damage_dice: (1, 6),
            attack_damage_bonus: 1,
            attack_cooldown_ms: 900,
            detection_range_tiles: 20.0,
            traits: vec![MonsterTrait::PackCreature, MonsterTrait::PackTactics],
            sprite_id: "wolf",
        }
    }

    // Level 2-3 Monsters

    fn orc_warrior() -> Self {
        Self {
            monster_type: MonsterType::OrcWarrior,
            name: "Orc Warrior",
            level: 2,
            xp_reward: 100,
            hp: 15,
            ac: 13,
            stats: Stats { str: 16, dex: 12, con: 16, int: 7, wis: 11, cha: 10 },
            speed: 200.0,
            attack_damage_dice: (1, 12),
            attack_damage_bonus: 3,
            attack_cooldown_ms: 1300,
            detection_range_tiles: 16.0,
            traits: vec![MonsterTrait::PackCreature],
            sprite_id: "orc_warrior",
        }
    }

    fn zombie() -> Self {
        Self {
            monster_type: MonsterType::Zombie,
            name: "Zombie",
            level: 2,
            xp_reward: 100,
            hp: 22,
            ac: 8,
            stats: Stats { str: 13, dex: 6, con: 16, int: 3, wis: 6, cha: 5 },
            speed: 120.0,
            attack_damage_dice: (1, 6),
            attack_damage_bonus: 1,
            attack_cooldown_ms: 1500,
            detection_range_tiles: 10.0,
            traits: vec![MonsterTrait::UndeadResilience, MonsterTrait::UndeadFortitude],
            sprite_id: "zombie",
        }
    }

    fn giant_spider() -> Self {
        Self {
            monster_type: MonsterType::GiantSpider,
            name: "Giant Spider",
            level: 2,
            xp_reward: 100,
            hp: 26,
            ac: 14,
            stats: Stats { str: 14, dex: 16, con: 12, int: 2, wis: 11, cha: 4 },
            speed: 200.0,
            attack_damage_dice: (1, 6),
            attack_damage_bonus: 2,
            attack_cooldown_ms: 1000,
            detection_range_tiles: 14.0,
            traits: vec![MonsterTrait::WebAttack],
            sprite_id: "giant_spider",
        }
    }

    fn hobgoblin() -> Self {
        Self {
            monster_type: MonsterType::Hobgoblin,
            name: "Hobgoblin",
            level: 3,
            xp_reward: 200,
            hp: 11,
            ac: 16,
            stats: Stats { str: 13, dex: 12, con: 12, int: 10, wis: 10, cha: 9 },
            speed: 200.0,
            attack_damage_dice: (1, 8),
            attack_damage_bonus: 1,
            attack_cooldown_ms: 1000,
            detection_range_tiles: 18.0,
            traits: vec![MonsterTrait::MartialAdvantage],
            sprite_id: "hobgoblin",
        }
    }

    // Level 4-5 Monsters

    fn ogre() -> Self {
        Self {
            monster_type: MonsterType::Ogre,
            name: "Ogre",
            level: 4,
            xp_reward: 400,
            hp: 59,
            ac: 11,
            stats: Stats { str: 19, dex: 8, con: 16, int: 5, wis: 7, cha: 7 },
            speed: 220.0,
            attack_damage_dice: (2, 8),
            attack_damage_bonus: 4,
            attack_cooldown_ms: 1800,
            detection_range_tiles: 14.0,
            traits: vec![MonsterTrait::PowerfulBuild],
            sprite_id: "ogre",
        }
    }

    fn wight() -> Self {
        Self {
            monster_type: MonsterType::Wight,
            name: "Wight",
            level: 5,
            xp_reward: 800,
            hp: 45,
            ac: 14,
            stats: Stats { str: 15, dex: 14, con: 16, int: 10, wis: 13, cha: 15 },
            speed: 200.0,
            attack_damage_dice: (1, 8),
            attack_damage_bonus: 2,
            attack_cooldown_ms: 1000,
            detection_range_tiles: 20.0,
            traits: vec![MonsterTrait::UndeadResilience, MonsterTrait::UndeadCommander],
            sprite_id: "wight",
        }
    }

    fn troll() -> Self {
        Self {
            monster_type: MonsterType::Troll,
            name: "Troll",
            level: 5,
            xp_reward: 800,
            hp: 84,
            ac: 15,
            stats: Stats { str: 18, dex: 13, con: 20, int: 7, wis: 9, cha: 7 },
            speed: 200.0,
            attack_damage_dice: (1, 6),
            attack_damage_bonus: 4,
            attack_cooldown_ms: 800,
            detection_range_tiles: 25.0,
            traits: vec![MonsterTrait::Regeneration, MonsterTrait::KeenSmell],
            sprite_id: "troll",
        }
    }

    // Boss Monsters

    fn young_dragon() -> Self {
        Self {
            monster_type: MonsterType::YoungDragon,
            name: "Young Dragon",
            level: 8,
            xp_reward: 2000,
            hp: 178,
            ac: 18,
            stats: Stats { str: 19, dex: 10, con: 21, int: 12, wis: 11, cha: 15 },
            speed: 220.0,
            attack_damage_dice: (2, 10),
            attack_damage_bonus: 4,
            attack_cooldown_ms: 1500,
            detection_range_tiles: 30.0,
            traits: vec![MonsterTrait::DragonScales, MonsterTrait::FrightfulPresence],
            sprite_id: "young_dragon",
        }
    }

    fn lich() -> Self {
        Self {
            monster_type: MonsterType::Lich,
            name: "Lich",
            level: 10,
            xp_reward: 3000,
            hp: 135,
            ac: 17,
            stats: Stats { str: 11, dex: 16, con: 16, int: 20, wis: 14, cha: 16 },
            speed: 180.0,
            attack_damage_dice: (3, 6),
            attack_damage_bonus: 0,
            attack_cooldown_ms: 1000,
            detection_range_tiles: 35.0,
            traits: vec![
                MonsterTrait::MagicResistance,
                MonsterTrait::Phylactery,
                MonsterTrait::UndeadCommander,
            ],
            sprite_id: "lich",
        }
    }

    fn demon_lord() -> Self {
        Self {
            monster_type: MonsterType::DemonLord,
            name: "Demon Lord",
            level: 12,
            xp_reward: 5000,
            hp: 262,
            ac: 19,
            stats: Stats { str: 22, dex: 15, con: 25, int: 16, wis: 13, cha: 18 },
            speed: 240.0,
            attack_damage_dice: (2, 6),
            attack_damage_bonus: 6,
            attack_cooldown_ms: 1200,
            detection_range_tiles: 40.0,
            traits: vec![MonsterTrait::DemonResilience, MonsterTrait::AuraOfTerror],
            sprite_id: "demon_lord",
        }
    }
}

/// Get loot table for a monster type
pub fn get_loot_table(monster_type: MonsterType) -> LootTable {
    match monster_type {
        MonsterType::GiantRat => LootTable {
            drops: vec![
                LootDrop { item_id: "rat_meat".to_string(), quantity: 1, chance: 0.30 },
                LootDrop { item_id: "rat_pelt".to_string(), quantity: 1, chance: 0.10 },
            ],
            guaranteed_copper_min: 0,
            guaranteed_copper_max: 0,
        },
        MonsterType::Goblin => LootTable {
            drops: vec![
                LootDrop { item_id: "rusty_dagger".to_string(), quantity: 1, chance: 0.30 },
                LootDrop { item_id: "healing_potion_minor".to_string(), quantity: 1, chance: 0.20 },
                LootDrop { item_id: "goblin_ear".to_string(), quantity: 1, chance: 0.10 },
            ],
            guaranteed_copper_min: 1,
            guaranteed_copper_max: 5,
        },
        MonsterType::Skeleton => LootTable {
            drops: vec![
                LootDrop { item_id: "bone_shard".to_string(), quantity: 1, chance: 0.50 },
                LootDrop { item_id: "rusty_shortsword".to_string(), quantity: 1, chance: 0.30 },
                LootDrop { item_id: "ancient_coin".to_string(), quantity: 1, chance: 0.15 },
                LootDrop { item_id: "skeleton_key".to_string(), quantity: 1, chance: 0.05 },
            ],
            guaranteed_copper_min: 0,
            guaranteed_copper_max: 0,
        },
        MonsterType::Wolf => LootTable {
            drops: vec![
                LootDrop { item_id: "wolf_pelt".to_string(), quantity: 1, chance: 0.70 },
                LootDrop { item_id: "wolf_meat".to_string(), quantity: 1, chance: 0.20 },
                LootDrop { item_id: "wolf_fang".to_string(), quantity: 1, chance: 0.10 },
            ],
            guaranteed_copper_min: 0,
            guaranteed_copper_max: 0,
        },
        MonsterType::OrcWarrior => LootTable {
            drops: vec![
                LootDrop { item_id: "greataxe".to_string(), quantity: 1, chance: 0.30 },
                LootDrop { item_id: "hide_armor".to_string(), quantity: 1, chance: 0.20 },
                LootDrop { item_id: "orcish_trinket".to_string(), quantity: 1, chance: 0.10 },
            ],
            guaranteed_copper_min: 5,
            guaranteed_copper_max: 15,
        },
        MonsterType::Zombie => LootTable {
            drops: vec![
                LootDrop { item_id: "rotten_flesh".to_string(), quantity: 1, chance: 0.60 },
                LootDrop { item_id: "tattered_cloth".to_string(), quantity: 1, chance: 0.20 },
                LootDrop { item_id: "brain".to_string(), quantity: 1, chance: 0.05 },
            ],
            guaranteed_copper_min: 0,
            guaranteed_copper_max: 3,
        },
        MonsterType::GiantSpider => LootTable {
            drops: vec![
                LootDrop { item_id: "spider_silk".to_string(), quantity: 2, chance: 0.80 },
                LootDrop { item_id: "poison_gland".to_string(), quantity: 1, chance: 0.40 },
                LootDrop { item_id: "giant_fang".to_string(), quantity: 1, chance: 0.20 },
                LootDrop { item_id: "spider_egg".to_string(), quantity: 1, chance: 0.10 },
            ],
            guaranteed_copper_min: 0,
            guaranteed_copper_max: 0,
        },
        MonsterType::Hobgoblin => LootTable {
            drops: vec![
                LootDrop { item_id: "longsword".to_string(), quantity: 1, chance: 0.40 },
                LootDrop { item_id: "chain_mail".to_string(), quantity: 1, chance: 0.30 },
                LootDrop { item_id: "military_badge".to_string(), quantity: 1, chance: 0.20 },
                LootDrop { item_id: "silver_coin".to_string(), quantity: 1, chance: 0.10 },
            ],
            guaranteed_copper_min: 10,
            guaranteed_copper_max: 30,
        },
        MonsterType::Ogre => LootTable {
            drops: vec![
                LootDrop { item_id: "greatclub".to_string(), quantity: 1, chance: 0.40 },
                LootDrop { item_id: "giants_toe".to_string(), quantity: 1, chance: 0.30 },
                LootDrop { item_id: "ogre_hide".to_string(), quantity: 1, chance: 0.20 },
                LootDrop { item_id: "silver_coin".to_string(), quantity: 1, chance: 0.10 },
            ],
            guaranteed_copper_min: 20,
            guaranteed_copper_max: 50,
        },
        MonsterType::Wight => LootTable {
            drops: vec![
                LootDrop { item_id: "enchanted_longsword".to_string(), quantity: 1, chance: 0.50 },
                LootDrop { item_id: "wight_crown".to_string(), quantity: 1, chance: 0.30 },
                LootDrop { item_id: "soul_gem".to_string(), quantity: 1, chance: 0.20 },
                LootDrop { item_id: "silver_coin".to_string(), quantity: 4, chance: 0.10 },
            ],
            guaranteed_copper_min: 30,
            guaranteed_copper_max: 80,
        },
        MonsterType::Troll => LootTable {
            drops: vec![
                LootDrop { item_id: "troll_blood".to_string(), quantity: 2, chance: 0.90 },
                LootDrop { item_id: "troll_hide".to_string(), quantity: 1, chance: 0.60 },
                LootDrop { item_id: "troll_heart".to_string(), quantity: 1, chance: 0.20 },
                LootDrop { item_id: "silver_coin".to_string(), quantity: 6, chance: 0.10 },
            ],
            guaranteed_copper_min: 50,
            guaranteed_copper_max: 100,
        },
        MonsterType::YoungDragon => LootTable {
            drops: vec![
                LootDrop { item_id: "dragon_scales".to_string(), quantity: 4, chance: 1.0 },
                LootDrop { item_id: "gem".to_string(), quantity: 8, chance: 1.0 },
                LootDrop { item_id: "magic_item".to_string(), quantity: 1, chance: 0.50 },
                LootDrop { item_id: "rare_weapon".to_string(), quantity: 1, chance: 0.25 },
            ],
            guaranteed_copper_min: 100,
            guaranteed_copper_max: 500,
        },
        MonsterType::Lich => LootTable {
            drops: vec![
                LootDrop { item_id: "spellbook".to_string(), quantity: 1, chance: 1.0 },
                LootDrop { item_id: "lich_crown".to_string(), quantity: 1, chance: 1.0 },
                LootDrop { item_id: "staff_of_power".to_string(), quantity: 1, chance: 1.0 },
                LootDrop { item_id: "magic_item".to_string(), quantity: 8, chance: 0.75 },
            ],
            guaranteed_copper_min: 200,
            guaranteed_copper_max: 800,
        },
        MonsterType::DemonLord => LootTable {
            drops: vec![
                LootDrop { item_id: "demon_heart".to_string(), quantity: 1, chance: 1.0 },
                LootDrop { item_id: "infernal_blade".to_string(), quantity: 1, chance: 1.0 },
                LootDrop { item_id: "magic_item".to_string(), quantity: 18, chance: 0.90 },
                LootDrop { item_id: "unique_artifact".to_string(), quantity: 1, chance: 1.0 },
            ],
            guaranteed_copper_min: 500,
            guaranteed_copper_max: 2000,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monster_templates() {
        // Test that all monsters have valid stats
        for monster_type in [
            MonsterType::GiantRat,
            MonsterType::Goblin,
            MonsterType::Skeleton,
            MonsterType::Wolf,
        ] {
            let template = MonsterTemplate::get(monster_type);
            assert!(template.hp > 0);
            assert!(template.ac > 0);
            assert!(template.speed > 0.0);
        }
    }

    #[test]
    fn test_loot_tables() {
        // Test that all monsters have loot tables
        for monster_type in [MonsterType::GiantRat, MonsterType::Goblin] {
            let loot = get_loot_table(monster_type);
            // Boss monsters should have guaranteed copper
            if monster_type == MonsterType::YoungDragon {
                assert!(loot.guaranteed_copper_min > 0);
            }
        }
    }
}
