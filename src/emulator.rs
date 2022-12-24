mod display;
mod input;
mod memory;
mod runner;

use std::collections::VecDeque;

pub use display::EmuDisplay;

use memory::SCRIPT_ADDR;
use minifb::{Window, WindowOptions};

/// The main emulator which contains all components and runs logic
pub struct Emulator {
    pub memory: [u8; 4096],     // 4096 bytes of ram
    pub display: EmuDisplay,    // display data will be adapted from here
    pub window: Window,         // window object for simulating the screen
    pub index: usize,           // index register, used to access memory
    pub counter: usize, // program counter, the current place in memory that is being executed
    pub stack: VecDeque<usize>, // used for returning from subroutines
    pub timer: u8,      // delay timer, decremented at 60hz with display drawing
    pub s_timer: u8,    // sound timer, beeps at nonzero values
    pub reg: [u8; 16],  // general purpose registers
    pub tick_us: u16,   // microseconds per tick (1428 for 700tps)
    pub ips: bool,      // whether to shift in place for 8XY6 and 8XYE
}

impl Emulator {
    pub fn new(display: EmuDisplay, tick_us: u16, ips: bool) -> Self {
        let dim = match display {
            EmuDisplay::Chip8(_) => (64, 32),
            EmuDisplay::SuperChip(_) => (128, 64),
        };

        let mut window = Window::new(
            "Bee Chip-8 :)",
            dim.0 * 4,
            dim.1 * 4,
            WindowOptions::default(),
        )
        .unwrap();

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Emulator {
            memory: [0; 4096],
            display,
            window,
            index: 0,
            counter: SCRIPT_ADDR,
            stack: VecDeque::new(),
            timer: 255,
            s_timer: 255,
            reg: [0; 16],
            tick_us,
            ips,
        }
    }

    pub fn run_script(&mut self, script: impl AsRef<[u8]>) {
        let script = script.as_ref();

        self.load_font();
        self.load_script(script);
        self.main_loop();
    }
}
