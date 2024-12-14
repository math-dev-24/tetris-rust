use ggez::graphics;
use rand::Rng;

#[derive(Clone)]
pub struct Block {
    pub shape: Shape,
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn new(shape: Shape, x: i32, y: i32) -> Self {
        Self { shape, x, y }
    }
    pub fn move_to(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
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

#[derive(Clone)]
pub struct Shape {
    pub blocks: Vec<(i32, i32)>,
    pub color: graphics::Color,
}

impl Shape {
    pub fn new(blocks: Vec<(i32, i32)>, color: graphics::Color) -> Self {
        Self { blocks, color }
    }
    pub fn rotate(&mut self, dx: i32, dy: i32) {
        for (x, y) in &mut self.blocks {
            *x += dx;
            *y += dy;
        }
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let shapes = vec![
            (vec![(0, 0), (1, 0), (0, 1), (1, 1)], graphics::Color::new(1.0, 0.0, 0.0, 1.0)), // Carré
            (vec![(0, 0), (0, 1), (0, 2), (0, 3)], graphics::Color::new(0.0, 1.0, 0.0, 1.0)), // Ligne
            (vec![(0, 0), (0, 1), (1, 1)], graphics::Color::new(0.0, 0.0, 1.0, 1.0)), // L
        ];
        let (blocks, color) = &shapes[rng.gen_range(0..shapes.len())];
        Self::new(blocks.clone(), *color)
    }
}

