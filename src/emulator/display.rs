use bitvec::prelude::*;

use super::Emulator;

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

    pub fn clear(&mut self) {
        match self {
            Self::Chip8(buf) => {
                *buf = bitarr!(u64, Msb0; 0; CHIP8_SIZE);
            }
            Self::SuperChip(buf) => {
                *buf = bitarr!(u64, Msb0; 0; SUPERCHIP_SIZE);
            }
        }
    }

    pub fn draw(&mut self, sprite: &[u8], coords: (u8, u8)) -> u8 {
        // a type for holding temporary "Scratch" data as bits to be iterated through
        type Scratch = BitArr!(for 4, in u8, Msb0);

        let mut scratch: Scratch = bitarr!(u8, Msb0; 0; 4);

        // weird way to get dimensions for this display
        let dim = match self {
            Self::Chip8(_) => (64, 32),
            Self::SuperChip(_) => (128, 64),
        };

        let x = coords.0 % dim.0;
        let mut y = coords.1 % dim.1;
        let mut carry = 0;

        for byte in sprite {
            let pos = x as usize + y as usize * dim.0 as usize;
            let offset = if x < dim.0 - 8 { 8 } else { dim.0 - x } as usize;

            // get the slice of memory from the
            let buf = match self {
                Self::Chip8(buf) => buf.get_mut(pos..pos + offset).unwrap(),
                Self::SuperChip(buf) => buf.get_mut(pos..pos + offset).unwrap(),
            };

            scratch.store_be(*byte);

            for i in 0..offset {
                let mut bit = buf.get_mut(i).unwrap();

                if scratch[i] && *bit {
                    *bit = false;
                    carry = 1;
                } else {
                    *bit ^= scratch[i];
                }
            }

            y += 1;

            if y >= dim.1 {
                break;
            }
        }

        carry
    }
}

impl Emulator {
    pub fn refresh_display(&mut self) {
        let full = 0x00_FF_FF_FF;
        let empty = 0;

        match self.display {
            EmuDisplay::Chip8(buf) => {
                let len = 64 * 32;
                let mut output: Vec<u32> = vec![0; len];

                for i in 0..len {
                    if buf[i] {
                        output[i] = full;
                    } else {
                        output[i] = empty;
                    }
                }

                self.window
                    .update_with_buffer(&output[0..], 64, 32)
                    .unwrap();
            }
            EmuDisplay::SuperChip(buf) => {
                let len = 128 * 64;
                let mut output: Vec<u32> = vec![0; len];

                for i in 0..len {
                    if buf[i] {
                        output[i] = full;
                    } else {
                        output[i] = empty;
                    }
                }

                self.window
                    .update_with_buffer(&output[0..], 128, 64)
                    .unwrap();
            }
        }
    }
}
