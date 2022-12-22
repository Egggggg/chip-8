use std::collections::VecDeque;

use bitvec::prelude::*;

use super::loader::SCRIPT_ADDR;

// creates the type to be used for the display
// a bitvec::BitArray, because the display is monochrome
// this is a flat array, we need to make sure to account for that later
type Chip8Display = BitArr!(for 64 * 32, in u64, Msb0);
type SuperChipDisplay = BitArr!(for 128 * 64, in u64, Msb0);

/// Possible display types (sizes)
pub enum EmuDisplay {
    Chip8(Chip8Display),
    SuperChip(SuperChipDisplay),
}

impl EmuDisplay {
    pub fn new(kind: &str) -> Self {
        if kind == "chip8" {
            let out: Chip8Display = bitarr!(u64, Msb0; 0; 64 * 32);
            Self::Chip8(out)
        } else {
            let out: SuperChipDisplay = bitarr!(u64, Msb0; 0; 128 * 64);
            Self::SuperChip(out)
        }
    }
}

/// The main emulator, contains all components, and has methods for interfacing with them through itself
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
