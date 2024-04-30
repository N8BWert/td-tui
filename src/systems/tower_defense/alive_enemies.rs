//!
//! Systems dealing with alive enemies
//! 

#![allow(unused)]

use std::sync::{Arc, RwLock};

use nate_engine::system;

use crate::TowerDefenseWorld;

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
        for (entity_id, health) in health.iter().enumerate().rev() {
            if let Some(health) = health.as_ref() {
                if *health == 0 {
                    remove_entities.push(entity_id);
                }
            }
        }
    }

    // Remove entities from the remove entities list
    let mut write_world = world.write().unwrap();
    write_world.remove_entities(remove_entities);
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
}