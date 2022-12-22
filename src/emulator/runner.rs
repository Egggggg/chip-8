use super::model::Emulator;
use std::time::Instant;

impl Emulator {
    /// Main emulator loop, runs the program loaded in memory
    pub fn main_loop(&mut self) {
        // this will be updated each cycle, so we can know when enough time has passed for a new one
        let mut tick = Instant::now();

        loop {
            // `tick` will never get too much higher than `self.tick_us`, so we can coerce it down to u16
            if (tick.elapsed().as_micros() as u16) < self.tick_us {
                // keep waiting if a full tick hasnt passed yet
                continue;
            }

            tick = Instant::now();
        }
    }
}
