pub enum TimerState {
    Play,
    Pause,
    End,
}

pub struct PomodoroTimer {
    pub current_time: u32,
    pub initial_work_seconds: u32,
    pub state: TimerState,
}

impl PomodoroTimer {
    pub fn new(initial_minutes: u32) -> Self {
        let seconds = initial_minutes * 60;
        PomodoroTimer {
            current_time: seconds,
            initial_work_seconds: seconds,
            state: TimerState::Pause,
        }
    }

    pub fn tick(&mut self) {
        if self.current_time == 0 {
            return;
        }

        self.current_time -= 1;

        if self.current_time == 0 {
            self.state = TimerState::End;
            println!("Timer reached zero!");
        }
    }

    // Logic: Snap back to the stored base time
    pub fn reset_to_base(&mut self) {
        self.current_time = self.initial_work_seconds;
        self.state = TimerState::Pause;
    }

    // Adjusts the base time and snaps the current time to it
    pub fn adjust_time(&mut self, offset: i32) {
        let new_time = (self.initial_work_seconds as i32) + offset;
        // Clamp between 5 minutes and 60 minutes
        let clamped = new_time.clamp(300, 3600) as u32;

        self.initial_work_seconds = clamped;
        self.current_time = clamped;
    }
}
