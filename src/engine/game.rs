use super::{GameLogic, GameResult, GameSettings};
use crate::utilities::Logger;
use sdl2::{render::WindowCanvas, EventPump, Sdl, VideoSubsystem};
use std::{thread, time::Duration};

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

    pub fn run(&mut self, logic: &mut dyn GameLogic, logger: Logger) -> GameResult {
        logger.info("before load ... ");
        logic.load()?;
        logger.info("after load ... ");

        logger.info("before loop ... ");
        'main_loop: loop {
            for event in self.event_pump.poll_iter() {
                if let None = logic.on_event(event) {
                    logger.info("Breaking loop .. ");
                    break 'main_loop;
                }
            }

            logic.update()?;
            logic.draw(&mut self.canvas)?;

            self.sleep();
        }

        logger.info("after loop ... ");

        return Ok(());
    }

    // fps
    fn sleep(&self) {
        thread::sleep(Duration::new(0, 1_000_000_000u32 / self.settings.get_fps()));
    }
}
