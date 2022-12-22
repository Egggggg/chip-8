mod emulator;

use emulator::model::{EmuDisplay, Emulator};

fn main() {
    let display = EmuDisplay::new("chip8");
    let mut emu = Emulator::new(display, 142);

    emu.main_loop()
}
