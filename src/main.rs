//!
//! Tower Defense Game Built with a tui using my game engine
//! 

use nate_engine::Engine;

use clap::Parser;

mod tui;

pub mod world;
pub use world::tower_defense_world::TowerDefenseWorld;

pub mod systems;

/// The total number of positions for enemies to move through
pub const TOTAL_POSITIONS: u32 = 100;
/// The total number of towers
pub const TOTAL_TOWERS: u32 = 10;

/// The Type of Tower
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TowerType {
    // broken towers deal 0 units of damager per 1 second
    Broken,
    // the base tower deals 1 unit of damage per 1 second
    Base,
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

        // Add a set of towers
        let tower_ids = world.add_entities(TOTAL_TOWERS as usize);
        world.set_tower_types(&tower_ids, tower_ids.iter().map(|_v| TowerType::Broken).collect());
        world.set_tower_type(tower_ids[0], TowerType::Base);

        // Add Enemies
        let enemy_ids = world.add_entities(10 as usize);
        world.set_enemy_types(&enemy_ids, enemy_ids.iter().map(|_v| EnemyType::Base).collect());
        world.set_sprites(&enemy_ids, vec![String::from("O"); 10]);
        world.set_healths(&enemy_ids, enemy_ids.iter().map(|_v| 1).collect());
        world.set_enemy_positions(&enemy_ids, enemy_ids.iter().enumerate().map(|v| TOTAL_POSITIONS - v.0 as u32).collect());

        // Initialize Singular Components
        world.set_base_health(100);
        world.set_base_damage(0);
        world.set_removal_entities(Vec::new());
        world.set_alive_enemies(0);
    }

    let renderer = tui::TowerDefenseRenderer::new()?;

    let mut engine = Engine::new(
        30,
        args.workers,
        world,
        vec![

        ],
        Box::new(renderer)
    );

    engine.run();
    
    Ok(())
}