use super::Emulator;
use minifb::Key;

impl Emulator {
    pub fn scan_key(&self, key: u8) -> bool {
        match key {
            0x0 => self.window.is_key_down(Key::Key1),
            0x1 => self.window.is_key_down(Key::Key2),
            0x2 => self.window.is_key_down(Key::Key3),
            0x3 => self.window.is_key_down(Key::Key4),
            0x4 => self.window.is_key_down(Key::Q),
            0x5 => self.window.is_key_down(Key::W),
            0x6 => self.window.is_key_down(Key::E),
            0x7 => self.window.is_key_down(Key::R),
            0x8 => self.window.is_key_down(Key::A),
            0x9 => self.window.is_key_down(Key::S),
            0xA => self.window.is_key_down(Key::D),
            0xB => self.window.is_key_down(Key::F),
            0xC => self.window.is_key_down(Key::Z),
            0xD => self.window.is_key_down(Key::X),
            0xE => self.window.is_key_down(Key::C),
            0xF => self.window.is_key_down(Key::V),
            _ => unreachable!(),
        }
    }
}
