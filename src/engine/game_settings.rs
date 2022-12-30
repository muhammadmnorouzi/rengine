pub struct GameSettings {
    fps: u32,
}

impl GameSettings {
    pub fn new(fps: u32) -> Self {
        Self { fps }
    }

    pub fn get_fps(&self) -> u32 {
        self.fps
    }
}
