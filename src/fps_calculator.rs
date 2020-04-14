use std::collections::VecDeque;
use std::time::Instant;

use crate::observer::RenderLoopObserver;

const FPS_ARRAY_SIZE: usize = 100;

pub struct FpsCalculator {
    frame_times: VecDeque<Instant>,
}

impl RenderLoopObserver for FpsCalculator {
    fn new() -> Self {
        let mut frame_times: VecDeque<Instant> = VecDeque::with_capacity(FPS_ARRAY_SIZE);
        frame_times.push_back(Instant::now());
        FpsCalculator { frame_times }
    }

    fn tick(&mut self) {
        let earliest_frame = if self.frame_times.len() == FPS_ARRAY_SIZE {
            self.frame_times.pop_front().unwrap()
        } else {
            *(self.frame_times.front().unwrap())
        };
        let elapsed = earliest_frame.elapsed();
        let fps = 1000000.0 * self.frame_times.len() as f64 / elapsed.as_micros() as f64;
        println!("FPS: {:?}, elapsed: {:?}", fps, elapsed);
        self.frame_times.push_back(Instant::now());
    }
}
