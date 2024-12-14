use ggez::event::EventHandler;
use std::time::{Duration, Instant};
use ggez::{graphics, Context, GameError, GameResult};
use ggez::input::keyboard::KeyInput;
use crate::types::grid::Grid;
use crate::types::shapes::{Block, Shape};

pub struct GameState{
    grid: Grid,
    current_shape: Block,
    next_shape: Block,
    score: u32,
    last_update: Instant,
    action: bool,
}

impl GameState{
    const GRID_SIZE: (usize, usize) = (10, 20);
    pub fn new() -> Self {
        let grid = Grid::new(Self::GRID_SIZE.0, Self::GRID_SIZE.1);
        Self {
            grid,
            current_shape: Block::new(Shape::random(), 0, 0),
            next_shape: Block::new(Shape::random(), 0, 0),
            score: 0,
            last_update: Instant::now(),
            action: false,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.last_update.elapsed() <= Duration::from_secs(1) && !self.action {
            return Ok(());
        }
        self.last_update = Instant::now();
        self.action = false;

        if self.grid.is_valid_position(&self.current_shape.shape, self.current_shape.x, self.current_shape.y + 1) {
            self.current_shape.y += 1;
        } else {
            self.grid.place_shape(&self.current_shape.shape, self.current_shape.x, self.current_shape.y);

            let lines_cleared = self.grid.clear_full_lines();
            self.score += lines_cleared as u32 * 100;

            self.current_shape = self.next_shape.clone();
            self.next_shape = Block::new(Shape::random(), 5, 0);

            if !self.grid.is_valid_position(&self.current_shape.shape, self.current_shape.x, self.current_shape.y) {
                println!("Game Over!");
                return Ok(());
            }
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, None);
        self.grid.draw(&mut canvas, _ctx);
        self.current_shape.draw(&mut canvas, _ctx);
        self.next_shape.draw(&mut canvas, _ctx);
        canvas.finish(_ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        use ggez::input::keyboard::KeyCode;

        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Left => {
                    if self.grid.is_valid_position(&self.current_shape.shape, self.current_shape.x - 1, self.current_shape.y) {
                        self.current_shape.x -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.grid.is_valid_position(&self.current_shape.shape, self.current_shape.x + 1, self.current_shape.y) {
                        self.current_shape.x += 1;
                    }
                }
                KeyCode::Down => {
                    if self.grid.is_valid_position(&self.current_shape.shape, self.current_shape.x, self.current_shape.y + 1) {
                        self.current_shape.y += 1;
                    }
                }
                KeyCode::Space => {
                    self.current_shape.shape.rotate();
                }
                _ => {}
            }
        }
        self.action = true;
        Ok(())
    }

}
