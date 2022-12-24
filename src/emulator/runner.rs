use super::Emulator;
use std::time::Instant;

impl Emulator {
    /// Main emulator loop, runs the program loaded in memory
    pub fn main_loop(&mut self) {
        // this will be updated each cycle, so we can know when enough time has passed for a new one
        let mut tick = Instant::now();
        let mut display_tick = Instant::now();
        let mut display_changed = false;
        let mut rng = rand::thread_rng();

        loop {
            // when 1/60th of a second has passed, refresh display
            if display_tick.elapsed().as_micros() >= 16666 {
                if display_changed {
                    self.refresh_display();
                }

                display_tick = Instant::now();
            }

            if tick.elapsed().as_micros() < self.tick_us as u128 {
                // keep waiting if a full tick hasnt passed yet
                continue;
            }

            tick = Instant::now();

            let byte1 = self.memory[self.counter];
            let byte2 = self.memory[self.counter + 1];

            self.counter += 2;

            // get the first and second halves (nibbles) of each instruction byte
            let n1 = (byte1 & 0xF0) >> 4;
            let n2 = byte1 & 0x0F;
            let n3 = (byte2 & 0xF0) >> 4;
            let n4 = byte2 & 0x0F;

            // prepare values before matching instructions
            // all instructions with operands use some combination of this set
            let x = n2 as usize;
            let y = n3 as usize;
            let n = n4;
            let nn = byte2;
            let nnn = ((n2 as usize) << 8) + byte2 as usize;

            match (n1, n2, n3, n4) {
                (0x0, 0x0, 0xE, 0x0) => {
                    // 00E0 - Clear screen
                    self.display.clear();
                    display_changed = true;
                }
                (0x1, ..) => {
                    // 1NNN - Jump
                    // Jump to memory address `NNN`
                    self.counter = nnn;
                }
                (0x6, ..) => {
                    // 6XNN - Set register
                    // Set register `VX` to `NN`
                    self.reg[x] = nn;
                }
                (0x7, ..) => {
                    // 7XNN - Add to register
                    // Add `NN` to register `VX`
                    self.reg[x] = self.reg[x].wrapping_add(nn);
                }
                (0xA, ..) => {
                    // ANNN - Set index
                    // Set index register `I` to `NNN`
                    self.index = nnn;
                }
                (0xD, ..) => {
                    // DXYN - Display
                    // Displays the sprite found in memory at I with height N in position (VX,VY)
                    let coords = (self.reg[x], self.reg[y]);

                    // gets the sprite starting from I and going N pixels down
                    // each byte is a row of pixels
                    let sprite = &self.memory[self.index..self.index + n as usize];

                    self.reg[0xF] = self.display.draw(sprite, coords);
                    display_changed = true;
                }
                _ => continue, // ignore unknown instructions
            }
        }
    }
}
