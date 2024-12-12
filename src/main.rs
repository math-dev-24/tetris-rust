mod types;

use ggez::ContextBuilder;
use types::game::GameState;



fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("tetris", "Mathieu B")
    .build().expect("Aie, Erreur lors de la création du context");

    let state = GameState::new(&mut ctx);

    ggez::event::run(ctx, event_loop, state)

}
