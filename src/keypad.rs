use minifb::{Key, Window};

const POSSIBLE_KEYS: [Key; 16] = [
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Q,
    Key::W,
    Key::E,
    Key::R,
    Key::A,
    Key::S,
    Key::D,
    Key::F,
    Key::Z,
    Key::X,
    Key::C,
    Key::V,
];

fn key_to_hex(key: Key) -> u8 {
    match key {
        Key::X => 0x0,
        Key::Key1 => 0x1,
        Key::Key2 => 0x2,
        Key::Key3 => 0x3,
        Key::Q => 0x4,
        Key::W => 0x5,
        Key::E => 0x6,
        Key::A => 0x7,
        Key::S => 0x8,
        Key::D => 0x9,
        Key::Z => 0xA,
        Key::C => 0xB,
        Key::Key4 => 0xC,
        Key::R => 0xD,
        Key::F => 0xE,
        Key::V => 0xF,
        // Only possible key values that will be converted, so this will never happen
        _ => unreachable!(),
    }
}

fn hex_to_key(hex: u8) -> Option<Key> {
    Some(match hex {
        0x0 => Key::X,
        0x1 => Key::Key1,
        0x2 => Key::Key2,
        0x3 => Key::Key3,
        0x4 => Key::Q,
        0x5 => Key::W,
        0x6 => Key::E,
        0x7 => Key::A,
        0x8 => Key::S,
        0x9 => Key::D,
        0xA => Key::Z,
        0xB => Key::C,
        0xC => Key::Key4,
        0xD => Key::R,
        0xE => Key::F,
        0xF => Key::V,
        _ => {
            log::error!("invalid hex value for key");
            return None;
        }
    })
}

pub fn get_pressed_key(window: &Window) -> Option<u8> {
    let pressed_keys = window.get_keys();
    pressed_keys
        .into_iter()
        .find(|key| POSSIBLE_KEYS.contains(key))
        .map(key_to_hex)
}

pub fn is_key_pressed(window: &Window, key_hex: u8) -> Option<bool> {
    let key = hex_to_key(key_hex)?;
    Some(window.is_key_down(key))
}
