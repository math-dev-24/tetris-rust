use ggez::event::EventHandler;
use std::time::{Duration, Instant};
use ggez::{graphics, Context, GameError, GameResult};
use ggez::input::keyboard::KeyInput;
use crate::types::grid::Grid;
use crate::types::shapes::{Block, Shape};

pub struct GameState{
    grid: Grid,
    next_block: Block,
    score: u32,
    last_update: Instant,
    action: bool,
    is_game_over: bool,
}

impl GameState{
    const GRID_SIZE: (usize, usize) = (10, 20);
    pub fn new() -> Self {
        let weight = 25.0;
        let margin = 10.0;
        let mut grid = Grid::new(
            Self::GRID_SIZE.0,
            Self::GRID_SIZE.1,
            Block::new(Shape::random(), 0, 0, weight, margin),
            weight,
            margin,
        );

        Self {
            grid,
            next_block: Block::new(Shape::random(), 0, 0, weight, margin),
            score: 0,
            last_update: Instant::now(),
            action: false,
            is_game_over: false,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.last_update.elapsed() <= Duration::from_millis(500) && !self.action {
            return Ok(());
        }
        if self.is_game_over {
            return Ok(());
        }

        // je peux avancer ?
        if self.grid.is_valid_position(0, 1) {
            self.grid.active_block.row += 1;
        } else {
            // je ne peux pas avancer
            // next block + placer la shape
            self.grid.place_shape();

            let tmp_next_shape: Block = self.next_block.clone();
            self.next_block = Block::new(Shape::random(), 0, 0, self.grid.weight, self.grid.margin);
            self.grid.spawn_new_block(tmp_next_shape);

            // checker si une line est possible de clear
            let lines_cleared = self.grid.clear_full_lines();
            self.score += lines_cleared as u32 * 100;

            if !self.grid.is_valid_position(0, 0) {
                println!("Terminé, avec un score: {}", self.score);
                self.is_game_over = true;
            }
        }
        self.last_update = Instant::now();
        self.action = false;
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, None);
        self.grid.draw(&mut canvas, _ctx);
        self.grid.active_block.draw(&mut canvas, _ctx);
        canvas.finish(_ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        use ggez::input::keyboard::KeyCode;

        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Left => {
                    if self.grid.is_valid_position(-1, 0) {
                        self.grid.active_block.col -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.grid.is_valid_position(1, 0) {
                        self.grid.active_block.col += 1;
                    }
                }
                KeyCode::Down => {
                    if self.grid.is_valid_position(0, 1) {
                        self.grid.active_block.row += 1;
                    }
                }
                KeyCode::Space => {
                    self.grid.active_block.shape.rotate();
                }
                _ => {}
            }
        }
        self.action = true;
        Ok(())
    }

}
