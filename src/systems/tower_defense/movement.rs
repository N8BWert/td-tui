//!
//! Movement Systems
//! 

use nate_engine::system;

use crate::EnemyType;
use crate::world::tower_defense_world::TowerDefenseWorld;

#[system(
    world=TowerDefenseWorld,
    read=[enemy_type],
    write=[health, enemy_position],
    _write=[base_health],
    filter=[*enemy_type == EnemyType::Base],
)]
pub fn base_enemy_movement_system() {
    *enemy_position = enemy_position.saturating_sub(1);
    if *enemy_position == 0 {
        *base_health = base_health.saturating_sub(1);
        *health = health.saturating_sub(1);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_base_enemy_moves_one_space_per_update() {
        let world = TowerDefenseWorld::new();

        let enemy_id: usize;
        {
            let mut world = world.write().unwrap();

            // Initialize the singular components
            world.initialize_singular_components(100);

            // Add an enemy at position 30
            enemy_id = world.add_enemy(
                EnemyType::Base,
                String::from("X"),
                30,
                1
            );
        }

        base_enemy_movement_system(world.clone());

        // Check that the Enemy moved to position 29
        let read_world = world.read().unwrap();
        assert_eq!(read_world.enemy_position.read().unwrap()[enemy_id].unwrap(), 29);
    }

    #[test]
    fn test_base_enemy_deals_one_base_damage_at_position_0() {
        let world = TowerDefenseWorld::new();

        let enemy_id: usize;
        {
            let mut world = world.write().unwrap();

            // Initialize the singular components
            world.initialize_singular_components(100);

            // Add an enemy at position 1
            enemy_id = world.add_enemy(
                EnemyType::Base,
                String::from("X"),
                1,
                1
            );
        }

        base_enemy_movement_system(world.clone());

        // Check that the enemy is at position 0, has 0 health, and the bas health is 99
        let read_world = world.read().unwrap();
        assert_eq!(read_world.enemy_position.read().unwrap()[enemy_id].unwrap(), 0);
        assert_eq!(read_world.health.read().unwrap()[enemy_id].unwrap(), 0);
        assert_eq!(read_world.base_health.read().unwrap().unwrap(), 99);
    }
}