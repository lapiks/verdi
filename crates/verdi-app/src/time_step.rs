use std::time::Instant;

pub struct TimeStep {
    delta_time: f32,
    last_time: Instant,
}

impl TimeStep {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            last_time: Instant::now(),
        }
    }

    pub fn delta(&mut self) -> f32 {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_millis() as f32 * 0.001;
        self.last_time = current_time;
        self.delta_time = delta;
        delta
    }
}