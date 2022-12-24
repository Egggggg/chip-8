use rand::Rng;

use super::{memory::FONT_ADDR, Emulator};

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

                self.timer = self.timer.wrapping_sub(1);
                self.s_timer = self.s_timer.wrapping_sub(1);

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
                (0x0, 0x0, 0xE, 0xE) => {
                    // 00EE - End subroutine
                    // move to the last address on the stack
                    self.counter = self.stack.pop_back().unwrap_or(self.counter);
                }
                (0x1, ..) => {
                    // 1NNN - Jump
                    // Jump to memory address `NNN`
                    self.counter = nnn;
                }
                (0x2, ..) => {
                    // 2NNN - Start subroutine
                    self.stack.push_back(self.counter);
                    self.counter = nnn;
                }
                (0x3, ..) => {
                    // 3XNN - Skip if equal to immediate
                    if self.reg[x] == nn {
                        self.counter += 2;
                    }
                }
                (0x4, ..) => {
                    // 4XNN - Skip if not equal to immediate
                    if self.reg[x] != nn {
                        self.counter += 2;
                    }
                }
                (0x5, _, _, 0x0) => {
                    // 5XY0 - Skip if equal
                    if self.reg[x] == self.reg[y] {
                        self.counter += 2;
                    }
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
                (0x8, _, _, 0x0) => {
                    // 8XY0 - Set to other register
                    self.reg[x] = self.reg[y];
                }
                (0x8, _, _, 0x1) => {
                    // 8XY1 - OR
                    self.reg[x] |= self.reg[y];
                }
                (0x8, _, _, 0x2) => {
                    // 8XY2 - AND
                    self.reg[x] &= self.reg[y];
                }
                (0x8, _, _, 0x3) => {
                    // 8XY3 - XOR
                    self.reg[x] ^= self.reg[y];
                }
                (0x8, _, _, 0x4) => {
                    // 8XY4 - Add registers
                    self.reg[x] = self.reg[x].wrapping_add(self.reg[y]);
                }
                (0x8, _, _, 0x5) => {
                    // 8XY5 - Subtract Y from X
                    // set carry flag if overflowed
                    self.reg[0xF] = if self.reg[y] > self.reg[x] { 1 } else { 0 };
                    self.reg[x] = self.reg[x].wrapping_sub(self.reg[y]);
                }
                (0x8, _, _, 0x6) => {
                    // 8XY6 - Shift right
                    // ips (in place shifting) means x should be shifted.
                    // without it, y will be moved into x before being shifted
                    if !self.ips {
                        self.reg[x] = self.reg[y];
                    }

                    self.reg[0xF] = self.reg[x] & 0b1;
                    self.reg[x] >>= 1;
                }
                (0x8, _, _, 0x7) => {
                    // 8XY7 - Subtract X from Y
                    // set carry flag if overflowed
                    self.reg[0xF] = if self.reg[x] > self.reg[y] { 1 } else { 0 };
                    self.reg[y] = self.reg[y].wrapping_sub(self.reg[x]);
                }
                (0x8, _, _, 0xE) => {
                    // 8XYE - Shift left
                    // ips (in place shifting) means x should be shifted.
                    // without it, y will be moved into x before being shifted
                    if !self.ips {
                        self.reg[x] = self.reg[y];
                    }

                    self.reg[0xF] = self.reg[x] & 0b1000_0000;
                    self.reg[x] <<= 1;
                }
                (0x9, _, _, 0x0) => {
                    // 9XY0 - Skip if not equal
                    if self.reg[x] != self.reg[y] {
                        self.counter += 2;
                    }
                }
                (0xB, ..) => {
                    // BNNN - Jump with offset
                    // this jumps to V0 offset by NNN bytes
                    self.counter = self.reg[0] as usize + nnn;
                }
                (0xC, ..) => {
                    // CXNN - Random
                    // a random u8 is generated and ANDed together with nn, then put in VX
                    self.reg[x] = nn & rng.gen::<u8>();
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
                (0xE, _, 0x9, 0xE) => {
                    // EX9E - Skip if key pressed
                    if self.scan_key(self.reg[x]) {
                        self.counter += 2;
                    }
                }
                (0xE, _, 0xA, 0x1) => {
                    // EXA1 - Skip if key not pressed
                    if !self.scan_key(self.reg[x]) {
                        self.counter += 2;
                    }
                }
                (0xF, _, 0x0, 0x7) => {
                    // FX07 - Set VX to delay timer
                    self.reg[x] = self.timer;
                }
                (0xF, _, 0x0, 0xA) => {
                    // FX0A - Get key
                    // blocks until any key is pressed, then stores that key in VX
                    // this is done by just looping back to this same instruction
                    let scanned = self.scan_any();

                    match scanned {
                        Some(code) => self.reg[x] = code,
                        None => self.counter -= 2,
                    };
                }
                (0xF, _, 0x1, 0x5) => {
                    // FX15 - Set delay timer to VX
                    self.timer = self.reg[x];
                }
                (0xF, _, 0x1, 0x8) => {
                    // FX18 - Set sound timer to VX
                    self.s_timer = self.reg[x];
                }
                (0xF, _, 0x1, 0xE) => {
                    // FX1E - Add to index
                    // adds VX to I, setting the carry flag if I leaves memory
                    self.index += self.reg[x] as usize;
                    self.reg[0xF] = if self.index > 0xFFF { 1 } else { 0 };
                }
                (0xF, _, 0x2, 0x9) => {
                    // FX29 - Font character
                    // sets I to the location of the character in the last nibble of VX
                    let to = self.reg[x] & 0x0F;

                    // multiply by 5 because each character contains 5 bytes
                    self.index = FONT_ADDR + to as usize * 5;
                }
                (0xF, _, 0x3, 0x3) => {
                    // FX33 - Binary coded decimal conversion
                    // stores the decimal representation of VX across I, I+1, and I+2
                    // one digit per byte
                    let val = self.reg[x];

                    // 123
                    // ones = 3
                    // tens = 2
                    // hundreds = 1
                    let hundreds = val / 100;
                    let tens = (val % 100) / 10;
                    let ones = val % 10;

                    self.set_mem(&[hundreds, tens, ones]);
                }
                (0xF, _, 0x5, 0x5) => {
                    // FX55 - Store memory
                    // stores V0 through VX in memory
                    let block = &self.reg[0..=x];
                    let mut moving: Vec<u8> = vec![0; x + 1];

                    moving.copy_from_slice(block);
                    self.set_mem(moving);
                }
                (0xF, _, 0x6, 0x5) => {
                    // FX65 - Load memory
                    // loads X bytes from memory into registers V0-VX
                    let moving = self.load_mem(x);

                    for i in 0..=x {
                        self.reg[i] = moving[i];
                    }
                }
                _ => {
                    continue;
                } // ignore unknown instructions
            }
        }
    }
}
