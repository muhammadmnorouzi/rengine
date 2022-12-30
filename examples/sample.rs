use rengine::engine::game::{Game , GameResult };

fn main() -> GameResult {
    Game::build().run()?;

    Ok(())
}