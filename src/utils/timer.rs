pub struct Timer {
    period: f32,
    current_time: f32,
    is_stopped: bool,
    is_completed: bool,
}

impl Timer {
    ///How long till the alarm is sounded - in seconds
    pub fn new(period: f32) -> Self {
        Timer {
            period,
            current_time: 0f32,
            is_stopped: false,
            is_completed: false,
        }
    }

    pub fn new_stopped(period: f32) -> Self {
        let mut timer = Self::new(period);
        timer.stop();

        timer
    }

    pub fn tick(&mut self, delta_t: f32) -> bool {
        self.current_time += delta_t;

        self.is_completed = self.current_time >= self.period;

        self.is_completed
    }

    pub fn timer_elapsed(&self) -> bool {
        self.is_completed
    }

    pub fn stop(&mut self) {
        self.is_stopped = true;
    }

    pub fn start(&mut self) {
        self.is_stopped = false;
    }

    pub fn reset(&mut self) {
        self.is_completed = false;
        self.current_time = 0f32;
        self.is_stopped = false;
    }
}
