use ggez::graphics;
use once_cell::sync::Lazy;
use crate::types::shapes::Shape;

pub static SCUBE: Lazy<Shape> = Lazy::new(|| Shape {
    blocks: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    color: graphics::Color::new(1.0, 0.0, 0.0, 1.0),
});

pub static SLINE: Lazy<Shape> = Lazy::new(|| Shape {
    blocks: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
    color: graphics::Color::new(0.0, 1.0, 0.0, 1.0),
});

pub static SL: Lazy<Shape> = Lazy::new(|| Shape {
    blocks: vec![(0, 0), (1, 0), (1, 1)],
    color: graphics::Color::new(0.0, 0.0, 1.0, 1.0),
});

pub static SZ: Lazy<Shape> = Lazy::new(|| Shape {
    blocks: vec![(0, 0), (1, 0), (1, 1), (2, 1)],
    color: graphics::Color::new(1.0, 1.0, 0.0, 1.0),
});

pub static SHAPES: Lazy<Vec<&'static Shape>> = Lazy::new(|| vec![&SCUBE, &SLINE, &SL, &SZ]);