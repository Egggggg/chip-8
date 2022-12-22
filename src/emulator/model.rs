use std::collections::VecDeque;

pub use super::display::EmuDisplay;
use super::loader::SCRIPT_ADDR;

/// The main emulator which contains all components and runs logic
pub struct Emulator {
    pub memory: [u8; 4096],     // 4096 bytes of ram
    pub display: EmuDisplay,    // display data will be adapted from here
    pub counter: usize, // program counter, the current place in memory that is being executed
    pub stack: VecDeque<usize>, // used for returning from subroutines
    pub timer: u8,      // delay timer, decremented at 60hz with display drawing
    pub s_timer: u8,    // sound timer, beeps at nonzero values
    pub reg: [u8; 16],  // general purpose registers
    pub tick_us: u16,   // microseconds per tick (142 for 700tps)
}

impl Emulator {
    pub fn new(display: EmuDisplay, tick_us: u16) -> Self {
        Emulator {
            memory: [0; 4096],
            display,
            counter: SCRIPT_ADDR,
            stack: VecDeque::new(),
            timer: 255,
            s_timer: 255,
            reg: [0; 16],
            tick_us,
        }
    }
}
