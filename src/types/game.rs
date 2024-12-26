use ggez::event::EventHandler;
use std::time::{Duration, Instant};
use ggez::{graphics, Context, GameError, GameResult};
use ggez::graphics::{Color, DrawParam, Text, TextFragment, PxScale, FontData};
use ggez::input::keyboard::KeyInput;
use rand::Rng;
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
        let weight = 40.0;
        let margin = 30.0;
        let grid = Grid::new(
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

    fn draw_next_block(&self, canvas: &mut graphics::Canvas, _ctx: &mut ggez::Context) {
        let index_x = self.grid.width as f32 * self.grid.weight + (self.grid.margin * 2.0);
        let mut index_y = 100.0;

        let text_fragment = TextFragment {
            text: "Suivant :".to_string(),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(40.0)),
            ..Default::default()
        };

        let draw_params = DrawParam::default()
            .dest([index_x, index_y]);

        let text = Text::new(text_fragment);
        canvas.draw(&text, draw_params);

        index_y += 40.0;

        let rect = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                index_x,
                index_y,
                6.0 * self.grid.weight,
                6.0 * self.grid.weight,
            ),
            Color::new(0.3, 0.3, 0.3, 1.0),
        ).unwrap();

        canvas.draw(&rect, graphics::DrawParam::default());

        for (d_col, d_row) in &self.next_block.shape.shape_position {
            let n_col = *d_col as f32;
            let n_row = *d_row as f32;

            let mesh = graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        index_x + n_col * self.grid.weight + self.grid.margin * 2.0,
                        index_y + n_row * self.grid.weight + self.grid.margin * 2.0,
                        self.grid.weight,
                        self.grid.weight,
                    ),
                    self.next_block.shape.color,
            ).unwrap();
            canvas.draw(&mesh, graphics::DrawParam::default());
        }
    }

    fn draw_score(&self, canvas: &mut graphics::Canvas, _ctx: &mut ggez::Context) {

        let x = self.grid.weight * self.grid.width as f32 + self.grid.margin * 2.0;
        let y =  20.0;

        let rect = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                x,
                y,
                self.grid.weight * self.grid.width as f32,
                50.0,
            ),
            Color::new(0.3, 0.3, 0.3, 1.0),
        ).unwrap();

        canvas.draw(&rect, graphics::DrawParam::default());

        let text_fragment = TextFragment {
            text: format!("Score: {}", self.score),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(40.0)),
            ..Default::default()
        };
        let draw_params = DrawParam::default()
            .dest([x + 14.0, y + 7.0]);

        let text = Text::new(text_fragment);
        canvas.draw(&text, draw_params);
    }

    fn draw_lose(&self, canvas: &mut graphics::Canvas, _ctx: &mut ggez::Context) {

        let rect = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.0,
                0.0,
                1000.0,
                1000.0,
            ),
            Color::new(1.0, 0.0, 0.0, 1.0),
        ).unwrap();

        canvas.draw(&rect, graphics::DrawParam::default());

        let text_fragment = TextFragment {
            text: "Game Over".to_string(),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(60.0)),
            ..Default::default()
        };

        let draw_params = DrawParam::default()
            .dest([200.0, 200.0]);

        let text = Text::new(text_fragment);
        canvas.draw(&text, draw_params);
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        if self.is_game_over {
            return Ok(());
        }

        let mut time_millis_second = 300;
        // augmentation de la vitesse de jeu
        if self.score > 1000 && self.score < 1500 {
            time_millis_second = 200;
        }

        if self.score > 1500 {
            time_millis_second = 100;
        }

        if self.last_update.elapsed() <= Duration::from_millis(time_millis_second) && !self.action {
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
            let rand_x = rand::thread_rng().gen_range(0..(self.grid.width - 4));

            self.next_block = Block::new(Shape::random(), rand_x as i32, 0, self.grid.weight, self.grid.margin);
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
        self.draw_score(&mut canvas, _ctx);
        self.draw_next_block(&mut canvas, _ctx);
        if self.is_game_over {
            self.draw_lose(&mut canvas, _ctx);
        }
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
                    let mut tmp_shape = self.grid.active_block.shape.clone();
                    tmp_shape.rotate();
                    let mut can_rotate = true;

                    for &(d_col, d_row) in &tmp_shape.shape_position {
                        let n_col = self.grid.active_block.col + d_col;
                        let n_row = self.grid.active_block.row + d_row;
                        if n_col < 0 || n_col >= self.grid.width as i32 || n_row < 0 || n_row >= self.grid.height as i32 {
                            can_rotate = false;
                            break;
                        }
                    }
                    if can_rotate {
                        self.grid.active_block.shape.rotate();
                    }
                }
                _ => {}
            }
        }
        self.action = true;
        Ok(())
    }

}
