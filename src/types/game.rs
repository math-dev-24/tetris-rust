use ggez::graphics::{self, Color, Mesh};
use ggez::{Context, GameResult};


const GRID_WIDTH: usize = 100;
const CELL_WIDTH: f32 = 10.0;
const GRID_HEIGHT: usize = 100;

pub type Grid = [[Option<u8>; GRID_WIDTH]; GRID_HEIGHT];

pub struct Block {
    shape: Vec<Vec<u8>>,
    x: i32,
    y: i32,
}

pub struct GameState {
    grid: Grid,
    current_block: Block,
    next_block: Block,
    score: u32,
    game_over: bool,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            grid: [[None; GRID_WIDTH]; GRID_HEIGHT],
            current_block: Block {
                shape: vec![vec![1, 1], vec![1, 1]],
                x: (GRID_WIDTH / 2) as i32,
                y: 0,
            },
            next_block: Block {
                shape: vec![vec![1, 1], vec![1, 1]],
                x: (GRID_WIDTH / 2) as i32,
                y: 0,
            },
            score: 0,
            game_over: false,
        }
    }
}


impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        for i in 0..self.grid[0].len() {
            let x = i as f32 * CELL_WIDTH;
            let line = Mesh::new_line(
                ctx,                 
                &[ggez::mint::Point2 { x, y: 0.0 }, ggez::mint::Point2 { x, y: GRID_HEIGHT as f32 * CELL_WIDTH }],
                1.0,
                Color::BLACK
                ).expect("Erreur lors de la construction de la grille");
            canvas.draw(&line, graphics::DrawParam::default());
        }
        canvas.finish(ctx)
    }
}