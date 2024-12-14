use ggez::graphics;
use rand::Rng;
use crate::types::static_shape::SHAPES;

#[derive(Clone, Debug)]
pub struct Block {
    pub shape: Shape,
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn new(shape: Shape, x: i32, y: i32) -> Self {
        Self { shape, x, y }
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut ggez::Context) {
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.x as f32 * 10.0,
                self.y as f32 * 10.0,
                10.0,
                10.0,
            ),
            self.shape.color,
        )
        .unwrap();
        canvas.draw(&mesh, graphics::DrawParam::default());
    }
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub blocks: Vec<(i32, i32)>,
    pub color: graphics::Color,
}

impl Shape {
    pub fn new(blocks: Vec<(i32, i32)>, color: graphics::Color) -> Self {
        Self { blocks, color }
    }
    pub fn rotate(&mut self) {
        self.blocks = self.blocks
            .iter()
            .map(|&(x, y)| {
                println!("x : {}, y : {}", x, y);
                let rotated_x = y;
                let rotated_y = x * -1;
                println!("rotated_x : {}, rotated_y : {}", rotated_x, rotated_y);
                (rotated_x, rotated_y)
            })
            .collect();
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        SHAPES[rng.gen_range(0..SHAPES.len())].clone()
    }
}


