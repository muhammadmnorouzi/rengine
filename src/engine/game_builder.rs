use super::{
    game::Game, game_builder_settings::GameBuilderSettings, window_settings::WindowSettings,
};
use sdl2::pixels::Color;
use sdl2::Sdl;

pub struct GameBuilder<'a> {
    reserved: &'a (),
}

impl<'a> GameBuilder<'a> {
    pub fn build(settings: &'a GameBuilderSettings<'a>) -> Result<Game<'a>, String> {
        let context: Sdl = sdl2::init()?;
        let video_subsystem = context.video()?;

        let window = video_subsystem
            .window(
                settings.get_window_settings().get_title(),
                settings.get_window_settings().get_width(),
                settings.get_window_settings().get_height(),
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        let event_pump = context.event_pump()?;

        let game: Game = Game::new(
            settings.get_game_settings(),
            context,
            video_subsystem,
            canvas,
            event_pump,
        );

        Ok(game)
    }
}
