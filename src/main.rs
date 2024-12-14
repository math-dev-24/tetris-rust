mod types;
mod tests;

use ggez::{ContextBuilder, event};
use types::game::GameState;

fn main() -> ggez::GameResult {
    let (ctx, event_loop) =
        ContextBuilder::new("tetris", "Mathieu B")
        .build()
        .expect("Aie, Erreur lors de la création du context");

    let state = GameState::new();

    event::run(ctx, event_loop, state)
}