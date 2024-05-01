//!
//! Tower Defense Game Built with a tui using nate's game engine
//! 

pub mod tui;

pub mod world;
pub use world::tower_defense_world::TowerDefenseWorld;

pub mod systems;

/// The total number of positions for enemies to move through
pub const TOTAL_POSITIONS: u32 = 100;
/// The total number of towers
pub const TOTAL_TOWERS: u32 = 10;
/// The separation between towers
pub const TOWER_SEPARATION: u32 = TOTAL_POSITIONS / TOTAL_TOWERS;

/// The Type of Tower
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TowerType {
    // broken towers deal 0 units of damager per 1 second
    Broken,
    // the base tower deals 1 unit of damage per 1 second
    Base,
    // The second tower deals 1 unit of damage per 0.5 seconds
    Second,
    // The third tower deals 3 units of damage per 1 second
    Third,
    // The forth tower deals 2 units of damage per 0.5 seconds
    Fourth,
    // The fifth tower deals 3 units of damage per 0.5 seconds
    Fifth,
    // The final tower deals 3 units of damage per 0.25 seconds
    Final,
}

impl TowerType {
    /// The cost to upgrade a tower
    pub fn upgrade_price(&self) -> u32 {
        match self {
            TowerType::Broken => 10,
            TowerType::Base => 20,
            TowerType::Second => 30,
            TowerType::Third => 40,
            TowerType::Fourth => 50,
            TowerType::Fifth => 60,
            TowerType::Final => 0,
        }
    }

    /// The number of points gained from selling a tower
    pub fn sell_price(&self) -> u32 {
        match self {
            TowerType::Broken => 0,
            _ => 5,
        }
    }
}

/// Target Enemy for a given tower
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TowerTarget {
    // The first enemy
    First,
    // The second enemy
    Second,
    // The last enemy
    Last,
}

/// The Type of Enemy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    // the base enemy moves 1 unit per second and has a base health of 1
    Base,
    // The second class of enemy that moves 2 units per second and has a base health of 2
    Second,
}