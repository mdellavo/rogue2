use hecs::Entity;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Species {
    Human,
    Elf,
    Dwarf,
    Halfling,
    HalfOrc,
    Gnome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterClass {
    Fighter,
    Rogue,
    Cleric,
    Wizard,
    Ranger,
    Barbarian,
}

#[derive(Debug, Clone)]
pub struct Character {
    pub species: Species,
    pub class: CharacterClass,
    pub level: u32,
    pub experience: u32,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub connection_id: u64,
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub sprite_id: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct ArmorClass {
    pub value: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct MovementSpeed {
    pub pixels_per_second: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct VisionRange {
    pub tiles: f32,
}

#[derive(Debug, Clone)]
pub struct Cooldowns {
    pub attack: Option<Instant>,
    pub ability: Option<Instant>,
}

impl Default for Cooldowns {
    fn default() -> Self {
        Self {
            attack: None,
            ability: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AttackSpeed {
    pub cooldown_ms: u64,
}

#[derive(Debug, Clone)]
pub enum AIBehavior {
    Idle,
    Hostile { target: Option<Entity> },
}

#[derive(Debug, Clone)]
pub struct AI {
    pub behavior: AIBehavior,
}

// Ability effects
#[derive(Debug, Clone)]
pub enum AbilityEffect {
    Heal { amount: i32 },
    Damage { amount: i32 },
    Buff { duration_ms: u64 },
}

#[derive(Debug, Clone)]
pub struct ActiveAbility {
    pub effect: AbilityEffect,
    pub expires_at: Instant,
}

// Monster types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterType {
    GiantRat,
    Goblin,
    Skeleton,
    Wolf,
    OrcWarrior,
    Zombie,
    GiantSpider,
    Hobgoblin,
    Ogre,
    Wight,
    Troll,
    YoungDragon,
    Lich,
    DemonLord,
}

#[derive(Debug, Clone)]
pub struct Monster {
    pub monster_type: MonsterType,
    pub level: u32,
    pub xp_reward: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct DetectionRange {
    pub tiles: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SpawnPoint {
    pub x: f32,
    pub y: f32,
    pub max_distance: f32, // How far from spawn point monster can roam
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Flee,
    Dead,
}

#[derive(Debug, Clone)]
pub struct AIController {
    pub state: AIState,
    pub target: Option<Entity>,
    pub last_seen_position: Option<(f32, f32)>,
    pub chase_timer: f32, // Time spent chasing before giving up
}

impl Default for AIController {
    fn default() -> Self {
        Self {
            state: AIState::Idle,
            target: None,
            last_seen_position: None,
            chase_timer: 0.0,
        }
    }
}

// Monster traits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterTrait {
    PackCreature,          // Calls allies when attacked
    UndeadResilience,      // Immune to poison, takes double from holy
    PackTactics,           // Bonus damage when ally nearby
    UndeadFortitude,       // Chance to survive at 1 HP
    WebAttack,             // Chance to slow on hit
    MartialAdvantage,      // Bonus damage with ally nearby
    PowerfulBuild,         // Knockback on hit
    UndeadCommander,       // Commands undead
    Regeneration,          // Heals over time
    KeenSmell,             // Increased detection
    DragonScales,          // Damage resistance
    FrightfulPresence,     // Enemies have attack penalty
    MagicResistance,       // Resist spells
    Phylactery,            // Respawn after death
    DemonResilience,       // Resist all but holy
    AuraOfTerror,          // Enemies deal less damage
}

#[derive(Debug, Clone)]
pub struct MonsterTraits {
    pub traits: Vec<MonsterTrait>,
}

// Loot table
#[derive(Debug, Clone)]
pub struct LootDrop {
    pub item_id: String,
    pub quantity: u32,
    pub chance: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub struct LootTable {
    pub drops: Vec<LootDrop>,
    pub guaranteed_copper_min: u32,
    pub guaranteed_copper_max: u32,
}

// Status effects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusEffect {
    Slowed,
    Poisoned,
    Stunned,
    Knockback,
    Burning,
    Frozen,
}

#[derive(Debug, Clone)]
pub struct StatusEffects {
    pub effects: Vec<(StatusEffect, Instant, u64)>, // (effect, started_at, duration_ms)
}

impl Default for StatusEffects {
    fn default() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
}

// Regeneration
#[derive(Debug, Clone, Copy)]
pub struct Regeneration {
    pub hp_per_second: i32,
    pub disabled_until: Option<Instant>,
}

// Boss marker
#[derive(Debug, Clone, Copy)]
pub struct Boss {
    pub respawn_cooldown_hours: u32,
}

// Items and Equipment
#[derive(Debug, Clone)]
pub struct Item {
    pub item_id: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Weapon,
    Armor,
    Shield,
    Helmet,
    Accessory,
    Consumable,
    Material,
    Quest,
    Currency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemRarity {
    Common,      // White
    Uncommon,    // Green, +1 bonus
    Rare,        // Blue, +2 bonus
    Epic,        // Purple, +3 bonus
    Legendary,   // Orange, +4 bonus
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Dagger,
    Shortsword,
    Longsword,
    Mace,
    Battleaxe,
    Warhammer,
    Greatsword,
    Greataxe,
    Maul,
    Quarterstaff,
    Shortbow,
    Longbow,
    LightCrossbow,
    HeavyCrossbow,
    Wand,
    Staff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmorType {
    ClothRobes,
    PaddedArmor,
    LeatherArmor,
    StuddedLeather,
    HideArmor,
    ChainShirt,
    ScaleMail,
    Breastplate,
    RingMail,
    ChainMail,
    SplintArmor,
    PlateArmor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShieldType {
    Buckler,
    Standard,
    Tower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelmetType {
    ClothCap,
    LeatherCap,
    ChainCoif,
    SteelHelm,
    GreatHelm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessoryType {
    RingOfProtection,
    AmuletOfHealth,
    RingOfStrength,
    RingOfDexterity,
    RingOfConstitution,
    CloakOfResistance,
    BootsOfSpeed,
    GlovesOfOgrePower,
    HeadbandOfIntellect,
    PeriaptOfWisdom,
    RingOfRegeneration,
    AmuletOfLifeDrain,
    RingOfFireResistance,
    RingOfSpellStoring,
    BootsOfElvenkind,
    CloakOfInvisibility,
    BeltOfGiantStrength,
    RingOfFeatherFalling,
    AmuletOfTheDevout,
    RingOfEvasion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enchantment {
    // Weapon enchantments
    Plus1,
    Plus2,
    Plus3,
    Flaming,
    Frost,
    Shock,
    Vampiric,
    Keen,
    Holy,
    Venom,
    Mighty,
    Swift,
    Reach,
    // Armor enchantments
    FireResistance,
    ColdResistance,
    LightningResistance,
    Fortification,
    Shadow,
    Speed,
    Radiant,
    Thorns,
    Featherweight,
    Absorbing,
}

#[derive(Debug, Clone)]
pub struct WeaponStats {
    pub weapon_type: WeaponType,
    pub damage_dice: (u32, u32),  // (num_dice, die_size) e.g., (1, 8) for 1d8
    pub damage_bonus: i32,
    pub attack_speed: f32,  // Cooldown in seconds
    pub range: f32,  // Range in tiles
    pub is_two_handed: bool,
    pub is_finesse: bool,  // Use DEX instead of STR
    pub is_versatile: bool,
    pub versatile_damage: Option<(u32, u32)>,  // Damage when wielded two-handed
    pub str_requirement: Option<i32>,
    pub enchantments: Vec<Enchantment>,
}

#[derive(Debug, Clone)]
pub struct ArmorStats {
    pub armor_type: ArmorType,
    pub base_ac: i32,
    pub dex_bonus_cap: Option<i32>,  // None = full DEX, Some(2) = max +2 DEX
    pub movement_penalty: f32,  // Percentage (e.g., 0.1 = -10% speed)
    pub weight: u32,
    pub str_requirement: Option<i32>,
    pub stealth_disadvantage: bool,
    pub enchantments: Vec<Enchantment>,
}

#[derive(Debug, Clone)]
pub struct ShieldStats {
    pub shield_type: ShieldType,
    pub ac_bonus: i32,
    pub weight: u32,
    pub str_requirement: Option<i32>,
    pub movement_penalty: f32,
    pub enchantments: Vec<Enchantment>,
}

#[derive(Debug, Clone)]
pub struct HelmetStats {
    pub helmet_type: HelmetType,
    pub ac_bonus: i32,
    pub str_requirement: Option<i32>,
    pub vision_penalty: f32,  // Percentage reduction in vision range
    pub enchantments: Vec<Enchantment>,
}

#[derive(Debug, Clone)]
pub struct AccessoryStats {
    pub accessory_type: AccessoryType,
    pub effects: AccessoryEffects,
}

#[derive(Debug, Clone, Default)]
pub struct AccessoryEffects {
    pub ac_bonus: i32,
    pub hp_bonus: i32,
    pub str_bonus: i32,
    pub dex_bonus: i32,
    pub con_bonus: i32,
    pub int_bonus: i32,
    pub wis_bonus: i32,
    pub cha_bonus: i32,
    pub movement_speed_bonus: f32,  // Percentage
    pub hp_regen_per_5s: i32,
    pub lifesteal_percent: f32,
    pub fire_resistance: f32,
    pub evasion_chance: f32,
}

#[derive(Debug, Clone)]
pub struct Equipment {
    pub main_hand: Option<EquippedItem>,
    pub off_hand: Option<EquippedItem>,
    pub armor: Option<EquippedItem>,
    pub helmet: Option<EquippedItem>,
    pub accessory1: Option<EquippedItem>,
    pub accessory2: Option<EquippedItem>,
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            main_hand: None,
            off_hand: None,
            armor: None,
            helmet: None,
            accessory1: None,
            accessory2: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EquippedItem {
    pub item_id: String,
    pub rarity: ItemRarity,
    pub stats: ItemStats,
}

#[derive(Debug, Clone)]
pub enum ItemStats {
    Weapon(WeaponStats),
    Armor(ArmorStats),
    Shield(ShieldStats),
    Helmet(HelmetStats),
    Accessory(AccessoryStats),
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    pub capacity: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            capacity: 20,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub item_id: String,
    pub quantity: u32,
    pub rarity: ItemRarity,
    pub identified: bool,
    pub stats: Option<ItemStats>,
}
