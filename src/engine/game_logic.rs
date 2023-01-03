use super::{GameEvent, GameResult};

pub trait GameLogic {
    fn load(&mut self) -> GameResult;
    fn update(&self) -> GameResult;
    fn draw(&self) -> GameResult;
    fn on_event(&self, event: GameEvent) -> Option<GameResult>;
}
