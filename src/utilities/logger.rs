pub struct Logger {}

impl Logger {
    pub fn info(&self , log : &str) {
        self.main_logger("info" , log);
    }

    pub fn debug(&self , log : &str) {
        self.main_logger("debug" , log);
    }

    pub fn warning(&self , log : &str) {
        self.main_logger("warning" , log);
    }

    fn main_logger(&self , level : &str , log :impl Into<String> ) {
        println!("{:} >> {:} |" ,level  ,log.into());
    }
}