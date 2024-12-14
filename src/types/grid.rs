use ggez::graphics;
use crate::types::shapes::Shape;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Option<graphics::Color>>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![None; width]; height],
        }
    }

    // Vérifie si une position est valide (pas de collision, dans les limites)
    pub fn is_valid_position(&self, shape: &Shape, x: i32, y: i32) -> bool {
        for &(dx, dy) in &shape.blocks {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx >= self.width as i32 || ny < 0 || ny >= self.height as i32 {
                return false; // Hors des limites
            }
            if self.cells[ny as usize][nx as usize].is_some() {
                return false; // Collision
            }
        }
        true
    }

    // Place une forme sur la grille
    pub fn place_shape(&mut self, shape: &Shape, x: i32, y: i32) {
        for &(dx, dy) in &shape.blocks {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                self.cells[ny as usize][nx as usize] = Some(shape.color);
            }
        }
    }

    // Supprime les lignes complètes
    pub fn clear_full_lines(&mut self) -> usize {
        let mut cleared_lines = 0;

        self.cells.retain(|row| {
            if row.iter().all(|cell| cell.is_some()) {
                cleared_lines += 1;
                false // Supprime la ligne
            } else {
                true
            }
        });

        // Ajoute des lignes vides en haut
        for _ in 0..cleared_lines {
            self.cells.insert(0, vec![None; self.width]);
        }

        cleared_lines
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut ggez::Context) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(color) = cell {
                    let mesh = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(x as f32 * 10.0, y as f32 * 10.0, 10.0, 10.0),
                        *color,
                    )
                        .unwrap();
                    canvas.draw(&mesh, graphics::DrawParam::default());
                }
            }
        }
    }
}
