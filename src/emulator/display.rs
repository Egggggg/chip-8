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

    pub fn draw(&mut self, sprite: &[u8], coords: (u8, u8)) {
        // a type for holding temporary "Scratch" data as bits to be iterated through
        type Scratch = BitArr!(for 4, in u8, Msb0);

        let mut scratch: Scratch = bitarr!(u8, Msb0; 0; 4);

        // weird way to get dimensions for this display
        let dim = match self {
            Self::Chip8(_) => (64, 32),
            Self::SuperChip(_) => (128, 64),
        };

        let x = coords.0 % dim.0;
        let y = coords.1 % dim.1;

        for byte in sprite {
            // sprite bytes are always 0xX0
            // we can shift it over to 0x0X to save space in scratch
            let byte = byte >> 4;

            scratch.store_be(byte);

            for bit in scratch {}
        }
    }
}
