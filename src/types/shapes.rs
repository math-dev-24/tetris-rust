use ggez::graphics;
use rand::Rng;
use crate::types::static_shape::SHAPES;

#[derive(Clone, Debug)]
pub struct Block {
    pub shape: Shape,
    pub col: i32,
    pub row: i32,
    pub weight: f32,
    pub margin: f32,
}

impl Block {
    pub fn new(shape: Shape, x: i32, y: i32, w: f32, m: f32) -> Self {
        Self {shape, col: x, row: y, weight: w, margin: m}
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut ggez::Context) {
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.col as f32 * self.weight + self.margin,
                self.row as f32 * self.weight + self.margin,
                self.weight,
                self.weight,
            ),
            self.shape.color,
        )
        .unwrap();
        canvas.draw(&mesh, graphics::DrawParam::default());
    }
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub shape_position: Vec<(i32, i32)>,
    pub color: graphics::Color,
}

impl Shape {
    pub fn new(blocks: Vec<(i32, i32)>, color: graphics::Color) -> Self {
        Self { shape_position: blocks, color }
    }
    pub fn rotate(&mut self) {
        self.shape_position = self.shape_position
            .iter()
            .map(|&(x, y)| {
                let rotated_x = y;
                let rotated_y = x * -1;
                (rotated_x, rotated_y)
            })
            .collect();
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        SHAPES[rng.gen_range(0..SHAPES.len())].clone()
    }
}


