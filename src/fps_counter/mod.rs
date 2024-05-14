use std::time::Instant;

use crate::config::EVENT_INTERVAL;

#[derive(Debug)]
pub struct FPSCounter {
    pub frame_count: u64,
    pub last_frame_time: Instant,
    pub skipped_frames: f64,
}

impl Default for FPSCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl FPSCounter {
    fn new() -> FPSCounter {
        FPSCounter {
            frame_count: 0,
            last_frame_time: Instant::now(),
            skipped_frames: 0.0,
        }
    }

    pub fn update(&mut self) -> f64 {
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time);
        let seconds = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;
        let fps = self.frame_count as f64 / seconds;
        let target_fps = (1000 / EVENT_INTERVAL) as f64;
        let diff = target_fps - fps;
        if diff >= 1.0 {
            self.skipped_frames += diff;
        }
        self.last_frame_time = now;
        self.frame_count = 0;
        fps
    }
}
