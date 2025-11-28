use crate::ecs::components::*;

/// Get weapon stats by weapon type
pub fn get_weapon_stats(weapon_type: WeaponType) -> WeaponStats {
    match weapon_type {
        WeaponType::Dagger => dagger(),
        WeaponType::Shortsword => shortsword(),
        WeaponType::Longsword => longsword(),
        WeaponType::Mace => mace(),
        WeaponType::Battleaxe => battleaxe(),
        WeaponType::Warhammer => warhammer(),
        WeaponType::Greatsword => greatsword(),
        WeaponType::Greataxe => greataxe(),
        WeaponType::Maul => maul(),
        WeaponType::Quarterstaff => quarterstaff(),
        WeaponType::Shortbow => shortbow(),
        WeaponType::Longbow => longbow(),
        WeaponType::LightCrossbow => light_crossbow(),
        WeaponType::HeavyCrossbow => heavy_crossbow(),
        WeaponType::Wand => wand(),
        WeaponType::Staff => staff(),
    }
}

/// Get weapon value in gold
pub fn get_weapon_value(weapon_type: WeaponType) -> u32 {
    match weapon_type {
        WeaponType::Dagger => 2,
        WeaponType::Shortsword => 10,
        WeaponType::Longsword => 15,
        WeaponType::Mace => 12,
        WeaponType::Battleaxe => 18,
        WeaponType::Warhammer => 20,
        WeaponType::Greatsword => 50,
        WeaponType::Greataxe => 45,
        WeaponType::Maul => 55,
        WeaponType::Quarterstaff => 5,
        WeaponType::Shortbow => 25,
        WeaponType::Longbow => 50,
        WeaponType::LightCrossbow => 35,
        WeaponType::HeavyCrossbow => 75,
        WeaponType::Wand => 30,
        WeaponType::Staff => 40,
    }
}

// One-handed melee weapons

fn dagger() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Dagger,
        damage_dice: (1, 4),
        damage_bonus: 0,
        attack_speed: 0.7,
        range: 1.0,
        is_two_handed: false,
        is_finesse: true,
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn shortsword() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Shortsword,
        damage_dice: (1, 6),
        damage_bonus: 0,
        attack_speed: 0.9,
        range: 1.2,
        is_two_handed: false,
        is_finesse: true,
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn longsword() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Longsword,
        damage_dice: (1, 8),
        damage_bonus: 0,
        attack_speed: 1.0,
        range: 1.5,
        is_two_handed: false,
        is_finesse: false,
        is_versatile: true,
        versatile_damage: Some((1, 10)),
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn mace() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Mace,
        damage_dice: (1, 6),
        damage_bonus: 0,
        attack_speed: 1.0,
        range: 1.3,
        is_two_handed: false,
        is_finesse: false,
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn battleaxe() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Battleaxe,
        damage_dice: (1, 8),
        damage_bonus: 0,
        attack_speed: 1.1,
        range: 1.4,
        is_two_handed: false,
        is_finesse: false,
        is_versatile: true,
        versatile_damage: Some((1, 10)),
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn warhammer() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Warhammer,
        damage_dice: (1, 8),
        damage_bonus: 0,
        attack_speed: 1.2,
        range: 1.3,
        is_two_handed: false,
        is_finesse: false,
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

// Two-handed melee weapons

fn greatsword() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Greatsword,
        damage_dice: (2, 6),
        damage_bonus: 0,
        attack_speed: 1.3,
        range: 2.0,
        is_two_handed: true,
        is_finesse: false,
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn greataxe() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Greataxe,
        damage_dice: (1, 12),
        damage_bonus: 0,
        attack_speed: 1.4,
        range: 1.8,
        is_two_handed: true,
        is_finesse: false,
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn maul() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Maul,
        damage_dice: (2, 6),
        damage_bonus: 0,
        attack_speed: 1.5,
        range: 1.6,
        is_two_handed: true,
        is_finesse: false,
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn quarterstaff() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Quarterstaff,
        damage_dice: (1, 6),
        damage_bonus: 0,
        attack_speed: 0.9,
        range: 1.8,
        is_two_handed: false,
        is_finesse: false,
        is_versatile: true,
        versatile_damage: Some((1, 8)),
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

// Ranged weapons

fn shortbow() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Shortbow,
        damage_dice: (1, 6),
        damage_bonus: 0,
        attack_speed: 1.0,
        range: 15.0,
        is_two_handed: true,
        is_finesse: true,  // DEX-based
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn longbow() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Longbow,
        damage_dice: (1, 8),
        damage_bonus: 0,
        attack_speed: 1.2,
        range: 20.0,
        is_two_handed: true,
        is_finesse: true,  // DEX-based
        is_versatile: false,
        versatile_damage: None,
        str_requirement: Some(13),
        enchantments: Vec::new(),
    }
}

fn light_crossbow() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::LightCrossbow,
        damage_dice: (1, 8),
        damage_bonus: 0,
        attack_speed: 1.5,
        range: 16.0,
        is_two_handed: true,
        is_finesse: false,
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn heavy_crossbow() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::HeavyCrossbow,
        damage_dice: (1, 10),
        damage_bonus: 0,
        attack_speed: 2.0,
        range: 18.0,
        is_two_handed: true,
        is_finesse: false,
        is_versatile: false,
        versatile_damage: None,
        str_requirement: Some(15),
        enchantments: Vec::new(),
    }
}

// Magic weapons

fn wand() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Wand,
        damage_dice: (1, 4),
        damage_bonus: 0,
        attack_speed: 1.0,
        range: 12.0,
        is_two_handed: false,
        is_finesse: true,  // INT-based (we'll handle this separately)
        is_versatile: false,
        versatile_damage: None,
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

fn staff() -> WeaponStats {
    WeaponStats {
        weapon_type: WeaponType::Staff,
        damage_dice: (1, 6),
        damage_bonus: 0,
        attack_speed: 1.2,
        range: 10.0,  // Magic range, can also melee at 1.8
        is_two_handed: false,
        is_finesse: true,  // INT-based
        is_versatile: true,
        versatile_damage: Some((1, 8)),
        str_requirement: None,
        enchantments: Vec::new(),
    }
}

/// Apply enchantment to weapon stats
pub fn apply_enchantment(mut stats: WeaponStats, enchantment: Enchantment) -> WeaponStats {
    stats.enchantments.push(enchantment);

    match enchantment {
        Enchantment::Plus1 => {
            stats.damage_bonus += 1;
        }
        Enchantment::Plus2 => {
            stats.damage_bonus += 2;
        }
        Enchantment::Plus3 => {
            stats.damage_bonus += 3;
        }
        Enchantment::Swift => {
            stats.attack_speed *= 0.8;  // 20% faster
        }
        Enchantment::Reach => {
            stats.range += 0.5;  // +0.5 tiles
        }
        _ => {}  // Other enchantments are handled during damage calculation
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_weapons_defined() {
        // Ensure all weapon types have implementations
        for weapon_type in [
            WeaponType::Dagger,
            WeaponType::Shortsword,
            WeaponType::Longsword,
            WeaponType::Mace,
            WeaponType::Battleaxe,
            WeaponType::Warhammer,
            WeaponType::Greatsword,
            WeaponType::Greataxe,
            WeaponType::Maul,
            WeaponType::Quarterstaff,
            WeaponType::Shortbow,
            WeaponType::Longbow,
            WeaponType::LightCrossbow,
            WeaponType::HeavyCrossbow,
            WeaponType::Wand,
            WeaponType::Staff,
        ] {
            let stats = get_weapon_stats(weapon_type);
            assert!(stats.damage_dice.0 > 0);
            assert!(stats.damage_dice.1 > 0);
            assert!(stats.attack_speed > 0.0);
            assert!(stats.range > 0.0);
        }
    }

    #[test]
    fn test_weapon_values() {
        assert_eq!(get_weapon_value(WeaponType::Dagger), 2);
        assert_eq!(get_weapon_value(WeaponType::Longsword), 15);
        assert_eq!(get_weapon_value(WeaponType::Greatsword), 50);
    }

    #[test]
    fn test_versatile_weapons() {
        let longsword = get_weapon_stats(WeaponType::Longsword);
        assert!(longsword.is_versatile);
        assert_eq!(longsword.versatile_damage, Some((1, 10)));

        let dagger = get_weapon_stats(WeaponType::Dagger);
        assert!(!dagger.is_versatile);
    }

    #[test]
    fn test_two_handed_weapons() {
        let greatsword = get_weapon_stats(WeaponType::Greatsword);
        assert!(greatsword.is_two_handed);

        let longsword = get_weapon_stats(WeaponType::Longsword);
        assert!(!longsword.is_two_handed);
    }

    #[test]
    fn test_finesse_weapons() {
        let dagger = get_weapon_stats(WeaponType::Dagger);
        assert!(dagger.is_finesse);

        let warhammer = get_weapon_stats(WeaponType::Warhammer);
        assert!(!warhammer.is_finesse);
    }

    #[test]
    fn test_enchantment_application() {
        let longsword = get_weapon_stats(WeaponType::Longsword);
        let enhanced = apply_enchantment(longsword, Enchantment::Plus2);

        assert_eq!(enhanced.damage_bonus, 2);
        assert!(enhanced.enchantments.contains(&Enchantment::Plus2));
    }

    #[test]
    fn test_swift_enchantment() {
        let dagger = get_weapon_stats(WeaponType::Dagger);
        let original_speed = dagger.attack_speed;
        let swift_dagger = apply_enchantment(dagger, Enchantment::Swift);

        assert!(swift_dagger.attack_speed < original_speed);
        assert!((swift_dagger.attack_speed - original_speed * 0.8).abs() < 0.01);
    }
}
