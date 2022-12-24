use super::Emulator;
use bimap::BiMap;
use minifb::Key;

impl Emulator {
    fn get_keycodes() -> BiMap<u8, Key> {
        BiMap::from_iter([
            (0x0, Key::X),
            (0x1, Key::Key1),
            (0x2, Key::Key2),
            (0x3, Key::Key3),
            (0x4, Key::Q),
            (0x5, Key::W),
            (0x6, Key::E),
            (0x7, Key::A),
            (0x8, Key::S),
            (0x9, Key::D),
            (0xA, Key::Z),
            (0xB, Key::C),
            (0xC, Key::Key4),
            (0xD, Key::R),
            (0xE, Key::F),
            (0xF, Key::V),
        ])
    }

    pub fn scan_key(&self, key: u8) -> bool {
        let keycodes = Self::get_keycodes();
        let Some(key) = keycodes.get_by_left(&key) else { return false };

        self.window.is_key_down(*key)
    }

    pub fn scan_any(&self) -> Option<u8> {
        let keycodes = Self::get_keycodes();
        let pressed = self.window.get_keys();

        dbg!(self.scan_key(0x3));

        let key = keycodes.get_by_left(&0x3);

        dbg!(key);
        dbg!(&pressed);

        for key in pressed {
            dbg!(key);

            if let Some(code) = keycodes.get_by_right(&key) {
                return Some(*code);
            }
        }

        None
    }
}
