pub struct Game {
    pub running: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            running: false, 
        }
    }
}

impl Game {
    pub fn run(&self) {
        
    }
}