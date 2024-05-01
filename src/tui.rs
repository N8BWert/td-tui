//!
//! Tui Renderer using Ratatui
//! 

use std::io::{self, stdout, Stdout};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::time::Duration;

use nate_engine::Renderer;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, Borders},
};

use crate::{TowerDefenseWorld, TowerTarget, TOTAL_POSITIONS, TOTAL_TOWERS, TOWER_SEPARATION};

fn set_tower_target(target_tower: u32, target: TowerTarget, mut target_enemies: RwLockWriteGuard<Vec<Option<TowerTarget>>>) {
    let mut current_tower = 0;
    for target_enemy in target_enemies.iter_mut().filter(|v| v.is_some()) {
        if current_tower == target_tower {
            *target_enemy = Some(target);
            return;
        }
        current_tower += 1;
    }
}

pub struct TowerDefenseRenderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TowerDefenseRenderer {
    pub fn new() -> io::Result<Self> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(Self {
            terminal,
        })
    }
}

impl Drop for TowerDefenseRenderer {
    fn drop(&mut self) {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}

impl Renderer<TowerDefenseWorld> for TowerDefenseRenderer {
    type Error = String;

    fn render(&mut self, world: Arc<RwLock<TowerDefenseWorld>>) -> Result<(), Self::Error> {
        let world = world.read().unwrap();

        let _err = self.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Canvas::default()
                    .block(
                        Block::default()
                        .borders(Borders::ALL)
                        .title(
                            format!(
                                "Tower Defense tui ----- Base Health: {} ----- Alive Enemies: {} ----- Points: {} ----- Level: {}",
                                (*world.base_health.read().unwrap()).unwrap(),
                                (*world.alive_enemies.read().unwrap()).unwrap(),
                                (*world.points.read().unwrap()).unwrap(),
                                (*world.level.read().unwrap()).unwrap(),
                            )
                        )
                    )
                    .background_color(Color::Black)
                    .x_bounds([0.0, TOTAL_POSITIONS as f64])
                    .y_bounds([-25.0, 24.0])
                    .paint(|ctx| {
                        if world.help_displayed.read().unwrap().unwrap() {
                            ctx.print(0.0, 9.0, "
                                q - quit                               d (right arrow) - move tower selection right     a (left arrow) - move tower selection left\n
                            ");
                            ctx.print(0.0, 0.0, "
                                w (up arrow) - upgrade tower           s (down arrow) - sell tower                      1 - make tower attack first enemy\n
                            ");
                            ctx.print(0.0, -10.0, "
                                2 - make tower attack second enemy     3 - make tower attack last enemy                 h - toggle help\n
                            ");
                        } else {
                            let mut tower_num = 0;
                            let sprite = world.sprite.read().unwrap();
                            let tower_type = world.tower_type.read().unwrap();
                            let health = world.health.read().unwrap();
                            let enemy_position = world.enemy_position.read().unwrap();
    
                            for ((((
                                _entity_id,
                                sprite),
                                tower_type),
                                health),
                                enemy_position
                            ) in sprite.iter().enumerate()
                                .zip(tower_type.iter())
                                .zip(health.iter())
                                .zip(enemy_position.iter()).filter(|v| v.0.0.0.1.is_some()) {
                                let sprite = sprite.as_ref().unwrap();
                                if tower_type.is_some() {
                                    // Draw Upgrade Cost Above Tower
                                    if tower_num == world.selected_tower.read().unwrap().unwrap() {
                                        ctx.print(
                                            TOWER_SEPARATION as f64 * (tower_num as f64 + 0.5),
                                            if tower_num % 2 == 0 { 12.0 } else { -12.0 },
                                            sprite.clone().white().bold().underlined(),
                                        );
                                        ctx.print(
                                            TOWER_SEPARATION as f64 * (tower_num as f64 + 0.5),
                                            if tower_num % 2 == 0 { 16.0 } else { -17.0 },
                                            format!("{}", tower_type.unwrap().upgrade_price()).bold(),
                                        )
                                    } else {
                                        ctx.print(
                                            TOWER_SEPARATION as f64 * (tower_num as f64 + 0.5),
                                            if tower_num % 2 == 0 { 12.0 } else { -12.0 },
                                            sprite.clone().white(),
                                        );
                                        ctx.print(
                                            TOWER_SEPARATION as f64 * (tower_num as f64 + 0.5),
                                            if tower_num % 2 == 0 { 16.0 } else { -17.0 },
                                            format!("{}", tower_type.unwrap().upgrade_price()),
                                        );
                                    }
                                    tower_num += 1;
                                } else if let (Some(position), Some(health)) = (enemy_position, health) {
                                    if *position < TOTAL_POSITIONS {
                                        match *health {
                                            0 => (),
                                            1 => ctx.print(*position as f64, 0.0, sprite.clone().red()),
                                            2 => ctx.print(*position as f64, 0.0, sprite.clone().light_red()),
                                            3 => ctx.print(*position as f64, 0.0, sprite.clone().light_yellow()),
                                            4 => ctx.print(*position as f64, 0.0, sprite.clone().yellow()),
                                            5 => ctx.print(*position as f64, 0.0, sprite.clone().light_green()),
                                            6 => ctx.print(*position as f64, 0.0, sprite.clone().green()),
                                            7 => ctx.print(*position as f64, 0.0, sprite.clone().light_blue()),
                                            8 => ctx.print(*position as f64, 0.0, sprite.clone().blue()),
                                            9 => ctx.print(*position as f64, 0.0, sprite.clone().light_cyan()),
                                            10 => ctx.print(*position as f64, 0.0, sprite.clone().cyan()),
                                            11 => ctx.print(*position as f64, 0.0, sprite.clone().light_magenta()),
                                            12 => ctx.print(*position as f64, 0.0, sprite.clone().magenta()),
                                            13 => ctx.print(*position as f64, 0.0, sprite.clone().gray()),
                                            14 => ctx.print(*position as f64, 0.0, sprite.clone().white()),
                                            15 => ctx.print(*position as f64, 0.0, sprite.clone().bold().red()),
                                            16 => ctx.print(*position as f64, 0.0, sprite.clone().bold().yellow()),
                                            17 => ctx.print(*position as f64, 0.0, sprite.clone().bold().green()),
                                            18 => ctx.print(*position as f64, 0.0, sprite.clone().bold().blue()),
                                            19 => ctx.print(*position as f64, 0.0, sprite.clone().bold().cyan()),
                                            _ => ctx.print(*position as f64, 0.0, sprite.clone().bold().magenta()),
                                        }
                                    }
                                }
                            }   
                        }
                    }),
                area
            )
        });

        // Check for quit
        if event::poll(Duration::from_millis(5)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Err("Leaving Render Thread".into()),
                        KeyCode::Char('a') | KeyCode::Left => {
                            // Retreat the selected tower
                            let selected_tower = world.selected_tower.read().unwrap().unwrap();
                            if selected_tower == 0 {
                                *world.selected_tower.write().unwrap() = Some(TOTAL_TOWERS - 1);
                            } else {
                                *world.selected_tower.write().unwrap() = Some(selected_tower - 1);
                            }
                        },
                        KeyCode::Char('d') | KeyCode::Right => {
                            // Advance the selected tower
                            let selected_tower = world.selected_tower.read().unwrap().unwrap();
                            *world.selected_tower.write().unwrap() = Some((selected_tower + 1) % TOTAL_TOWERS);
                        },
                        KeyCode::Char('1') => {
                            // Set the selected tower to target the first enemy
                            set_tower_target(
                                world.selected_tower.read().unwrap().unwrap(),
                                TowerTarget::First,
                                world.target_enemy.write().unwrap(),
                            );
                        },
                        KeyCode::Char('2') => {
                            // Set the selected tower to target the second enemy
                            set_tower_target(
                                world.selected_tower.read().unwrap().unwrap(),
                                TowerTarget::Second,
                                world.target_enemy.write().unwrap(),
                            );
                        },
                        KeyCode::Char('3') => {
                            // Set the selected tower to target the last enemy
                            set_tower_target(
                                world.selected_tower.read().unwrap().unwrap(),
                                TowerTarget::Last,
                                world.target_enemy.write().unwrap(),
                            );
                        },
                        KeyCode::Char('h') => {
                            // Toggle Help on and off
                            if world.help_displayed.read().unwrap().unwrap() {
                                *world.help_displayed.write().unwrap() = Some(false);
                            } else {
                                *world.help_displayed.write().unwrap() = Some(true);
                            }
                        },
                        KeyCode::Char('w') | KeyCode::Up => {
                            *world.upgrading_tower.write().unwrap() = Some(true);
                        },
                        KeyCode::Char('s') | KeyCode::Down => {
                            *world.downgrading_tower.write().unwrap() = Some(true);
                        }
                        _ => (),
                    }
                }
                if key.kind == KeyEventKind::Press &&
                    key.code == KeyCode::Char('q') {
                    return Err("Leaving Render Thread".into());
                }
            }
        }

        Ok(())
    }
}