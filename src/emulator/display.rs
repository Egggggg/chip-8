use std::ops::BitXorAssign;

use bitvec::prelude::*;

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

    pub fn clear(&mut self) {
        match self {
            Self::Chip8(display) => {
                display.bitxor_assign(0);
            }
        }
    }
}
