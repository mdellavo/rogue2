use crate::ecs::components::*;

pub struct CharacterBuilder {
    species: Species,
    class: CharacterClass,
}

impl CharacterBuilder {
    pub fn new(species: Species, class: CharacterClass) -> Self {
        Self { species, class }
    }

    pub fn build(&self) -> CharacterSheet {
        let base_stats = self.calculate_stats();
        let base_hp = self.calculate_base_hp();
        let hp_bonus = self.class_hp_bonus();

        CharacterSheet {
            species: self.species,
            class: self.class,
            level: 1,
            experience: 0,
            stats: base_stats,
            max_hp: base_hp + hp_bonus,
            ac: self.class_ac(),
            movement_speed: self.species_movement_speed(),
            vision_range: self.species_vision_range(),
            attack_cooldown_ms: self.class_attack_cooldown(),
        }
    }

    fn calculate_stats(&self) -> Stats {
        let mut stats = Stats {
            str: 10,
            dex: 10,
            con: 10,
            int: 10,
            wis: 10,
            cha: 10,
        };

        // Apply species bonuses
        match self.species {
            Species::Human => {
                stats.str += 1;
                stats.dex += 1;
                stats.con += 1;
                stats.int += 1;
                stats.wis += 1;
                stats.cha += 1;
            }
            Species::Elf => {
                stats.dex += 2;
                stats.int += 1;
                stats.con -= 1;
            }
            Species::Dwarf => {
                stats.con += 2;
                stats.str += 1;
                stats.dex -= 1;
            }
            Species::Halfling => {
                stats.dex += 2;
                stats.cha += 1;
                stats.str -= 1;
            }
            Species::HalfOrc => {
                stats.str += 2;
                stats.con += 1;
                stats.int -= 1;
            }
            Species::Gnome => {
                stats.int += 2;
                stats.dex += 1;
                stats.str -= 1;
            }
        }

        // Apply class bonuses
        match self.class {
            CharacterClass::Fighter => {
                stats.str += 2;
                stats.con += 1;
            }
            CharacterClass::Rogue => {
                stats.dex += 2;
                stats.cha += 1;
            }
            CharacterClass::Cleric => {
                stats.wis += 2;
                stats.con += 1;
            }
            CharacterClass::Wizard => {
                stats.int += 2;
                stats.wis += 1;
            }
            CharacterClass::Ranger => {
                stats.dex += 2;
                stats.wis += 1;
            }
            CharacterClass::Barbarian => {
                stats.str += 2;
                stats.con += 2;
            }
        }

        stats
    }

    fn calculate_base_hp(&self) -> i32 {
        match self.species {
            Species::Human => 10,
            Species::Elf => 8,
            Species::Dwarf => 12,
            Species::Halfling => 8,
            Species::HalfOrc => 12,
            Species::Gnome => 8,
        }
    }

    fn class_hp_bonus(&self) -> i32 {
        match self.class {
            CharacterClass::Fighter => 2,
            CharacterClass::Rogue => 0,
            CharacterClass::Cleric => 1,
            CharacterClass::Wizard => -2,
            CharacterClass::Ranger => 1,
            CharacterClass::Barbarian => 4,
        }
    }

    fn class_ac(&self) -> i32 {
        match self.class {
            CharacterClass::Fighter => 16,  // Chain mail
            CharacterClass::Rogue => 14,    // Leather armor
            CharacterClass::Cleric => 15,   // Scale mail
            CharacterClass::Wizard => 11,   // Cloth robes
            CharacterClass::Ranger => 14,   // Leather armor
            CharacterClass::Barbarian => 13, // No armor, CON-based
        }
    }

    fn species_movement_speed(&self) -> f32 {
        match self.species {
            Species::Human => 200.0,
            Species::Elf => 220.0,      // +10%
            Species::Dwarf => 180.0,    // -10%
            Species::Halfling => 190.0, // -5%
            Species::HalfOrc => 210.0,  // +5%
            Species::Gnome => 180.0,    // -10%
        }
    }

    fn species_vision_range(&self) -> f32 {
        match self.species {
            Species::Elf => 24.0,  // Keen Senses: +4 tiles
            _ => 20.0,             // Normal vision
        }
    }

    fn class_attack_cooldown(&self) -> u64 {
        match self.class {
            CharacterClass::Fighter => 800,     // 20% faster
            CharacterClass::Rogue => 700,       // 30% faster
            CharacterClass::Cleric => 1000,     // Normal
            CharacterClass::Wizard => 1200,     // 20% slower
            CharacterClass::Ranger => 1000,     // Normal
            CharacterClass::Barbarian => 1100,  // 10% slower
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharacterSheet {
    pub species: Species,
    pub class: CharacterClass,
    pub level: u32,
    pub experience: u32,
    pub stats: Stats,
    pub max_hp: i32,
    pub ac: i32,
    pub movement_speed: f32,
    pub vision_range: f32,
    pub attack_cooldown_ms: u64,
}

// Racial trait implementations
pub mod traits {
    use super::*;

    pub fn has_damage_resistance(species: Species) -> bool {
        matches!(species, Species::Dwarf)
    }

    pub fn apply_damage_resistance(damage: i32, species: Species) -> i32 {
        if has_damage_resistance(species) {
            // Stone's Endurance: Reduce damage by 1 (minimum 1)
            (damage - 1).max(1)
        } else {
            damage
        }
    }

    pub fn has_lucky(species: Species) -> bool {
        matches!(species, Species::Halfling)
    }

    pub fn can_reroll_ones(roll: i32, species: Species) -> bool {
        has_lucky(species) && roll == 1
    }

    pub fn has_relentless_endurance(species: Species) -> bool {
        matches!(species, Species::HalfOrc)
    }

    pub fn xp_bonus_multiplier(species: Species) -> f32 {
        match species {
            Species::Human => 1.1,  // Versatile: +10% XP
            _ => 1.0,
        }
    }
}

// Class ability implementations
pub mod abilities {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ClassAbility {
        SecondWind,      // Fighter: Heal 25% max HP
        SneakAttack,     // Rogue: Double damage
        HealingWord,     // Cleric: Heal ally 30% max HP
        MagicMissile,    // Wizard: 3 projectiles 1d4+INT
        HuntersMark,     // Ranger: +1d6 damage for 30s
        Rage,            // Barbarian: +2 damage, 50% resist for 10s
    }

    impl ClassAbility {
        pub fn for_class(class: CharacterClass) -> Self {
            match class {
                CharacterClass::Fighter => ClassAbility::SecondWind,
                CharacterClass::Rogue => ClassAbility::SneakAttack,
                CharacterClass::Cleric => ClassAbility::HealingWord,
                CharacterClass::Wizard => ClassAbility::MagicMissile,
                CharacterClass::Ranger => ClassAbility::HuntersMark,
                CharacterClass::Barbarian => ClassAbility::Rage,
            }
        }

        pub fn cooldown_ms(&self) -> u64 {
            match self {
                ClassAbility::SecondWind => 60000,    // 60 seconds
                ClassAbility::SneakAttack => 10000,   // 10 seconds
                ClassAbility::HealingWord => 30000,   // 30 seconds
                ClassAbility::MagicMissile => 15000,  // 15 seconds
                ClassAbility::HuntersMark => 45000,   // 45 seconds
                ClassAbility::Rage => 60000,          // 60 seconds
            }
        }

        pub fn name(&self) -> &'static str {
            match self {
                ClassAbility::SecondWind => "Second Wind",
                ClassAbility::SneakAttack => "Sneak Attack",
                ClassAbility::HealingWord => "Healing Word",
                ClassAbility::MagicMissile => "Magic Missile",
                ClassAbility::HuntersMark => "Hunter's Mark",
                ClassAbility::Rage => "Rage",
            }
        }
    }
}

// Experience and leveling
pub mod progression {
    use super::*;

    pub fn xp_for_level(level: u32) -> u32 {
        if level <= 1 {
            0
        } else {
            // Exponential curve: 300, 900, 2700, 8100, etc.
            300 * 3_u32.pow(level - 2)
        }
    }

    pub fn level_from_xp(xp: u32) -> u32 {
        let mut level = 1;
        while level < 20 && xp >= xp_for_level(level + 1) {
            level += 1;
        }
        level
    }

    pub fn add_xp(current_xp: u32, gained_xp: u32, species: Species) -> u32 {
        let multiplier = traits::xp_bonus_multiplier(species);
        let bonus_xp = (gained_xp as f32 * multiplier) as u32;
        current_xp + bonus_xp
    }

    pub fn apply_level_up(_stats: &mut Stats, max_hp: &mut i32) {
        // On level up: +5 HP
        // Player will choose which 2 stats to increase by +1 (future feature)
        *max_hp += 5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_fighter() {
        let builder = CharacterBuilder::new(Species::Human, CharacterClass::Fighter);
        let sheet = builder.build();

        assert_eq!(sheet.stats.str, 13); // 10 + 1 (human) + 2 (fighter)
        assert_eq!(sheet.stats.con, 12); // 10 + 1 (human) + 1 (fighter)
        assert_eq!(sheet.max_hp, 12);    // 10 (human) + 2 (fighter)
        assert_eq!(sheet.ac, 16);
        assert_eq!(sheet.movement_speed, 200.0);
    }

    #[test]
    fn test_elf_ranger() {
        let builder = CharacterBuilder::new(Species::Elf, CharacterClass::Ranger);
        let sheet = builder.build();

        assert_eq!(sheet.stats.dex, 14); // 10 + 2 (elf) + 2 (ranger)
        assert_eq!(sheet.stats.con, 9);  // 10 - 1 (elf)
        assert_eq!(sheet.max_hp, 9);     // 8 (elf) + 1 (ranger)
        assert_eq!(sheet.vision_range, 24.0); // Keen Senses
        assert_eq!(sheet.movement_speed, 220.0); // +10% faster
    }

    #[test]
    fn test_xp_progression() {
        assert_eq!(progression::xp_for_level(2), 300);
        assert_eq!(progression::xp_for_level(3), 900);
        assert_eq!(progression::xp_for_level(4), 2700);

        assert_eq!(progression::level_from_xp(0), 1);
        assert_eq!(progression::level_from_xp(299), 1);
        assert_eq!(progression::level_from_xp(300), 2);
        assert_eq!(progression::level_from_xp(900), 3);
    }
}
