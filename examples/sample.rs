use rengine::engine::{
    GameBuilder, GameBuilderSettings, GameEvent, GameKeyCode, GameLogic, GameResult, GameSettings,
    WindowSettings,
};
use rengine::utilities::Logger;
use rengine::sdl2::render::WindowCanvas;
use std::time::Instant;

fn main() -> GameResult {

    let window_settings = WindowSettings::new("RENGINE", 800, 600);
    let game_settings = GameSettings::new(1u32);
    // let game_settings = GameSettings::new(30u32);

    let game_builder_settings = GameBuilderSettings::new(&window_settings, &game_settings);
    let mut game = GameBuilder::build(&game_builder_settings)?;

    let mut logic: MyGame = MyGame {
        start_time: Instant::now(),
    };

    let logger = Logger {};

    game.run(&mut logic , logger)?;

    Ok(())
}

pub struct MyGame {
    start_time: Instant,
}

impl GameLogic for MyGame {
    fn load(&mut self) -> GameResult {
        println!("Game started ... ");
        self.start_time = Instant::now();

        Ok(())
    }

    fn update(&self) -> GameResult {
        println!("{:} ", self.start_time.elapsed().as_millis());

        Ok(())
    }

    fn draw(&self , canvas : &mut WindowCanvas) -> GameResult {
        println!("drawing ... ");

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