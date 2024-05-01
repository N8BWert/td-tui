//!
//! Systems dealing with alive enemies
//! 

#![allow(unused)]

use std::sync::{Arc, RwLock};

use nate_engine::system;

use crate::{EnemyType, TowerDefenseWorld, TOTAL_POSITIONS};

// (0, 10) -> (0, 20) -> (10, 0) -> (10, 10) -> (10, 20) -> (20, 0)

#[system(
    world=TowerDefenseWorld,
    read=[health, enemy_type],
    _write=[alive_enemies=0]
)]
fn count_alive_enemies() {
    if *health > 0 {
        *alive_enemies += 1;
    }
}

pub fn remove_dead_entities(world: Arc<RwLock<TowerDefenseWorld>>) {
    let mut remove_entities = Vec::new();
    {
        let read_world = world.read().unwrap();
        
        // Check for 0 health entities
        let health = read_world.health.read().unwrap();
        let enemy_type = read_world.enemy_type.read().unwrap();
        let mut points_ref = read_world.points.write().unwrap();
        let mut points = points_ref.as_mut().unwrap();
        for ((entity_id, health), enemy_type) in health.iter().enumerate().zip(enemy_type.iter()).rev() {
            if let Some(health) = health.as_ref() {
                if *health == 0 {
                    remove_entities.push(entity_id);
                    match enemy_type.unwrap() {
                        EnemyType::Base => *points += 1,
                        EnemyType::Second => *points += 2,
                        EnemyType::Third => *points += 3,
                        EnemyType::Fourth => *points += 4,
                        EnemyType::Fifth => *points += 5,
                        EnemyType::Final => *points += 10,
                    }
                }
            }
        }
    }

    // Remove entities from the remove entities list
    let mut write_world = world.write().unwrap();
    write_world.remove_entities(remove_entities);
}

/// If a level has been completed, increment the level and add some new enemies
pub fn spawn_more_enemies(world: Arc<RwLock<TowerDefenseWorld>>) {
    if *world.read().unwrap().alive_enemies.read().unwrap() == Some(0) {
        // Increment Level
        let next_level = {
            let world = world.read().unwrap();

            let mut level_ref = world.level.write().unwrap();
            *level_ref.as_mut().unwrap() += 1;
            *level_ref.as_ref().unwrap()
        };

        // Add enemies
        {
            let mut world = world.write().unwrap();

            // Add enemies
            let level_one_enemies = (next_level % 3) * 10;
            let level_two_enemies = ((next_level / 3) % 3) * 10;
            let level_three_enemies = ((next_level / 6) % 3) * 10;
            let level_four_enemies = ((next_level / 9) % 3) * 10;
            let level_five_enemies = ((next_level / 12) % 3) * 10;
            let level_six_enemies = ((next_level / 15) % 3) * 2;

            let level_one_positions = (0..level_one_enemies).map(|v| TOTAL_POSITIONS + v).collect();
            let level_two_positions = (0..level_two_enemies).map(|v| TOTAL_POSITIONS + level_one_enemies + v).collect();
            let level_three_positions = (0..level_three_enemies).map(|v| TOTAL_POSITIONS + level_one_enemies + level_two_enemies + v).collect();
            let level_four_positions = (0..level_four_enemies).map(|v| TOTAL_POSITIONS + level_one_enemies + level_two_enemies + level_three_enemies + v).collect();
            let level_five_positions = (0..level_four_enemies).map(|v| TOTAL_POSITIONS + level_one_enemies + level_two_enemies + level_three_enemies + level_four_enemies + v).collect();
            let level_six_positions = (0..level_six_enemies).map(|v| TOTAL_POSITIONS + level_one_enemies + level_two_enemies + v).collect();

            world.add_base_enemies(level_one_positions);
            world.add_second_enemies(level_two_positions);
            world.add_third_enemies(level_three_positions);
            world.add_fourth_enemies(level_four_positions);
            world.add_fifth_enemies(level_five_positions);
            world.add_final_enemies(level_six_positions);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alive_enemies_counts_alive_enemies() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize the Singular Components
            world.initialize_singular_components(100);

            // Spawn 100 enemies
            let positions = (1..101).map(|v| v).collect();
            let _ = world.add_base_enemies(positions);
        }

        count_alive_enemies(world.clone());

        // Check that 100 alive enemies were counted
        let read_world = world.read().unwrap();
        assert_eq!(read_world.alive_enemies.read().unwrap().unwrap(), 100);
    }

    #[test]
    fn test_alive_enemies_does_not_count_zero_health_enemies() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize the Singular Components
            world.initialize_singular_components(100);

            // Spawn 100 Enemies with 0 health
            let _ = world.add_enemies(
                (0..100).map(|_v| crate::EnemyType::Base).collect(),
                (0..100).map(|_v| String::from("X")).collect(),
                (1..101).collect(),
                (0..100).map(|_v| 0).collect(),
            );
        }

        count_alive_enemies(world.clone());

        // Check that 0 alive enemies were counted
        let read_world = world.read().unwrap();
        assert_eq!(read_world.alive_enemies.read().unwrap().unwrap(), 0);
    }

    #[test]
    fn test_remove_dead_entities_removes_dead_entities() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize the Singular Components
            world.initialize_singular_components(100);

            // Spawn 100 Enemies with 0 health
            let _ = world.add_enemies(
                (0..100).map(|_v| crate::EnemyType::Base).collect(),
                (0..100).map(|_v| String::from("X")).collect(),
                (1..101).collect(),
                (0..100).map(|_v| 0).collect(),
            );
        }

        remove_dead_entities(world.clone());

        // Check that there are 0 entities
        let read_world = world.read().unwrap();
        assert_eq!(read_world.tower_type.read().unwrap().len(), 0);
        assert_eq!(read_world.enemy_type.read().unwrap().len(), 0);
        assert_eq!(read_world.sprite.read().unwrap().len(), 0);
        assert_eq!(read_world.health.read().unwrap().len(), 0);
        assert_eq!(read_world.health_change.read().unwrap().len(), 0);
        assert_eq!(read_world.enemy_position.read().unwrap().len(), 0);
        assert_eq!(read_world.target_enemy.read().unwrap().len(), 0);
    }

    #[test]
    fn test_remove_dead_entities_ignores_entites_with_no_health_component() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize the Singular Components
            world.initialize_singular_components(100);

            // Create 100 entities with no health component
            let _ = world.add_entities(100);
        }

        remove_dead_entities(world.clone());

        // Check that there are still 100 entities
        let read_world = world.read().unwrap();
        assert_eq!(read_world.tower_type.read().unwrap().len(), 100);
        assert_eq!(read_world.enemy_type.read().unwrap().len(), 100);
        assert_eq!(read_world.sprite.read().unwrap().len(), 100);
        assert_eq!(read_world.health.read().unwrap().len(), 100);
        assert_eq!(read_world.health_change.read().unwrap().len(), 100);
        assert_eq!(read_world.enemy_position.read().unwrap().len(), 100);
        assert_eq!(read_world.target_enemy.read().unwrap().len(), 100);
    }

    #[test]
    fn test_spawn_enemies_level_2() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize singular components
            world.initialize_singular_components(100);
            world.set_level(1);
        }

        spawn_more_enemies(world.clone());

        // Make sure 20 level 1 enemies were instantiated
        let read_world = world.read().unwrap();
        assert_eq!(read_world.enemy_type.read().unwrap().iter().filter(|v| v.is_some() && v.unwrap() == EnemyType::Base).count(), 20);
        assert_eq!(*read_world.level.read().unwrap(), Some(2));
    }

    #[test]
    fn test_spawn_enemies_level_7() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize singular components
            world.initialize_singular_components(100);
            world.set_level(6);
        }

        spawn_more_enemies(world.clone());

        // Make sure 10 level 1 enemies and 20 level 2 enemies were instantiated
        let read_world = world.read().unwrap();
        assert_eq!(read_world.enemy_type.read().unwrap().iter().filter(|v| v.is_some() && v.unwrap() == EnemyType::Base).count(), 10);
        assert_eq!(read_world.enemy_type.read().unwrap().iter().filter(|v| v.is_some() && v.unwrap() == EnemyType::Second).count(), 20);
        assert_eq!(*read_world.level.read().unwrap(), Some(7));
    }
}