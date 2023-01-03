use super::{GameEvent, GameResult};
use sdl2::render::WindowCanvas;

pub trait GameLogic {
    fn load(&mut self) -> GameResult;
    fn update(&mut self) -> GameResult;
    fn draw(&self, canvas: &mut WindowCanvas) -> GameResult;
    fn on_event(&self, event: GameEvent) -> Option<GameResult>;
}
