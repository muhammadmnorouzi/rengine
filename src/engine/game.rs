use std::{thread , time::{Duration , Instant}};

pub type GameResult = Result<() , String>;
pub struct Game {}


impl Game {
    pub fn build() -> Game {
        Game {}
    }

    pub fn run(&mut self) -> GameResult {
        let mut counter : i32 = 0;

        let start_time = Instant::now();

        'main_loop: loop {
            counter += 1;

            println!("{:} {:}" , counter , start_time.elapsed().as_millis());

            if counter == 10 {
                break 'main_loop;
            }

            self.sleep_millis(100);
        }

        return Ok(());
    }

    fn sleep_millis(&self , millis : u64) {
        thread::sleep(Duration::from_millis(millis));
    } 
}