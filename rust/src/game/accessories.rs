use crate::ecs::components::*;

/// Get accessory stats by accessory type
pub fn get_accessory_stats(accessory_type: AccessoryType) -> AccessoryStats {
    match accessory_type {
        AccessoryType::RingOfProtection => ring_of_protection(),
        AccessoryType::AmuletOfHealth => amulet_of_health(),
        AccessoryType::RingOfStrength => ring_of_strength(),
        AccessoryType::RingOfDexterity => ring_of_dexterity(),
        AccessoryType::RingOfConstitution => ring_of_constitution(),
        AccessoryType::CloakOfResistance => cloak_of_resistance(),
        AccessoryType::BootsOfSpeed => boots_of_speed(),
        AccessoryType::GlovesOfOgrePower => gloves_of_ogre_power(),
        AccessoryType::HeadbandOfIntellect => headband_of_intellect(),
        AccessoryType::PeriaptOfWisdom => periapt_of_wisdom(),
        AccessoryType::RingOfRegeneration => ring_of_regeneration(),
        AccessoryType::AmuletOfLifeDrain => amulet_of_life_drain(),
        AccessoryType::RingOfFireResistance => ring_of_fire_resistance(),
        AccessoryType::RingOfSpellStoring => ring_of_spell_storing(),
        AccessoryType::BootsOfElvenkind => boots_of_elvenkind(),
        AccessoryType::CloakOfInvisibility => cloak_of_invisibility(),
        AccessoryType::BeltOfGiantStrength => belt_of_giant_strength(),
        AccessoryType::RingOfFeatherFalling => ring_of_feather_falling(),
        AccessoryType::AmuletOfTheDevout => amulet_of_the_devout(),
        AccessoryType::RingOfEvasion => ring_of_evasion(),
    }
}

/// Get accessory value in gold
pub fn get_accessory_value(accessory_type: AccessoryType) -> u32 {
    match accessory_type {
        // Common accessories: 50-150 gold
        AccessoryType::RingOfProtection => 100,
        AccessoryType::AmuletOfHealth => 150,
        AccessoryType::RingOfStrength => 100,
        AccessoryType::RingOfDexterity => 100,
        AccessoryType::RingOfConstitution => 100,
        AccessoryType::CloakOfResistance => 150,
        AccessoryType::BootsOfSpeed => 100,
        AccessoryType::GlovesOfOgrePower => 150,
        AccessoryType::HeadbandOfIntellect => 150,
        AccessoryType::PeriaptOfWisdom => 150,
        // Rare accessories: 300-1000 gold
        AccessoryType::RingOfRegeneration => 500,
        AccessoryType::AmuletOfLifeDrain => 600,
        AccessoryType::RingOfFireResistance => 400,
        AccessoryType::RingOfSpellStoring => 800,
        AccessoryType::BootsOfElvenkind => 300,
        AccessoryType::CloakOfInvisibility => 1000,
        AccessoryType::BeltOfGiantStrength => 800,
        AccessoryType::RingOfFeatherFalling => 400,
        AccessoryType::AmuletOfTheDevout => 700,
        AccessoryType::RingOfEvasion => 900,
    }
}

// Common accessories

fn ring_of_protection() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::RingOfProtection,
        effects: AccessoryEffects {
            ac_bonus: 1,
            ..Default::default()
        },
    }
}

fn amulet_of_health() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::AmuletOfHealth,
        effects: AccessoryEffects {
            hp_bonus: 10,
            ..Default::default()
        },
    }
}

fn ring_of_strength() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::RingOfStrength,
        effects: AccessoryEffects {
            str_bonus: 1,
            ..Default::default()
        },
    }
}

fn ring_of_dexterity() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::RingOfDexterity,
        effects: AccessoryEffects {
            dex_bonus: 1,
            ..Default::default()
        },
    }
}

fn ring_of_constitution() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::RingOfConstitution,
        effects: AccessoryEffects {
            con_bonus: 1,
            ..Default::default()
        },
    }
}

fn cloak_of_resistance() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::CloakOfResistance,
        effects: AccessoryEffects {
            // +1 to saves (implemented separately)
            ..Default::default()
        },
    }
}

fn boots_of_speed() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::BootsOfSpeed,
        effects: AccessoryEffects {
            movement_speed_bonus: 0.10,  // +10% speed
            ..Default::default()
        },
    }
}

fn gloves_of_ogre_power() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::GlovesOfOgrePower,
        effects: AccessoryEffects {
            str_bonus: 2,
            ..Default::default()
        },
    }
}

fn headband_of_intellect() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::HeadbandOfIntellect,
        effects: AccessoryEffects {
            int_bonus: 2,
            ..Default::default()
        },
    }
}

fn periapt_of_wisdom() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::PeriaptOfWisdom,
        effects: AccessoryEffects {
            wis_bonus: 2,
            ..Default::default()
        },
    }
}

// Rare accessories

fn ring_of_regeneration() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::RingOfRegeneration,
        effects: AccessoryEffects {
            hp_regen_per_5s: 1,  // 1 HP per 5 seconds
            ..Default::default()
        },
    }
}

fn amulet_of_life_drain() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::AmuletOfLifeDrain,
        effects: AccessoryEffects {
            lifesteal_percent: 0.20,  // 20% lifesteal
            ..Default::default()
        },
    }
}

fn ring_of_fire_resistance() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::RingOfFireResistance,
        effects: AccessoryEffects {
            fire_resistance: 0.50,  // 50% fire damage reduction
            ..Default::default()
        },
    }
}

fn ring_of_spell_storing() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::RingOfSpellStoring,
        effects: AccessoryEffects {
            // Store 1 spell charge (implemented separately when magic system exists)
            ..Default::default()
        },
    }
}

fn boots_of_elvenkind() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::BootsOfElvenkind,
        effects: AccessoryEffects {
            // Silent movement (stealth bonus, implemented separately)
            ..Default::default()
        },
    }
}

fn cloak_of_invisibility() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::CloakOfInvisibility,
        effects: AccessoryEffects {
            // Turn invisible (special ability, implemented separately)
            ..Default::default()
        },
    }
}

fn belt_of_giant_strength() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::BeltOfGiantStrength,
        effects: AccessoryEffects {
            str_bonus: 4,
            ..Default::default()
        },
    }
}

fn ring_of_feather_falling() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::RingOfFeatherFalling,
        effects: AccessoryEffects {
            // No fall damage (implemented separately, future)
            ..Default::default()
        },
    }
}

fn amulet_of_the_devout() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::AmuletOfTheDevout,
        effects: AccessoryEffects {
            wis_bonus: 1,
            // +1 to spell attacks (implemented separately when magic exists)
            ..Default::default()
        },
    }
}

fn ring_of_evasion() -> AccessoryStats {
    AccessoryStats {
        accessory_type: AccessoryType::RingOfEvasion,
        effects: AccessoryEffects {
            evasion_chance: 0.25,  // 25% chance to dodge attacks
            ..Default::default()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_accessories_defined() {
        let accessories = [
            AccessoryType::RingOfProtection,
            AccessoryType::AmuletOfHealth,
            AccessoryType::RingOfStrength,
            AccessoryType::RingOfDexterity,
            AccessoryType::RingOfConstitution,
            AccessoryType::CloakOfResistance,
            AccessoryType::BootsOfSpeed,
            AccessoryType::GlovesOfOgrePower,
            AccessoryType::HeadbandOfIntellect,
            AccessoryType::PeriaptOfWisdom,
            AccessoryType::RingOfRegeneration,
            AccessoryType::AmuletOfLifeDrain,
            AccessoryType::RingOfFireResistance,
            AccessoryType::RingOfSpellStoring,
            AccessoryType::BootsOfElvenkind,
            AccessoryType::CloakOfInvisibility,
            AccessoryType::BeltOfGiantStrength,
            AccessoryType::RingOfFeatherFalling,
            AccessoryType::AmuletOfTheDevout,
            AccessoryType::RingOfEvasion,
        ];

        for accessory in accessories {
            let stats = get_accessory_stats(accessory);
            // Just check it doesn't panic
            assert!(get_accessory_value(accessory) > 0);
        }
    }

    #[test]
    fn test_stat_bonuses() {
        let ring_str = get_accessory_stats(AccessoryType::RingOfStrength);
        assert_eq!(ring_str.effects.str_bonus, 1);

        let belt = get_accessory_stats(AccessoryType::BeltOfGiantStrength);
        assert_eq!(belt.effects.str_bonus, 4);
    }

    #[test]
    fn test_movement_bonus() {
        let boots = get_accessory_stats(AccessoryType::BootsOfSpeed);
        assert!((boots.effects.movement_speed_bonus - 0.10).abs() < 0.01);
    }

    #[test]
    fn test_regeneration() {
        let ring = get_accessory_stats(AccessoryType::RingOfRegeneration);
        assert_eq!(ring.effects.hp_regen_per_5s, 1);
    }

    #[test]
    fn test_lifesteal() {
        let amulet = get_accessory_stats(AccessoryType::AmuletOfLifeDrain);
        assert!((amulet.effects.lifesteal_percent - 0.20).abs() < 0.01);
    }

    #[test]
    fn test_evasion() {
        let ring = get_accessory_stats(AccessoryType::RingOfEvasion);
        assert!((ring.effects.evasion_chance - 0.25).abs() < 0.01);
    }
}
