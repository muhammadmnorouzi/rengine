use rengine::engine::{
    GameBuilder, GameBuilderSettings, GameEvent, GameKeyCode, GameLogic, GameResult, GameSettings,
    WindowSettings,
};
use rengine::sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};
use rengine::utilities::Logger;
use std::time::Instant;

const FPS: u32 = 30;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> GameResult {
    let window_settings = WindowSettings::new("RENGINE", WIDTH, HEIGHT);
    let game_settings = GameSettings::new(FPS);
    let game_builder_settings = GameBuilderSettings::new(&window_settings, &game_settings);
    let mut game = GameBuilder::build(&game_builder_settings)?;
    let mut logic: MyGame = MyGame::default();
    let logger = Logger {};

    game.run(&mut logic, logger)?;

    Ok(())
}

pub struct MyGame {
    start_time: Instant,
    rect: Rect,
}

impl MyGame {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            rect: Rect::new(0, 0, 0, 0),
        }
    }
}

impl GameLogic for MyGame {
    fn load(&mut self) -> GameResult {
        println!("Game started ... ");
        self.start_time = Instant::now();
        self.rect = Rect::new(50, 50, 20, 20);
        Ok(())
    }

    fn update(&mut self) -> GameResult {
        // println!("{:} ", self.start_time.elapsed().as_millis());
        let new_x = self.rect.x() + 1;
        self.rect.set_x(new_x);

        Ok(())
    }

    fn draw(&self, canvas: &mut WindowCanvas) -> GameResult {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(self.rect)?;

        canvas.present();

        Ok(())
    }

    fn on_event(&self, event: GameEvent) -> Option<GameResult> {
        return match event {
            GameEvent::Quit { .. }
            | GameEvent::KeyDown {
                keycode: Some(GameKeyCode::Escape),
                ..
            } => None,
            _ => Some(Ok(())),
        };
    }
}
