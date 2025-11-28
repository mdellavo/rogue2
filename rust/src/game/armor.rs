use crate::ecs::components::*;

/// Get armor stats by armor type
pub fn get_armor_stats(armor_type: ArmorType) -> ArmorStats {
    match armor_type {
        ArmorType::ClothRobes => cloth_robes(),
        ArmorType::PaddedArmor => padded_armor(),
        ArmorType::LeatherArmor => leather_armor(),
        ArmorType::StuddedLeather => studded_leather(),
        ArmorType::HideArmor => hide_armor(),
        ArmorType::ChainShirt => chain_shirt(),
        ArmorType::ScaleMail => scale_mail(),
        ArmorType::Breastplate => breastplate(),
        ArmorType::RingMail => ring_mail(),
        ArmorType::ChainMail => chain_mail(),
        ArmorType::SplintArmor => splint_armor(),
        ArmorType::PlateArmor => plate_armor(),
    }
}

/// Get armor value in gold
pub fn get_armor_value(armor_type: ArmorType) -> u32 {
    match armor_type {
        ArmorType::ClothRobes => 5,
        ArmorType::PaddedArmor => 10,
        ArmorType::LeatherArmor => 15,
        ArmorType::StuddedLeather => 45,
        ArmorType::HideArmor => 15,
        ArmorType::ChainShirt => 50,
        ArmorType::ScaleMail => 75,
        ArmorType::Breastplate => 400,
        ArmorType::RingMail => 30,
        ArmorType::ChainMail => 75,
        ArmorType::SplintArmor => 200,
        ArmorType::PlateArmor => 1500,
    }
}

// Light armor

fn cloth_robes() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::ClothRobes,
        base_ac: 11,
        dex_bonus_cap: None,  // Full DEX bonus
        movement_penalty: 0.0,
        weight: 4,
        str_requirement: None,
        stealth_disadvantage: false,
        enchantments: Vec::new(),
    }
}

fn padded_armor() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::PaddedArmor,
        base_ac: 11,
        dex_bonus_cap: None,
        movement_penalty: 0.0,
        weight: 8,
        str_requirement: None,
        stealth_disadvantage: true,
        enchantments: Vec::new(),
    }
}

fn leather_armor() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::LeatherArmor,
        base_ac: 12,
        dex_bonus_cap: None,
        movement_penalty: 0.0,
        weight: 10,
        str_requirement: None,
        stealth_disadvantage: false,
        enchantments: Vec::new(),
    }
}

fn studded_leather() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::StuddedLeather,
        base_ac: 13,
        dex_bonus_cap: None,
        movement_penalty: 0.0,
        weight: 13,
        str_requirement: None,
        stealth_disadvantage: false,
        enchantments: Vec::new(),
    }
}

// Medium armor

fn hide_armor() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::HideArmor,
        base_ac: 13,
        dex_bonus_cap: Some(2),
        movement_penalty: 0.05,  // -5% speed
        weight: 12,
        str_requirement: None,
        stealth_disadvantage: false,
        enchantments: Vec::new(),
    }
}

fn chain_shirt() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::ChainShirt,
        base_ac: 14,
        dex_bonus_cap: Some(2),
        movement_penalty: 0.05,
        weight: 20,
        str_requirement: None,
        stealth_disadvantage: false,
        enchantments: Vec::new(),
    }
}

fn scale_mail() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::ScaleMail,
        base_ac: 15,
        dex_bonus_cap: Some(2),
        movement_penalty: 0.10,  // -10% speed
        weight: 45,
        str_requirement: Some(12),
        stealth_disadvantage: true,
        enchantments: Vec::new(),
    }
}

fn breastplate() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::Breastplate,
        base_ac: 15,
        dex_bonus_cap: Some(2),
        movement_penalty: 0.05,
        weight: 20,
        str_requirement: None,
        stealth_disadvantage: false,
        enchantments: Vec::new(),
    }
}

// Heavy armor

fn ring_mail() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::RingMail,
        base_ac: 14,
        dex_bonus_cap: Some(0),  // No DEX bonus
        movement_penalty: 0.15,  // -15% speed
        weight: 40,
        str_requirement: Some(13),
        stealth_disadvantage: true,
        enchantments: Vec::new(),
    }
}

fn chain_mail() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::ChainMail,
        base_ac: 16,
        dex_bonus_cap: Some(0),
        movement_penalty: 0.15,
        weight: 55,
        str_requirement: Some(13),
        stealth_disadvantage: true,
        enchantments: Vec::new(),
    }
}

fn splint_armor() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::SplintArmor,
        base_ac: 17,
        dex_bonus_cap: Some(0),
        movement_penalty: 0.20,  // -20% speed
        weight: 60,
        str_requirement: Some(15),
        stealth_disadvantage: true,
        enchantments: Vec::new(),
    }
}

fn plate_armor() -> ArmorStats {
    ArmorStats {
        armor_type: ArmorType::PlateArmor,
        base_ac: 18,
        dex_bonus_cap: Some(0),
        movement_penalty: 0.20,
        weight: 65,
        str_requirement: Some(15),
        stealth_disadvantage: true,
        enchantments: Vec::new(),
    }
}

/// Apply enchantment to armor stats
pub fn apply_enchantment(mut stats: ArmorStats, enchantment: Enchantment) -> ArmorStats {
    stats.enchantments.push(enchantment);

    match enchantment {
        Enchantment::Plus1 => {
            stats.base_ac += 1;
        }
        Enchantment::Plus2 => {
            stats.base_ac += 2;
        }
        Enchantment::Plus3 => {
            stats.base_ac += 3;
        }
        Enchantment::Speed => {
            stats.movement_penalty = 0.0;  // Remove penalty
        }
        Enchantment::Shadow => {
            stats.stealth_disadvantage = false;  // Remove disadvantage
        }
        Enchantment::Featherweight => {
            stats.weight = 0;
            stats.str_requirement = None;
        }
        _ => {}  // Other enchantments handled elsewhere
    }

    stats
}

/// Get shield stats by shield type
pub fn get_shield_stats(shield_type: ShieldType) -> ShieldStats {
    match shield_type {
        ShieldType::Buckler => buckler(),
        ShieldType::Standard => standard_shield(),
        ShieldType::Tower => tower_shield(),
    }
}

/// Get shield value in gold
pub fn get_shield_value(shield_type: ShieldType) -> u32 {
    match shield_type {
        ShieldType::Buckler => 5,
        ShieldType::Standard => 10,
        ShieldType::Tower => 50,
    }
}

fn buckler() -> ShieldStats {
    ShieldStats {
        shield_type: ShieldType::Buckler,
        ac_bonus: 1,
        weight: 3,
        str_requirement: None,
        movement_penalty: 0.0,
        enchantments: Vec::new(),
    }
}

fn standard_shield() -> ShieldStats {
    ShieldStats {
        shield_type: ShieldType::Standard,
        ac_bonus: 2,
        weight: 6,
        str_requirement: None,
        movement_penalty: 0.0,
        enchantments: Vec::new(),
    }
}

fn tower_shield() -> ShieldStats {
    ShieldStats {
        shield_type: ShieldType::Tower,
        ac_bonus: 3,
        weight: 15,
        str_requirement: Some(15),
        movement_penalty: 0.10,  // -10% speed
        enchantments: Vec::new(),
    }
}

/// Get helmet stats by helmet type
pub fn get_helmet_stats(helmet_type: HelmetType) -> HelmetStats {
    match helmet_type {
        HelmetType::ClothCap => cloth_cap(),
        HelmetType::LeatherCap => leather_cap(),
        HelmetType::ChainCoif => chain_coif(),
        HelmetType::SteelHelm => steel_helm(),
        HelmetType::GreatHelm => great_helm(),
    }
}

/// Get helmet value in gold
pub fn get_helmet_value(helmet_type: HelmetType) -> u32 {
    match helmet_type {
        HelmetType::ClothCap => 1,
        HelmetType::LeatherCap => 5,
        HelmetType::ChainCoif => 25,
        HelmetType::SteelHelm => 75,
        HelmetType::GreatHelm => 150,
    }
}

fn cloth_cap() -> HelmetStats {
    HelmetStats {
        helmet_type: HelmetType::ClothCap,
        ac_bonus: 0,
        str_requirement: None,
        vision_penalty: 0.0,
        enchantments: Vec::new(),
    }
}

fn leather_cap() -> HelmetStats {
    HelmetStats {
        helmet_type: HelmetType::LeatherCap,
        ac_bonus: 1,
        str_requirement: None,
        vision_penalty: 0.0,
        enchantments: Vec::new(),
    }
}

fn chain_coif() -> HelmetStats {
    HelmetStats {
        helmet_type: HelmetType::ChainCoif,
        ac_bonus: 1,
        str_requirement: None,
        vision_penalty: 0.0,
        enchantments: Vec::new(),
    }
}

fn steel_helm() -> HelmetStats {
    HelmetStats {
        helmet_type: HelmetType::SteelHelm,
        ac_bonus: 2,
        str_requirement: None,
        vision_penalty: 0.05,  // -5% vision range
        enchantments: Vec::new(),
    }
}

fn great_helm() -> HelmetStats {
    HelmetStats {
        helmet_type: HelmetType::GreatHelm,
        ac_bonus: 3,
        str_requirement: Some(13),
        vision_penalty: 0.10,  // -10% vision range
        enchantments: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_armor_defined() {
        for armor_type in [
            ArmorType::ClothRobes,
            ArmorType::PaddedArmor,
            ArmorType::LeatherArmor,
            ArmorType::StuddedLeather,
            ArmorType::HideArmor,
            ArmorType::ChainShirt,
            ArmorType::ScaleMail,
            ArmorType::Breastplate,
            ArmorType::RingMail,
            ArmorType::ChainMail,
            ArmorType::SplintArmor,
            ArmorType::PlateArmor,
        ] {
            let stats = get_armor_stats(armor_type);
            assert!(stats.base_ac > 0);
        }
    }

    #[test]
    fn test_light_armor_no_dex_cap() {
        let leather = get_armor_stats(ArmorType::LeatherArmor);
        assert!(leather.dex_bonus_cap.is_none());
        assert_eq!(leather.movement_penalty, 0.0);
    }

    #[test]
    fn test_medium_armor_dex_cap() {
        let chain_shirt = get_armor_stats(ArmorType::ChainShirt);
        assert_eq!(chain_shirt.dex_bonus_cap, Some(2));
    }

    #[test]
    fn test_heavy_armor_no_dex() {
        let plate = get_armor_stats(ArmorType::PlateArmor);
        assert_eq!(plate.dex_bonus_cap, Some(0));
        assert!(plate.movement_penalty > 0.0);
    }

    #[test]
    fn test_enchantment_application() {
        let leather = get_armor_stats(ArmorType::LeatherArmor);
        let enhanced = apply_enchantment(leather, Enchantment::Plus2);
        assert_eq!(enhanced.base_ac, 14);  // 12 + 2
    }

    #[test]
    fn test_all_shields_defined() {
        for shield_type in [ShieldType::Buckler, ShieldType::Standard, ShieldType::Tower] {
            let stats = get_shield_stats(shield_type);
            assert!(stats.ac_bonus > 0);
        }
    }

    #[test]
    fn test_all_helmets_defined() {
        for helmet_type in [
            HelmetType::ClothCap,
            HelmetType::LeatherCap,
            HelmetType::ChainCoif,
            HelmetType::SteelHelm,
            HelmetType::GreatHelm,
        ] {
            let stats = get_helmet_stats(helmet_type);
            assert!(stats.ac_bonus >= 0);
        }
    }
}
