use crate::ecs::components::*;

/// Calculate final stats including all equipment bonuses
pub struct EquipmentCalculator;

impl EquipmentCalculator {
    /// Calculate final AC from base, armor, shield, helmet, and accessories
    pub fn calculate_ac(
        base_ac: i32,
        dex_modifier: i32,
        equipment: &Equipment,
    ) -> i32 {
        let mut total_ac = base_ac;

        // Armor contribution
        if let Some(armor_item) = &equipment.armor {
            if let ItemStats::Armor(armor) = &armor_item.stats {
                total_ac = armor.base_ac;

                // Apply DEX bonus based on armor type
                let dex_contribution = match armor.dex_bonus_cap {
                    None => dex_modifier,  // Full DEX (light armor)
                    Some(cap) => dex_modifier.min(cap),  // Capped DEX (medium/heavy)
                };
                total_ac += dex_contribution;
            }
        } else {
            // No armor: use base AC + full DEX
            total_ac += dex_modifier;
        }

        // Shield bonus
        if let Some(shield_item) = &equipment.off_hand {
            if let ItemStats::Shield(shield) = &shield_item.stats {
                total_ac += shield.ac_bonus;
            }
        }

        // Helmet bonus
        if let Some(helmet_item) = &equipment.helmet {
            if let ItemStats::Helmet(helmet) = &helmet_item.stats {
                total_ac += helmet.ac_bonus;
            }
        }

        // Accessory bonuses
        for accessory_opt in [&equipment.accessory1, &equipment.accessory2] {
            if let Some(accessory) = accessory_opt {
                if let ItemStats::Accessory(acc) = &accessory.stats {
                    total_ac += acc.effects.ac_bonus;
                }
            }
        }

        total_ac
    }

    /// Calculate modified stats from base stats + equipment bonuses
    pub fn calculate_stats(base_stats: &Stats, equipment: &Equipment) -> Stats {
        let mut final_stats = *base_stats;

        // Apply accessory bonuses
        for accessory_opt in [&equipment.accessory1, &equipment.accessory2] {
            if let Some(accessory) = accessory_opt {
                if let ItemStats::Accessory(acc) = &accessory.stats {
                    final_stats.str += acc.effects.str_bonus;
                    final_stats.dex += acc.effects.dex_bonus;
                    final_stats.con += acc.effects.con_bonus;
                    final_stats.int += acc.effects.int_bonus;
                    final_stats.wis += acc.effects.wis_bonus;
                    final_stats.cha += acc.effects.cha_bonus;
                }
            }
        }

        // Cap stats at 20 (D&D limit)
        final_stats.str = final_stats.str.min(20);
        final_stats.dex = final_stats.dex.min(20);
        final_stats.con = final_stats.con.min(20);
        final_stats.int = final_stats.int.min(20);
        final_stats.wis = final_stats.wis.min(20);
        final_stats.cha = final_stats.cha.min(20);

        final_stats
    }

    /// Calculate modified max HP from base + equipment bonuses
    pub fn calculate_max_hp(base_hp: i32, equipment: &Equipment) -> i32 {
        let mut total_hp = base_hp;

        for accessory_opt in [&equipment.accessory1, &equipment.accessory2] {
            if let Some(accessory) = accessory_opt {
                if let ItemStats::Accessory(acc) = &accessory.stats {
                    total_hp += acc.effects.hp_bonus;
                }
            }
        }

        total_hp.max(1)  // Minimum 1 HP
    }

    /// Calculate final movement speed with armor penalties and accessory bonuses
    pub fn calculate_movement_speed(
        base_speed: f32,
        equipment: &Equipment,
    ) -> f32 {
        let mut final_speed = base_speed;

        // Apply armor penalty
        if let Some(armor_item) = &equipment.armor {
            if let ItemStats::Armor(armor) = &armor_item.stats {
                // Check if armor has Speed enchantment
                if !armor.enchantments.contains(&Enchantment::Speed) {
                    final_speed *= 1.0 - armor.movement_penalty;
                }
            }
        }

        // Apply shield penalty
        if let Some(shield_item) = &equipment.off_hand {
            if let ItemStats::Shield(shield) = &shield_item.stats {
                final_speed *= 1.0 - shield.movement_penalty;
            }
        }

        // Apply accessory bonuses
        for accessory_opt in [&equipment.accessory1, &equipment.accessory2] {
            if let Some(accessory) = accessory_opt {
                if let ItemStats::Accessory(acc) = &accessory.stats {
                    final_speed *= 1.0 + acc.effects.movement_speed_bonus;
                }
            }
        }

        final_speed.max(50.0)  // Minimum 50 pixels/second
    }

    /// Get weapon attack speed with enchantments
    pub fn get_attack_speed(equipment: &Equipment) -> Option<f32> {
        if let Some(weapon_item) = &equipment.main_hand {
            if let ItemStats::Weapon(weapon) = &weapon_item.stats {
                return Some(weapon.attack_speed);
            }
        }
        None
    }

    /// Get weapon damage dice
    pub fn get_weapon_damage(equipment: &Equipment, wielding_two_handed: bool) -> Option<(u32, u32, i32)> {
        if let Some(weapon_item) = &equipment.main_hand {
            if let ItemStats::Weapon(weapon) = &weapon_item.stats {
                let dice = if wielding_two_handed && weapon.is_versatile {
                    weapon.versatile_damage.unwrap_or(weapon.damage_dice)
                } else {
                    weapon.damage_dice
                };
                return Some((dice.0, dice.1, weapon.damage_bonus));
            }
        }
        None
    }

    /// Check if weapon is finesse (use DEX instead of STR)
    pub fn is_weapon_finesse(equipment: &Equipment) -> bool {
        if let Some(weapon_item) = &equipment.main_hand {
            if let ItemStats::Weapon(weapon) = &weapon_item.stats {
                return weapon.is_finesse;
            }
        }
        false
    }

    /// Get weapon range
    pub fn get_weapon_range(equipment: &Equipment) -> f32 {
        if let Some(weapon_item) = &equipment.main_hand {
            if let ItemStats::Weapon(weapon) = &weapon_item.stats {
                return weapon.range;
            }
        }
        1.5  // Default melee range
    }

    /// Calculate lifesteal percentage from accessories
    pub fn get_lifesteal_percent(equipment: &Equipment) -> f32 {
        let mut total = 0.0;

        for accessory_opt in [&equipment.accessory1, &equipment.accessory2] {
            if let Some(accessory) = accessory_opt {
                if let ItemStats::Accessory(acc) = &accessory.stats {
                    total += acc.effects.lifesteal_percent;
                }
            }
        }

        // Also check for Vampiric weapon enchantment
        if let Some(weapon_item) = &equipment.main_hand {
            if let ItemStats::Weapon(weapon) = &weapon_item.stats {
                if weapon.enchantments.contains(&Enchantment::Vampiric) {
                    total += 0.50;  // 50% lifesteal
                }
            }
        }

        total
    }

    /// Calculate evasion chance from accessories
    pub fn get_evasion_chance(equipment: &Equipment) -> f32 {
        let mut total = 0.0;

        for accessory_opt in [&equipment.accessory1, &equipment.accessory2] {
            if let Some(accessory) = accessory_opt {
                if let ItemStats::Accessory(acc) = &accessory.stats {
                    total += acc.effects.evasion_chance;
                }
            }
        }

        total.min(0.75)  // Cap at 75% evasion
    }

    /// Get HP regeneration rate from accessories
    pub fn get_hp_regen_per_5s(equipment: &Equipment) -> i32 {
        let mut total = 0;

        for accessory_opt in [&equipment.accessory1, &equipment.accessory2] {
            if let Some(accessory) = accessory_opt {
                if let ItemStats::Accessory(acc) = &accessory.stats {
                    total += acc.effects.hp_regen_per_5s;
                }
            }
        }

        total
    }

    /// Check if equipment meets STR requirements
    pub fn meets_requirements(stats: &Stats, equipment: &Equipment) -> bool {
        // Check weapon STR requirement
        if let Some(weapon_item) = &equipment.main_hand {
            if let ItemStats::Weapon(weapon) = &weapon_item.stats {
                if let Some(req) = weapon.str_requirement {
                    if stats.str < req {
                        return false;
                    }
                }
            }
        }

        // Check armor STR requirement
        if let Some(armor_item) = &equipment.armor {
            if let ItemStats::Armor(armor) = &armor_item.stats {
                if let Some(req) = armor.str_requirement {
                    if stats.str < req {
                        return false;
                    }
                }
            }
        }

        // Check shield STR requirement
        if let Some(shield_item) = &equipment.off_hand {
            if let ItemStats::Shield(shield) = &shield_item.stats {
                if let Some(req) = shield.str_requirement {
                    if stats.str < req {
                        return false;
                    }
                }
            }
        }

        // Check helmet STR requirement
        if let Some(helmet_item) = &equipment.helmet {
            if let ItemStats::Helmet(helmet) = &helmet_item.stats {
                if let Some(req) = helmet.str_requirement {
                    if stats.str < req {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::weapons;
    use crate::game::armor;
    use crate::game::accessories;

    #[test]
    fn test_ac_calculation_no_armor() {
        let equipment = Equipment::default();
        let dex_mod = 2;
        let ac = EquipmentCalculator::calculate_ac(10, dex_mod, &equipment);
        assert_eq!(ac, 12);  // 10 + 2
    }

    #[test]
    fn test_ac_calculation_with_armor() {
        let mut equipment = Equipment::default();
        let leather = armor::get_armor_stats(ArmorType::LeatherArmor);
        equipment.armor = Some(EquippedItem {
            item_id: "leather_armor".to_string(),
            rarity: ItemRarity::Common,
            stats: ItemStats::Armor(leather),
        });

        let dex_mod = 3;
        let ac = EquipmentCalculator::calculate_ac(10, dex_mod, &equipment);
        assert_eq!(ac, 15);  // 12 (leather base) + 3 (full DEX)
    }

    #[test]
    fn test_ac_calculation_heavy_armor() {
        let mut equipment = Equipment::default();
        let plate = armor::get_armor_stats(ArmorType::PlateArmor);
        equipment.armor = Some(EquippedItem {
            item_id: "plate_armor".to_string(),
            rarity: ItemRarity::Common,
            stats: ItemStats::Armor(plate),
        });

        let dex_mod = 3;
        let ac = EquipmentCalculator::calculate_ac(10, dex_mod, &equipment);
        assert_eq!(ac, 18);  // 18 (plate base) + 0 (no DEX)
    }

    #[test]
    fn test_stat_bonuses() {
        let mut equipment = Equipment::default();
        let ring = accessories::get_accessory_stats(AccessoryType::RingOfStrength);
        equipment.accessory1 = Some(EquippedItem {
            item_id: "ring_of_strength".to_string(),
            rarity: ItemRarity::Common,
            stats: ItemStats::Accessory(ring),
        });

        let base_stats = Stats {
            str: 10,
            dex: 10,
            con: 10,
            int: 10,
            wis: 10,
            cha: 10,
        };

        let final_stats = EquipmentCalculator::calculate_stats(&base_stats, &equipment);
        assert_eq!(final_stats.str, 11);  // 10 + 1 from ring
    }

    #[test]
    fn test_movement_speed_penalty() {
        let mut equipment = Equipment::default();
        let plate = armor::get_armor_stats(ArmorType::PlateArmor);
        equipment.armor = Some(EquippedItem {
            item_id: "plate_armor".to_string(),
            rarity: ItemRarity::Common,
            stats: ItemStats::Armor(plate),
        });

        let base_speed = 200.0;
        let final_speed = EquipmentCalculator::calculate_movement_speed(base_speed, &equipment);
        assert_eq!(final_speed, 160.0);  // 200 * (1 - 0.20)
    }

    #[test]
    fn test_movement_speed_bonus() {
        let mut equipment = Equipment::default();
        let boots = accessories::get_accessory_stats(AccessoryType::BootsOfSpeed);
        equipment.accessory1 = Some(EquippedItem {
            item_id: "boots_of_speed".to_string(),
            rarity: ItemRarity::Common,
            stats: ItemStats::Accessory(boots),
        });

        let base_speed = 200.0;
        let final_speed = EquipmentCalculator::calculate_movement_speed(base_speed, &equipment);
        assert_eq!(final_speed, 220.0);  // 200 * 1.10
    }

    #[test]
    fn test_weapon_damage() {
        let mut equipment = Equipment::default();
        let longsword = weapons::get_weapon_stats(WeaponType::Longsword);
        equipment.main_hand = Some(EquippedItem {
            item_id: "longsword".to_string(),
            rarity: ItemRarity::Common,
            stats: ItemStats::Weapon(longsword),
        });

        let damage = EquipmentCalculator::get_weapon_damage(&equipment, false);
        assert_eq!(damage, Some((1, 8, 0)));  // 1d8+0

        // Two-handed versatile
        let damage_2h = EquipmentCalculator::get_weapon_damage(&equipment, true);
        assert_eq!(damage_2h, Some((1, 10, 0)));  // 1d10+0
    }

    #[test]
    fn test_lifesteal() {
        let mut equipment = Equipment::default();
        let amulet = accessories::get_accessory_stats(AccessoryType::AmuletOfLifeDrain);
        equipment.accessory1 = Some(EquippedItem {
            item_id: "amulet_of_life_drain".to_string(),
            rarity: ItemRarity::Common,
            stats: ItemStats::Accessory(amulet),
        });

        let lifesteal = EquipmentCalculator::get_lifesteal_percent(&equipment);
        assert!((lifesteal - 0.20).abs() < 0.01);
    }
}
