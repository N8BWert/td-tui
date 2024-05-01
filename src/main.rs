//!
//! Tower Defense Game Built with a tui using my game engine
//! 

use nate_engine::Engine;

use clap::Parser;

mod tui;

pub mod world;
pub use world::tower_defense_world::TowerDefenseWorld;

pub mod systems;
use systems::tower_defense::{
    alive_enemies::{count_alive_enemies, remove_dead_entities, spawn_more_enemies},
    movement::{base_enemy_movement_system, second_enemy_movement_system},
    tower::{base_tower_attack_ai, upgrade_tower, downgrade_tower},
};

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
}

impl TowerType {
    /// The cost to upgrade a tower
    pub fn upgrade_price(&self) -> u32 {
        match self {
            TowerType::Broken => 10,
            TowerType::Base => 20,
        }
    }

    /// The number of points gained from selling a tower
    pub fn sell_price(&self) -> u32 {
        match self {
            TowerType::Broken => 0,
            TowerType::Base => 5,
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Workers in the threadpool
    #[arg(short, long, default_value_t = 3)]
    workers: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let world = TowerDefenseWorld::new();
    {
        let mut world = world.write().unwrap();

        // Initialize Singular Components
        world.initialize_singular_components(100);

        // Add a real tower
        world.add_base_tower(TowerTarget::First, 0);

        // Add Broken Towers
        world.add_broken_towers((TOTAL_TOWERS - 1) as usize);

        // Add some enemies
        world.add_base_enemies(vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90]);
    }

    let renderer = tui::TowerDefenseRenderer::new()?;

    let mut engine = Engine::new(
        60,
        args.workers,
        world,
        vec![
            (count_alive_enemies, 100_000),
            (remove_dead_entities, 100_000),
            (base_enemy_movement_system, 1_000_000),
            (second_enemy_movement_system, 1_000_000),
            (base_tower_attack_ai, 1_000_000),
            (upgrade_tower, 50_000),
            (downgrade_tower, 50_000),
            (spawn_more_enemies, 1_000_000),
        ],
        Box::new(renderer)
    );

    engine.run();
    
    Ok(())
}