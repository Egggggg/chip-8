use super::model::Emulator;
use std::time::Instant;

impl Emulator {
    /// Main emulator loop, runs the program loaded in memory
    pub fn main_loop(&mut self) {
        // this will be updated each cycle, so we can know when enough time has passed for a new one
        let mut tick = Instant::now();
        let mut display_tick = Instant::now();
        let mut display_changed = false;

        loop {
            if tick.elapsed().as_micros() < self.tick_us as u128 {
                // keep waiting if a full tick hasnt passed yet
                continue;
            }

            tick = Instant::now();

            // when 1/60th of a second has passed, refresh display
            if display_tick.elapsed().as_micros() >= 16666 {
                if display_changed {
                    self.refresh_display();
                }
                display_tick = Instant::now();
            }

            let byte1 = self.memory[self.counter];
            let byte2 = self.memory[self.counter + 1];

            self.counter += 2;

            // get the first and second halves (nibbles) of each instruction byte
            let n1 = byte1 & 0x0F;
            let n2 = byte1 & 0xF0;
            let n3 = byte2 & 0x0F;
            let n4 = byte2 & 0xF0;

            match (n1, n2, n3, n4) {
                (0x0, 0x0, 0xE, 0x0) => {
                    // 00E0 - clear screen
                    self.display.clear();
                    display_changed = true;
                }
                (0x1, n1, n2, n3) => {
                    // 1NNN - jump
                    // combine [0, n1, n2, n3] into a 16 bit address
                    let addr = u16::from_be_bytes(&[0, n1, n2, n3]);
                }
                _ => continue,
            }
        }
    }

    fn refresh_display(&mut self) {}
}
