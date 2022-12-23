use std::ops::{BitXor, BitXorAssign};

use bitvec::prelude::*;

const CHIP8_SIZE: usize = 64 * 32;
const SUPERCHIP_SIZE: usize = 128 * 64;

// creates the type to be used for the display
// a bitvec::BitArray, because the display is monochrome
// this is a flat array, we need to make sure to account for that later
type Chip8Display = BitArr!(for CHIP8_SIZE, in u64, Msb0);
type SuperChipDisplay = BitArr!(for SUPERCHIP_SIZE, in u64, Msb0);

/// Possible display types (sizes)
pub enum EmuDisplay {
    Chip8(Chip8Display),
    SuperChip(SuperChipDisplay),
}

impl EmuDisplay {
    pub fn new(kind: &str) -> Self {
        if kind == "chip8" {
            let out: Chip8Display = bitarr!(u64, Msb0; 0; CHIP8_SIZE);
            Self::Chip8(out)
        } else {
            let out: SuperChipDisplay = bitarr!(u64, Msb0; 0; SUPERCHIP_SIZE);
            Self::SuperChip(out)
        }
    }

    fn refresh(&mut self) {}

    pub fn clear(&mut self) {
        match self {
            Self::Chip8(display) => {
                *display = bitarr!(u64, Msb0; 0; CHIP8_SIZE);
            }
            Self::SuperChip(display) => {
                *display = bitarr!(u64, Msb0; 0; SUPERCHIP_SIZE);
            }
        }

        self.refresh();
    }

    pub fn draw(&mut self, sprite: &[u8]) {}
}
