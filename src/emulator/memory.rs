use super::Emulator;

// the address to store the font at
pub const FONT_ADDR: usize = 0x50;

// the font is 80 bytes (5 bytes by 16 chars)
const FONT_SIZE: usize = 80;

const FONT: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// the address to store scripts at
pub const SCRIPT_ADDR: usize = 0x200;

impl Emulator {
    /// Loads the font into memory
    ///
    /// # Arguments
    ///
    /// * `memory` - Simulated memory to load the font into
    pub fn load_font(&mut self) {
        for i in 0..FONT_SIZE {
            self.memory[FONT_ADDR + i] = FONT[i];
        }
    }

    /// Loads a script into memory
    ///
    /// # Arguments
    ///
    /// * `memory` - Simulated memory to load the script into
    /// * `script` - Script to load into memory and then execute
    pub fn load_script(&mut self, script: &[u8]) {
        for i in 0..script.len() {
            self.memory[SCRIPT_ADDR + i] = script[i];
        }
    }

    pub fn set_mem(&mut self, block: impl AsRef<[u8]>) {
        let block = block.as_ref();

        for i in 0..block.len() {
            self.memory[self.index + i] = block[i];
        }
    }

    pub fn load_mem(&self, len: usize) -> Vec<u8> {
        let mut out = Vec::new();

        for i in 0..=len {
            out.push(self.memory[self.index + i]);
        }

        out
    }
}
