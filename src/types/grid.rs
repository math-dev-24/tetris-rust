use ggez::graphics;
use crate::types::shapes::Block;


pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub weight: f32,
    pub margin: f32,
    cells: Vec<Vec<Option<graphics::Color>>>,
    pub active_block: Block,
}

impl Grid {
    pub fn new(width: usize, height: usize, block: Block, w: f32, m: f32) -> Self {
        Self {
            width,
            height,
            weight: w,
            margin: m,
            cells: vec![vec![None; width]; height],
            active_block: block,
        }
    }

    pub fn spawn_new_block(&mut self, next_block: Block) {
        self.active_block = next_block;
    }

    // vérification si je peux avancer
    pub fn is_valid_position(&self, dx_col: i32, dx_row: i32) -> bool {
        for &(d_col, d_row) in &self.active_block.shape.shape_position {
            let n_col = self.active_block.col + d_col + dx_col;
            let n_row = self.active_block.row + d_row + dx_row;

            if n_col < 0 || n_col >= self.width as i32 || n_row < 0 || n_row >= self.height as i32 {
                return false;
            }
            if self.cells[n_row as usize][n_col as usize].is_some() {
                return false;
            }
        }
        true
    }

    // Place une forme sur la grille
    pub fn place_shape(&mut self) {
        for &(d_col, d_row) in &self.active_block.shape.shape_position {
            let n_col = self.active_block.col + d_col;
            let n_row = self.active_block.row + d_row;

            if n_col >= 0 && n_col < self.width as i32 && n_row >= 0 && n_row < self.height as i32 {
                self.cells[n_row as usize][n_col as usize] = Some(self.active_block.shape.color);
            }
        }
    }

    // Supprime les lignes complètes
    pub fn clear_full_lines(&mut self) -> usize {
        let mut cleared_lines = 0;

        self.cells.retain(|row| {
            if row.iter().all(|cell| cell.is_some()) {
                cleared_lines += 1;
                false
            } else {
                true
            }
        });

        for _ in 0..cleared_lines {
            self.cells.insert(0, vec![None; self.width]);
        }

        cleared_lines
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut ggez::Context) {

        let rect_board = graphics::Rect::new(
            self.margin,
            self.margin,
            self.width as f32 * self.weight,
            self.height as f32 * self.weight
        );

        let filled_board = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect_board,
            graphics::Color::new(0.2, 0.2, 0.2, 1.0),
        )
            .unwrap();
        canvas.draw(&filled_board, graphics::DrawParam::default());

        let border_board = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(3.0),
            rect_board,
            graphics::Color::new(0.0, 0.0, 0.0, 1.0),
        )
            .unwrap();
        canvas.draw(&border_board, graphics::DrawParam::default());

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let rect = graphics::Rect::new(
                    x as f32 * self.weight + self.margin,
                    y as f32 * self.weight + self.margin,
                    self.weight,
                    self.weight,
                );

                if let Some(color) = cell {
                    let filled_mesh = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        rect,
                        *color,
                    )
                        .unwrap();
                    canvas.draw(&filled_mesh, graphics::DrawParam::default());

                    let border_mesh = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(1.0),
                        rect,
                        graphics::Color::new(0.2, 0.2, 0.2, 1.0),
                    )
                        .unwrap();
                    canvas.draw(&border_mesh, graphics::DrawParam::default());
                }else {

                    let border_mesh = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::stroke(1.0),
                        rect,
                        graphics::Color::new(0.1, 0.1, 0.1, 1.0),
                    )
                        .unwrap();
                    canvas.draw(&border_mesh, graphics::DrawParam::default());
                }
            }
        }
    }
}
