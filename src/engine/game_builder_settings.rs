use super::{game_settings::GameSettings, window_settings::WindowSettings};

pub struct GameBuilderSettings<'a> {
    window_settings: &'a WindowSettings<'a>,
    game_settings: &'a GameSettings,
}

impl<'a> GameBuilderSettings<'a> {
    pub fn new(window_settings: &'a WindowSettings<'a>, game_settings: &'a GameSettings) -> Self {
        Self {
            window_settings,
            game_settings,
        }
    }

    pub fn get_window_settings(&self) -> &WindowSettings<'a> {
        return self.window_settings;
    }

    pub fn get_game_settings(&self) -> &GameSettings {
        return self.game_settings;
    }
}
