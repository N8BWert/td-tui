//!
//! Tui Renderer using Ratatui
//! 

use std::io::{self, stdout, Stdout};
use std::sync::{Arc, RwLock};
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

use crate::{TowerDefenseWorld, TOTAL_POSITIONS};

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
                                "Tower Defense tui ----- Base Health: {}",
                                (*world.base_health.read().unwrap()).unwrap(),
                            )
                        )
                    )
                    .background_color(Color::Black)
                    .x_bounds([0.0, TOTAL_POSITIONS as f64])
                    .y_bounds([-25.0, 24.0])
                    .paint(|ctx| {
                        for (sprite, position) in world.sprite.read().unwrap().iter().zip(world.enemy_position.read().unwrap().iter()).filter(|v| v.0.is_some() && v.1.is_some()) {
                            ctx.print(position.unwrap() as f64, 0.0, sprite.as_ref().unwrap().clone().green());
                        }
                    }),
                area
            )
        });

        // Check for quit
        if event::poll(Duration::from_millis(5)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press &&
                    key.code == KeyCode::Char('q') {
                    return Err("Leaving Render Thread".into());
                }
            }
        }

        Ok(())
    }
}