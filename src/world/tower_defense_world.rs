//!
//! Tower Defense World
//! 

use nate_engine::world;

use crate::{EnemyType, TowerTarget, TowerType, TOWER_SEPARATION};

/// World the running tower defense games
#[world(
    singular = [
        base_health,
        base_damage,
        removal_entities,
        alive_enemies,
        selected_tower,
        help_displayed,
        points,
        level,
        upgrading_tower,
        downgrading_tower,
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
    // Tower the cursor is on
    selected_tower: u32,
    // Whether or not help is being displayed
    help_displayed: bool,
    // Points obtained
    points: u32,
    // Current Level
    level: u32,
    // Upgrading the current tower (flag passed by the input handler)
    upgrading_tower: bool,
    // Downgrading the current tower (flag passed by the input handler)
    downgrading_tower: bool,
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

    /// Add a singular second enemy
    pub fn add_second_enemy(&mut self, position: u32) -> usize {
        self.add_enemy(
            EnemyType::Second,
            String::from("Q"),
            position,
            2,
        )
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

    /// Add a broken tower
    pub fn add_broken_tower(
        &mut self,
    ) -> usize {
        self.add_tower(
            TowerType::Broken,
            TowerTarget::First,
            (0, 0),
            String::from("-"),
        )
    }

    /// Add a base tower (range of 5 units)
    pub fn add_base_tower(
        &mut self,
        target_enemy: TowerTarget,
        tower_number: u32,
    ) -> usize {
        let start = TOWER_SEPARATION * tower_number;
        let end = TOWER_SEPARATION * (tower_number + 1);
        let midpoint = (start + end) / 2;
        self.add_tower(
            TowerType::Base,
            target_enemy,
            (midpoint - 2, midpoint + 2),
            String::from("!"),
        )
    }

    /// Upgrade a given tower
    pub fn upgrade_tower(
        &self,
        tower_number: u32,
        entity_id: usize,
        current_tower_type: TowerType,
    ) {
        let midpoint = TOWER_SEPARATION / 2 * (2 * tower_number + 1);
        match current_tower_type {
            TowerType::Broken => {
                *self.tower_type.write().unwrap().get_mut(entity_id).expect("Entity Id Must Be Valid") = Some(TowerType::Base);
                *self.tower_bounds.write().unwrap().get_mut(entity_id).expect("Entity ID Must be Valid") = Some((midpoint - 2, midpoint + 2));
                *self.sprite.write().unwrap().get_mut(entity_id).expect("Entity Id must be valid") = Some(String::from("!"));
            },
            TowerType::Base => {
                
            },
        }
    }

    /// Sell a given tower
    pub fn sell_tower(
        &self,
        tower_number: u32,
        entity_id: usize,
        current_tower_type: TowerType,
    ) {
        let _midpoint = TOWER_SEPARATION / 2 * (2 * tower_number + 1);
        match current_tower_type {
            TowerType::Broken => {

            },
            TowerType::Base => {
                *self.tower_type.write().unwrap().get_mut(entity_id).expect("Entity ID Must Be Valid") = Some(TowerType::Broken);
                *self.tower_bounds.write().unwrap().get_mut(entity_id).expect("Entity ID Must Be Valid") = Some((0, 0));
                *self.sprite.write().unwrap().get_mut(entity_id).expect("Entity Id Must Be Valid") = Some(String::from("-"));
            },
        }
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

    /// Add a bunch of second enemies
    pub fn add_second_enemies(&mut self, positions: Vec<u32>) -> Vec<usize> {
        let healths = positions.iter().map(|_v| 2).collect();
        self.add_enemies(
            positions.iter().map(|_v| EnemyType::Second).collect(),
            positions.iter().map(|_v| String::from("Q")).collect(),
            positions,
            healths,
        )
    }

    /// Add a bunch of broken towers
    pub fn add_broken_towers(&mut self, towers: usize) -> Vec<usize> {
        let tower_ids = self.add_entities(towers);
        self.set_tower_types(&tower_ids, vec![TowerType::Broken; towers]);
        self.set_target_enemys(&tower_ids, vec![TowerTarget::First; towers]);
        self.set_tower_boundss(&tower_ids, vec![(0, 0); towers]);
        self.set_sprites(&tower_ids, vec![String::from("-"); towers]);
        tower_ids
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
        self.set_selected_tower(0);
        self.set_help_displayed(false);
        self.set_points(10);
        self.set_level(1);
        self.set_upgrading_tower(false);
        self.set_downgrading_tower(false);
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
