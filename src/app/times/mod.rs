use std::time::Instant;

pub struct TimeServiceImpl {
    pub last_frame_time: Instant,
    pub time: f32,
    pub frames_per_sec: u32,
    pub frames: u32
}

impl TimeServiceImpl {
    pub fn new() -> Self {
        Self {
            last_frame_time: Instant::now(),
            time: 0f32,
            frames_per_sec: 0u32,
            frames: 0u32
        }
    }

    pub fn calcul_delta_time(&mut self) -> f32 {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = current_time;
        self.time += delta_time;
        self.frames += 1;
        if self.time >= 1f32 {
            self.time = 0f32;
            self.frames_per_sec = self.frames;
            self.frames = 0;
        }

        delta_time
    }
}