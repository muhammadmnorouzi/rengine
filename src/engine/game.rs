use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::VideoSubsystem;
use std::{
    thread,
    time::{Duration, Instant},
};

use super::game_settings::GameSettings;
use super::types::GameResult;

pub struct Game<'a> {
    settings: &'a GameSettings,
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
}

impl<'a> Game<'a> {
    pub(crate) fn new(
        settings: &'a GameSettings,
        sdl: Sdl,
        video_subsystem: VideoSubsystem,
        canvas: WindowCanvas,
        event_pump: EventPump,
    ) -> Self {
        Self {
            settings,
            sdl,
            video_subsystem,
            canvas,
            event_pump,
        }
    }

    pub fn run(&mut self) -> GameResult {
        let start_time = Instant::now();

        'main_loop: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main_loop,
                    _ => {}
                }
            }

            println!("{:} ", start_time.elapsed().as_millis());

            self.canvas.clear();
            self.canvas.present();

            self.sleep();
        }

        return Ok(());
    }

    fn sleep(&self) {
        thread::sleep(Duration::new(0, 1_000_000_000u32 / self.settings.get_fps()));
    }
}
