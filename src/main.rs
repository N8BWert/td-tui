//!
//! Tower Defense Game Built with a tui using my game engine
//! 

use nate_engine::Engine;

use clap::Parser;

use td_tui::systems::tower_defense::{
    alive_enemies::{count_alive_enemies, remove_dead_entities, spawn_more_enemies},
    movement::{
        base_enemy_movement_system,
        second_enemy_movement_system,
        third_enemy_movement_system,
        fourth_enemy_movement_system,
        fifth_enemy_movement_system,
        final_enemy_movement_system,
    },
    tower::{
        base_tower_attack_ai,
        second_tower_attack_ai,
        third_tower_attack_ai,
        fourth_tower_attack_ai,
        fifth_tower_attack_ai,
        final_tower_attack_ai,
        upgrade_tower,
        downgrade_tower,
    },
};

use td_tui::{TOTAL_TOWERS, TowerTarget, TowerDefenseWorld, tui::TowerDefenseRenderer};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Workers in the threadpool
    #[arg(short, long, default_value_t = 3)]
    workers: usize,

    // Multiplier of the base game speed to play at
    #[arg(short, long, default_value_t = 0.5)]
    multiplier: f32,
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

    let renderer = TowerDefenseRenderer::new()?;

    let mut engine = Engine::new(
        60,
        args.workers,
        world,
        vec![
            (count_alive_enemies, 100_000),
            (remove_dead_entities, 100_000),
            (base_enemy_movement_system, (1_000_000.0 * args.multiplier) as u128),
            (second_enemy_movement_system, (1_000_000.0 * args.multiplier) as u128),
            (third_enemy_movement_system, (1_000_000.0 * args.multiplier) as u128),
            (fourth_enemy_movement_system, (1_000_000.0 * args.multiplier) as u128),
            (fifth_enemy_movement_system, (1_000_000.0 * args.multiplier) as u128),
            (final_enemy_movement_system, (1_000_000.0 * args.multiplier) as u128),
            (base_tower_attack_ai, (1_000_000.0 * args.multiplier) as u128),
            (second_tower_attack_ai, (500_000.0 * args.multiplier) as u128),
            (third_tower_attack_ai, (1_000_000.0 * args.multiplier) as u128),
            (fourth_tower_attack_ai, (500_000.0 * args.multiplier) as u128),
            (fifth_tower_attack_ai, (500_000.0 * args.multiplier) as u128),
            (final_tower_attack_ai, (250_000.0 * args.multiplier) as u128),
            (upgrade_tower, 50_000),
            (downgrade_tower, 50_000),
            (spawn_more_enemies, 1_000_000),
        ],
        Box::new(renderer)
    );

    engine.run();
    
    Ok(())
}