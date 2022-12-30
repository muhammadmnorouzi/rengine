use rengine::engine::{
    game::Game, game_builder::GameBuilder, game_builder_settings::GameBuilderSettings,
    game_settings::GameSettings, types::GameResult, window_settings::WindowSettings,
};

fn main() -> GameResult {
    let window_settings = WindowSettings::new("RENGINE", 800, 600);
    let game_settings = GameSettings::new(30u32);

    let game_builder_settings = GameBuilderSettings::new(&window_settings, &game_settings);
    let mut game = GameBuilder::build(&game_builder_settings)?;

    game.run()?;

    Ok(())
}
