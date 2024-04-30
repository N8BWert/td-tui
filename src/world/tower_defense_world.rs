//!
//! Tower Defense World
//! 

use nate_engine::world;

use crate::{TowerType, EnemyType, TowerTarget};

#[world(
    singular = [
        base_health,
        base_damage,
        removal_entities,
        alive_enemies,
    ]
)]
pub struct TowerDefenseWorld {
    // The tower type for each tower
    tower_type: TowerType,
    // The enemy type for each enemy
    enemy_type: EnemyType,
    // The sprite for a displayable entity (string)
    sprite: String,
    // The health that an enemy has left
    health: u32,
    // The damage being done to an entity
    health_change: i32,
    // The position of the enemy [0, TOTAL_POSITIONS)
    enemy_position: u32,
    // The enemy index for a tower to target
    target_enemy: TowerTarget,
    // the left and right most position the tower can attack
    tower_bounds: (u32, u32),

    // The current health of the base
    base_health: u32,
    // The current damage to deal to the base
    base_damage: i32,
    // Entities to remove at the next remove step
    removal_entities: Vec<usize>,
    // Total Alive Enemies
    alive_enemies: u32,
}

impl TowerDefenseWorld {
    /// Add a singular enemy with its components
    pub fn add_enemy(
        &mut self,
        enemy_type: EnemyType,
        sprite: String,
        position: u32,
        health: u32
    ) -> usize {
        let enemy_id = self.add_entity();
        self.set_enemy_type(enemy_id, enemy_type);
        self.set_sprite(enemy_id, sprite);
        self.set_enemy_position(enemy_id, position);
        self.set_health(enemy_id, health);
        enemy_id
    }

    /// Add a singular base enemy
    pub fn add_base_enemy(&mut self, position: u32) -> usize {
        self.add_enemy(EnemyType::Base, String::from("X"), position, 1)
    }

    /// Add a tower entity
    pub fn add_tower(
        &mut self,
        tower_type: TowerType,
        target_enemy: TowerTarget,
        tower_bounds: (u32, u32),
        sprite: String,
    ) -> usize {
        let tower_id = self.add_entity();
        self.set_tower_type(tower_id, tower_type);
        self.set_target_enemy(tower_id, target_enemy);
        self.set_tower_bounds(tower_id, tower_bounds);
        self.set_sprite(tower_id, sprite);
        tower_id
    }

    /// Add a bunch of enemies with their components
    pub fn add_enemies(
        &mut self,
        enemy_types: Vec<EnemyType>,
        sprites: Vec<String>,
        positions: Vec<u32>,
        healths: Vec<u32>,
    ) -> Vec<usize> {
        let enemy_ids = self.add_entities(enemy_types.len());
        self.set_enemy_types(&enemy_ids, enemy_types);
        self.set_sprites(&enemy_ids, sprites);
        self.set_enemy_positions(&enemy_ids, positions);
        self.set_healths(&enemy_ids, healths);
        enemy_ids
    }

    /// Add a bunch of base enemies
    pub fn add_base_enemies(&mut self, positions: Vec<u32>) -> Vec<usize> {
        let healths = positions.iter().map(|_v| 1).collect();
        self.add_enemies(
            positions.iter().map(|_v| EnemyType::Base).collect(),
            positions.iter().map(|_v| String::from("X")).collect(),
            positions,
            healths,
        )
    }

    /// Initialize the singular components of the world
    pub fn initialize_singular_components(
        &mut self,
        base_health: u32,
    ) {
        self.set_base_health(base_health);
        self.set_base_damage(0);
        self.set_removal_entities(Vec::new());
        self.set_alive_enemies(0);
    }

    pub fn print_world(&mut self) {
        println!("Tower Defense World:\n");
        println!("Tower Types: {:?}", self.tower_type.read().unwrap());
        println!("Enemy Types: {:?}", self.enemy_type.read().unwrap());
        println!("Sprites: {:?}", self.sprite.read().unwrap());
        println!("Healths: {:?}", self.health.read().unwrap());
        println!("Health Changes: {:?}", self.health_change.read().unwrap());
        println!("Enemy Positions: {:?}", self.enemy_position.read().unwrap());
        println!("Target Enemies: {:?}", self.target_enemy.read().unwrap());
        println!("Tower Bounds: {:?}", self.tower_bounds.read().unwrap());
        println!("Base Health: {:?}", self.base_health.read().unwrap());
        println!("Base Damage: {:?}", self.base_damage.read().unwrap());
        println!("Removal Entities: {:?}", self.removal_entities.read().unwrap());
        println!("Alive Enemies: {:?}", self.alive_enemies.read().unwrap());
        println!("\n");
    }
}
