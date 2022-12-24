mod emulator;

use std::env;

use emulator::{EmuDisplay, Emulator};

fn main() {
    let path = &env::args().collect::<Vec<String>>()[1];
    let script = std::fs::read(path).unwrap();

    let display = EmuDisplay::new("chip8");
    let mut emu = Emulator::new(display, 1428, true);

    emu.run_script(script);
}
