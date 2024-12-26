mod types;
mod tests;

use ggez::{ContextBuilder, event};
use ggez::conf::WindowMode;
use types::game::GameState;

fn main() -> ggez::GameResult {
    let (ctx, event_loop) =
        ContextBuilder::new("Tetris by Math", "Mathieu B")
            .window_mode(WindowMode::default().dimensions(1000.0, 1000.0))
        .build()
        .expect("Aie, Erreur lors de la création de l'ouverture du jeu");

    let state = GameState::new();

    event::run(ctx, event_loop, state)
}