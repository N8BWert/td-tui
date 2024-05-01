//!
//! Tower-Specific Systems
//! 

use std::sync::RwLockReadGuard;

use crate::{TowerType, TowerTarget, TowerDefenseWorld};

use nate_engine::system;

/// Find the First, Second, or Last Enemy within range to target.
fn find_target_enemy_id(
    tower_bounds: &(u32, u32),
    target_enemy: &TowerTarget,
    enemy_position: RwLockReadGuard<Vec<Option<u32>>>
) -> Option<usize> {
    match target_enemy {
        TowerTarget::First => {
            let mut lowest_enemy_id: Option<usize> = None;
            let mut lowest_enemy_position: u32 = u32::MAX;
            for (enemy_id, enemy_position) in enemy_position.iter().enumerate().filter(|v| v.1.is_some()) {
                let enemy_position = enemy_position.unwrap();
                if tower_bounds.0 == enemy_position {
                    lowest_enemy_id = Some(enemy_id);
                    break;
                } else if tower_bounds.0 < enemy_position &&
                    enemy_position < tower_bounds.1 &&
                    enemy_position <= lowest_enemy_position {
                    lowest_enemy_id = Some(enemy_id);
                    lowest_enemy_position = enemy_position;
                }
            }
            lowest_enemy_id
        },
        TowerTarget::Second => {
            let mut lowest_enemy_id: Option<usize> = None;
            let mut lowest_enemy_position: u32 = u32::MAX;
            let mut second_enemy_id: Option<usize> = None;
            let mut second_enemy_position: u32 = u32::MAX;
            for (enemy_id, enemy_position) in enemy_position.iter().enumerate().filter(|v| v.1.is_some()) {
                let enemy_position = enemy_position.unwrap();
                if tower_bounds.0 <= enemy_position &&
                    enemy_position <= tower_bounds.1 &&
                    enemy_position < second_enemy_position {
                    if enemy_position < lowest_enemy_position {
                        second_enemy_id = lowest_enemy_id;
                        second_enemy_position = lowest_enemy_position;
                        lowest_enemy_id = Some(enemy_id);
                        lowest_enemy_position = enemy_position;
                    } else {
                        second_enemy_id = Some(enemy_id);
                        second_enemy_position = enemy_position;
                    }
                }
            }

            if second_enemy_id.is_some() {
                second_enemy_id
            } else {
                lowest_enemy_id
            }
        },
        TowerTarget::Last => {
            let mut highest_enemy_id: Option<usize> = None;
            let mut highest_enemy_position: u32 = 0;
            for (enemy_id, enemy_position) in enemy_position.iter().enumerate().filter(|v| v.1.is_some()) {
                let enemy_position = enemy_position.unwrap();
                if tower_bounds.1 == enemy_position {
                    highest_enemy_id = Some(enemy_id);
                    break;
                } else if tower_bounds.0 <= enemy_position &&
                    enemy_position < tower_bounds.1 &&
                    enemy_position >= highest_enemy_position {
                    highest_enemy_id = Some(enemy_id);
                    highest_enemy_position = enemy_position;
                }
            }
            highest_enemy_id
        }
    }
}

/// Do 1 damage per second to a given enemy
#[system(
    world=TowerDefenseWorld,
    read=[tower_type, target_enemy, tower_bounds],
    filter=[*tower_type == TowerType::Base]
)]
pub fn base_tower_attack_ai() {
    let target_enemy_id = find_target_enemy_id(tower_bounds, target_enemy, world.enemy_position.read().unwrap());

    // Attack the enemy
    if let Some(target_enemy_id) = target_enemy_id {
        let mut health = world.health.write().unwrap();
        if let Some(health_value) = health[target_enemy_id] {
            health[target_enemy_id] = Some(health_value.saturating_sub(1));
        }
    }
}

/// Check whether a tower should be upgraded and, if so, upgrade the tower
#[system(
    world=TowerDefenseWorld,
    _write=[upgrading_tower, points],
    _read=[selected_tower],
)]
pub fn upgrade_tower() {
    if *upgrading_tower {
        let mut current_tower_type: Option<TowerType> = None;
        let mut current_entity_id: Option<usize> = None;

        let mut current_tower = 0;
        for (entity_id, tower_type) in world.tower_type.read().unwrap().iter().enumerate().filter(|v| v.1.is_some()) {
            if current_tower == *selected_tower {
                current_tower_type = Some(tower_type.unwrap());
                current_entity_id = Some(entity_id);
                break;
            }
            current_tower += 1;
        }

        if let (Some(current_tower_type), Some(current_entity_id)) = (current_tower_type, current_entity_id) {
            if *points >= current_tower_type.upgrade_price() {
                world.upgrade_tower(*selected_tower, current_entity_id, current_tower_type);
                *points -= current_tower_type.upgrade_price();
            }
        }

        *upgrading_tower = false;
    }
}

/// Check whether a tower should be sold and, if so, sell the tower
#[system(
    world=TowerDefenseWorld,
    _write=[downgrading_tower, points],
    _read=[selected_tower],
)]
pub fn downgrade_tower() {
    if *downgrading_tower {
        let mut current_tower_type: Option<TowerType> = None;
        let mut current_entity_id: Option<usize> = None;

        let mut current_tower = 0;
        for (entity_id, tower_type) in world.tower_type.read().unwrap().iter().enumerate().filter(|v| v.1.is_some()) {
            if current_tower == *selected_tower {
                current_tower_type = Some(tower_type.unwrap());
                current_entity_id = Some(entity_id);
                break;
            }
            current_tower += 1;
        }

        if let (Some(current_tower_type), Some(current_entity_id)) = (current_tower_type, current_entity_id) {
            world.sell_tower(*selected_tower, current_entity_id, current_tower_type);
            *points += current_tower_type.sell_price();
        }

        *downgrading_tower = false;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;

    #[test]
    fn test_find_first_target_enemy_in_range() {
        let tower_bounds = (0, 5);
        let target_enemy = TowerTarget::First;
        let enemy_positions = RwLock::new(vec![
            None,
            Some(3),
            Some(6),
            Some(22),
            Some(2),
            Some(1),
        ]);
        assert_eq!(
            find_target_enemy_id(
                &tower_bounds,
                &target_enemy,
                enemy_positions.read().unwrap(),
            ),
            Some(5),
        );
    }

    #[test]
    fn test_find_first_target_enemy_on_left_bound() {
        let tower_bounds = (0, 5);
        let target_enemy = TowerTarget::First;
        let enemy_positions = RwLock::new(vec![
            Some(3),
            None,
            Some(6),
            Some(22),
            Some(0),
            None,
            Some(1),
        ]);
        assert_eq!(
            find_target_enemy_id(
                &tower_bounds,
                &target_enemy,
                enemy_positions.read().unwrap(),
            ),
            Some(4),
        );
    }

    #[test]
    fn test_find_first_target_enemy_out_of_range() {
        let tower_bounds = (0, 5);
        let target_enemy = TowerTarget::First;
        let enemy_positions = RwLock::new(vec![
            Some(6),
            Some(8),
            None,
            None,
            Some(10),
            Some(11),
            Some(12),
            Some(6),
        ]);
        assert_eq!(
            find_target_enemy_id(
                &tower_bounds,
                &target_enemy,
                enemy_positions.read().unwrap(),
            ),
            None,
        );
    }

    #[test]
    fn test_find_second_target_enemy_in_range() {
        let tower_bounds = (0, 5);
        let target_enemy = TowerTarget::Second;
        let enemy_positions = RwLock::new(vec![
            None,
            Some(1),
            Some(6),
            Some(7),
            Some(2),
            None,
            Some(3),
        ]);
        assert_eq!(
            find_target_enemy_id(
                &tower_bounds,
                &target_enemy,
                enemy_positions.read().unwrap(),
            ),
            Some(4)
        );
    }

    #[test]
    fn test_find_second_target_enemy_only_one_in_range() {
        let tower_bounds = (0, 5);
        let target_enemy = TowerTarget::Second;
        let enemy_positions = RwLock::new(vec![
            Some(1),
            Some(6),
            None,
            None,
            Some(7),
            Some(9),
        ]);
        assert_eq!(
            find_target_enemy_id(
                &tower_bounds,
                &target_enemy,
                enemy_positions.read().unwrap(),
            ),
            Some(0),
        );
    }

    #[test]
    fn test_find_second_target_enemy_out_of_range() {
        let tower_bounds = (0, 5);
        let target_enemy = TowerTarget::Second;
        let enemy_positions = RwLock::new(vec![
            None,
            Some(6),
            Some(7),
            None,
            Some(8),
            Some(9),
        ]);
        assert_eq!(
            find_target_enemy_id(
                &tower_bounds,
                &target_enemy,
                enemy_positions.read().unwrap(),
            ),
            None
        )
    }

    #[test]
    fn test_find_final_target_enemy_in_range() {
        let tower_bounds = (0, 5);
        let target_enemy = TowerTarget::Last;
        let enemy_positions = RwLock::new(vec![
            None,
            None,
            Some(4),
            Some(7),
            Some(100),
            Some(101),
        ]);
        assert_eq!(
            find_target_enemy_id(
                &tower_bounds,
                &target_enemy,
                enemy_positions.read().unwrap(),
            ),
            Some(2),
        );
    }

    #[test]
    fn test_find_final_target_enemy_on_right_bound() {
        let tower_bounds = (0, 5);
        let target_enemy = TowerTarget::Last;
        let enemy_positions = RwLock::new(vec![
            Some(4),
            Some(1),
            None,
            Some(3),
            Some(8),
            None,
            Some(6),
            Some(5),
        ]);
        assert_eq!(
            find_target_enemy_id(
                &tower_bounds,
                &target_enemy,
                enemy_positions.read().unwrap(),
            ),
            Some(7),
        );
    }

    #[test]
    fn test_find_final_target_enemy_out_of_range() {
        let tower_bounds = (0, 5);
        let target_enemy = TowerTarget::Last;
        let enemy_positions = RwLock::new(vec![
            Some(6),
            Some(7),
            None,
            None,
            Some(8),
            Some(9),
        ]);
        assert_eq!(
            find_target_enemy_id(
                &tower_bounds,
                &target_enemy,
                enemy_positions.read().unwrap(),
            ),
            None,
        );
    }

    #[test]
    fn test_base_tower_attack_first_enemy() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);

            // Add a base tower
            let _ = world.add_tower(TowerType::Base, TowerTarget::First, (10, 30), String::from("!"));

            // Add a few base enemies
            let _ = world.add_base_enemies(vec![12, 13, 20, 25, 30, 33, 36, 38]);
        }

        base_tower_attack_ai(world.clone());

        // Make sure the first enemy lost health (i.e. it has 0 health now)
        let read_world = world.read().unwrap();
        assert_eq!(read_world.health.read().unwrap()[1].unwrap(), 0);
    }

    #[test]
    fn test_base_tower_attack_second_enemy() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);

            // Add a base tower
            let _ = world.add_tower(TowerType::Base, TowerTarget::Second, (10, 30), String::from("!"));

            // Add a few base enemies
            let _ = world.add_base_enemies(vec![12, 13, 20, 25, 30, 33, 36, 38]);
        }

        base_tower_attack_ai(world.clone());

        // Make sure the second enemy lost health (i.e. it has 0 health now)
        let read_world = world.read().unwrap();
        assert_eq!(read_world.health.read().unwrap()[2].unwrap(), 0);
    }

    #[test]
    fn test_base_tower_attack_last_enemy() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);

            // Add a base tower
            let _ = world.add_tower(TowerType::Base, TowerTarget::Last, (10, 30), String::from("!"));

            // Add a few base enemies
            let _ = world.add_base_enemies(vec![12, 13, 20, 25, 30, 33, 36, 38]);
        }

        base_tower_attack_ai(world.clone());

        // Make sure the 5th enemy lost health (i.e. it has 0 health now)
        let read_world = world.read().unwrap();
        assert_eq!(read_world.health.read().unwrap()[5].unwrap(), 0);
    }

    #[test]
    fn test_upgrade_tower_no_tower_to_upgrade() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);
            world.set_selected_tower(0);
            world.set_upgrading_tower(false);
            world.set_points(10);

            // Add a broken tower
            let _ = world.add_broken_tower();
        }

        upgrade_tower(world.clone());

        // Make sure the broken tower wasn't upgraded and the current points didn't change
        let read_world = world.read().unwrap();
        assert_eq!(read_world.tower_type.read().unwrap()[0], Some(TowerType::Broken));
        assert_eq!(*read_world.points.read().unwrap(), Some(10));
    }

    #[test]
    fn test_upgrade_tower_first_tower() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);
            world.set_selected_tower(0);
            world.set_upgrading_tower(true);
            world.set_points(10);

            // Add a broken tower
            let _ = world.add_broken_tower();
        }

        upgrade_tower(world.clone());

        // Make sure the broken tower was upgraded and the current points is 0
        let read_world = world.read().unwrap();
        assert_eq!(read_world.tower_type.read().unwrap()[0], Some(TowerType::Base));
        assert_eq!(read_world.tower_bounds.read().unwrap()[0], Some((3, 7)));
        assert_eq!(read_world.sprite.read().unwrap()[0], Some(String::from("!")));
        assert_eq!(*read_world.points.read().unwrap(), Some(0));
        assert_eq!(*read_world.upgrading_tower.read().unwrap(), Some(false));
    }

    #[test]
    fn test_upgrade_a_tower() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);
            world.set_selected_tower(3);
            world.set_upgrading_tower(true);
            world.set_points(10);

            // Add a few broken towers
            let _ = world.add_broken_towers(10);
        }

        upgrade_tower(world.clone());

        // Make sure the 4th broken tower was upgraded
        let read_world = world.read().unwrap();
        assert_eq!(read_world.tower_type.read().unwrap()[3], Some(TowerType::Base));
        assert_eq!(read_world.tower_bounds.read().unwrap()[3], Some((33, 37)));
        assert_eq!(read_world.sprite.read().unwrap()[3], Some(String::from("!")));
        assert_eq!(*read_world.points.read().unwrap(), Some(0));
        assert_eq!(*read_world.upgrading_tower.read().unwrap(), Some(false));
    }

    #[test]
    fn test_upgrade_tower_not_enough_points() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);
            world.set_selected_tower(0);
            world.set_upgrading_tower(true);
            world.set_points(5);

            // Add a broken tower
            world.add_broken_tower();
        }

        upgrade_tower(world.clone());

        // Make sure the tower wasn't upgraded
        let read_world = world.read().unwrap();
        assert_eq!(read_world.tower_type.read().unwrap()[0], Some(TowerType::Broken));
        assert_eq!(*read_world.points.read().unwrap(), Some(5));
        assert_eq!(*read_world.upgrading_tower.read().unwrap(), Some(false));
    }

    #[test]
    fn test_downgrade_tower_not_downgrading_tower() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);
            world.set_points(10);

            // Add a base tower
            world.add_base_tower(TowerTarget::First, 0);
        }

        downgrade_tower(world.clone());

        // Make sure the tower wasn't sold
        let read_world = world.read().unwrap();
        assert_eq!(read_world.tower_type.read().unwrap()[0], Some(TowerType::Base));
        assert_eq!(*read_world.points.read().unwrap(), Some(10));
    }

    #[test]
    fn test_sell_first_tower() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);
            world.set_points(10);
            world.set_selected_tower(0);
            world.set_downgrading_tower(true);

            // Add a base tower
            world.add_base_tower(TowerTarget::First, 0);
        }

        downgrade_tower(world.clone());

        // Make sure the tower was sold
        let read_world = world.read().unwrap();
        assert_eq!(read_world.tower_type.read().unwrap()[0], Some(TowerType::Broken));
        assert_eq!(read_world.tower_bounds.read().unwrap()[0], Some((0, 0)));
        assert_eq!(read_world.sprite.read().unwrap()[0], Some(String::from("-")));
        assert_eq!(*read_world.points.read().unwrap(), Some(15));
        assert_eq!(*read_world.downgrading_tower.read().unwrap(), Some(false));
    }

    #[test]
    fn test_sell_tower() {
        let world = TowerDefenseWorld::new();

        {
            let mut world = world.write().unwrap();

            // Initialize Singular Components
            world.initialize_singular_components(100);
            world.set_points(10);
            world.set_selected_tower(2);
            world.set_downgrading_tower(true);

            // Add 3 base towers
            world.add_base_tower(TowerTarget::First, 0);
            world.add_base_tower(TowerTarget::First, 1);
            world.add_base_tower(TowerTarget::First, 2);
        }

        downgrade_tower(world.clone());

        // Make sure the tower was sold
        let read_world = world.read().unwrap();
        assert_eq!(read_world.tower_type.read().unwrap()[2], Some(TowerType::Broken));
        assert_eq!(read_world.tower_bounds.read().unwrap()[2], Some((0, 0)));
        assert_eq!(read_world.sprite.read().unwrap()[2], Some(String::from("-")));
        assert_eq!(*read_world.points.read().unwrap(), Some(15));
        assert_eq!(*read_world.downgrading_tower.read().unwrap(), Some(false));
    }
}