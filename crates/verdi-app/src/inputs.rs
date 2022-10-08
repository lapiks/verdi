use std::collections::HashMap;

use glium::glutin::{self, event::VirtualKeyCode};
use rlua::UserData;

pub struct Inputs {
    keys: HashMap<Key, bool>
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Key {
    Unknown,
    Z,
    Q,
    S,
    D,
}

impl UserData for Key {}

impl From<String> for Key {
    fn from(s: String) -> Self {
        if s == "z" { Key::Z }
        else if s == "Q" { Key::Q }
        else if s == "S" { Key::S }
        else if s == "D" { Key::D }
        else { Key::Unknown }
    }
}

impl From<VirtualKeyCode> for Key {
    fn from(k: VirtualKeyCode) -> Self {
        match k {
            VirtualKeyCode::Key1 => todo!(),
            VirtualKeyCode::Key2 => todo!(),
            VirtualKeyCode::Key3 => todo!(),
            VirtualKeyCode::Key4 => todo!(),
            VirtualKeyCode::Key5 => todo!(),
            VirtualKeyCode::Key6 => todo!(),
            VirtualKeyCode::Key7 => todo!(),
            VirtualKeyCode::Key8 => todo!(),
            VirtualKeyCode::Key9 => todo!(),
            VirtualKeyCode::Key0 => todo!(),
            VirtualKeyCode::A => todo!(),
            VirtualKeyCode::B => todo!(),
            VirtualKeyCode::C => todo!(),
            VirtualKeyCode::D => Key::D,
            VirtualKeyCode::E => todo!(),
            VirtualKeyCode::F => todo!(),
            VirtualKeyCode::G => todo!(),
            VirtualKeyCode::H => todo!(),
            VirtualKeyCode::I => todo!(),
            VirtualKeyCode::J => todo!(),
            VirtualKeyCode::K => todo!(),
            VirtualKeyCode::L => todo!(),
            VirtualKeyCode::M => todo!(),
            VirtualKeyCode::N => todo!(),
            VirtualKeyCode::O => todo!(),
            VirtualKeyCode::P => todo!(),
            VirtualKeyCode::Q => Key::Q,
            VirtualKeyCode::R => todo!(),
            VirtualKeyCode::S => Key::S,
            VirtualKeyCode::T => todo!(),
            VirtualKeyCode::U => todo!(),
            VirtualKeyCode::V => todo!(),
            VirtualKeyCode::W => todo!(),
            VirtualKeyCode::X => todo!(),
            VirtualKeyCode::Y => todo!(),
            VirtualKeyCode::Z => Key::Z,
            VirtualKeyCode::Escape => todo!(),
            VirtualKeyCode::F1 => todo!(),
            VirtualKeyCode::F2 => todo!(),
            VirtualKeyCode::F3 => todo!(),
            VirtualKeyCode::F4 => todo!(),
            VirtualKeyCode::F5 => todo!(),
            VirtualKeyCode::F6 => todo!(),
            VirtualKeyCode::F7 => todo!(),
            VirtualKeyCode::F8 => todo!(),
            VirtualKeyCode::F9 => todo!(),
            VirtualKeyCode::F10 => todo!(),
            VirtualKeyCode::F11 => todo!(),
            VirtualKeyCode::F12 => todo!(),
            VirtualKeyCode::F13 => todo!(),
            VirtualKeyCode::F14 => todo!(),
            VirtualKeyCode::F15 => todo!(),
            VirtualKeyCode::F16 => todo!(),
            VirtualKeyCode::F17 => todo!(),
            VirtualKeyCode::F18 => todo!(),
            VirtualKeyCode::F19 => todo!(),
            VirtualKeyCode::F20 => todo!(),
            VirtualKeyCode::F21 => todo!(),
            VirtualKeyCode::F22 => todo!(),
            VirtualKeyCode::F23 => todo!(),
            VirtualKeyCode::F24 => todo!(),
            VirtualKeyCode::Snapshot => todo!(),
            VirtualKeyCode::Scroll => todo!(),
            VirtualKeyCode::Pause => todo!(),
            VirtualKeyCode::Insert => todo!(),
            VirtualKeyCode::Home => todo!(),
            VirtualKeyCode::Delete => todo!(),
            VirtualKeyCode::End => todo!(),
            VirtualKeyCode::PageDown => todo!(),
            VirtualKeyCode::PageUp => todo!(),
            VirtualKeyCode::Left => todo!(),
            VirtualKeyCode::Up => todo!(),
            VirtualKeyCode::Right => todo!(),
            VirtualKeyCode::Down => todo!(),
            VirtualKeyCode::Back => todo!(),
            VirtualKeyCode::Return => todo!(),
            VirtualKeyCode::Space => todo!(),
            VirtualKeyCode::Compose => todo!(),
            VirtualKeyCode::Caret => todo!(),
            VirtualKeyCode::Numlock => todo!(),
            VirtualKeyCode::Numpad0 => todo!(),
            VirtualKeyCode::Numpad1 => todo!(),
            VirtualKeyCode::Numpad2 => todo!(),
            VirtualKeyCode::Numpad3 => todo!(),
            VirtualKeyCode::Numpad4 => todo!(),
            VirtualKeyCode::Numpad5 => todo!(),
            VirtualKeyCode::Numpad6 => todo!(),
            VirtualKeyCode::Numpad7 => todo!(),
            VirtualKeyCode::Numpad8 => todo!(),
            VirtualKeyCode::Numpad9 => todo!(),
            VirtualKeyCode::NumpadAdd => todo!(),
            VirtualKeyCode::NumpadDivide => todo!(),
            VirtualKeyCode::NumpadDecimal => todo!(),
            VirtualKeyCode::NumpadComma => todo!(),
            VirtualKeyCode::NumpadEnter => todo!(),
            VirtualKeyCode::NumpadEquals => todo!(),
            VirtualKeyCode::NumpadMultiply => todo!(),
            VirtualKeyCode::NumpadSubtract => todo!(),
            VirtualKeyCode::AbntC1 => todo!(),
            VirtualKeyCode::AbntC2 => todo!(),
            VirtualKeyCode::Apostrophe => todo!(),
            VirtualKeyCode::Apps => todo!(),
            VirtualKeyCode::Asterisk => todo!(),
            VirtualKeyCode::At => todo!(),
            VirtualKeyCode::Ax => todo!(),
            VirtualKeyCode::Backslash => todo!(),
            VirtualKeyCode::Calculator => todo!(),
            VirtualKeyCode::Capital => todo!(),
            VirtualKeyCode::Colon => todo!(),
            VirtualKeyCode::Comma => todo!(),
            VirtualKeyCode::Convert => todo!(),
            VirtualKeyCode::Equals => todo!(),
            VirtualKeyCode::Grave => todo!(),
            VirtualKeyCode::Kana => todo!(),
            VirtualKeyCode::Kanji => todo!(),
            VirtualKeyCode::LAlt => todo!(),
            VirtualKeyCode::LBracket => todo!(),
            VirtualKeyCode::LControl => todo!(),
            VirtualKeyCode::LShift => todo!(),
            VirtualKeyCode::LWin => todo!(),
            VirtualKeyCode::Mail => todo!(),
            VirtualKeyCode::MediaSelect => todo!(),
            VirtualKeyCode::MediaStop => todo!(),
            VirtualKeyCode::Minus => todo!(),
            VirtualKeyCode::Mute => todo!(),
            VirtualKeyCode::MyComputer => todo!(),
            VirtualKeyCode::NavigateForward => todo!(),
            VirtualKeyCode::NavigateBackward => todo!(),
            VirtualKeyCode::NextTrack => todo!(),
            VirtualKeyCode::NoConvert => todo!(),
            VirtualKeyCode::OEM102 => todo!(),
            VirtualKeyCode::Period => todo!(),
            VirtualKeyCode::PlayPause => todo!(),
            VirtualKeyCode::Plus => todo!(),
            VirtualKeyCode::Power => todo!(),
            VirtualKeyCode::PrevTrack => todo!(),
            VirtualKeyCode::RAlt => todo!(),
            VirtualKeyCode::RBracket => todo!(),
            VirtualKeyCode::RControl => todo!(),
            VirtualKeyCode::RShift => todo!(),
            VirtualKeyCode::RWin => todo!(),
            VirtualKeyCode::Semicolon => todo!(),
            VirtualKeyCode::Slash => todo!(),
            VirtualKeyCode::Sleep => todo!(),
            VirtualKeyCode::Stop => todo!(),
            VirtualKeyCode::Sysrq => todo!(),
            VirtualKeyCode::Tab => todo!(),
            VirtualKeyCode::Underline => todo!(),
            VirtualKeyCode::Unlabeled => todo!(),
            VirtualKeyCode::VolumeDown => todo!(),
            VirtualKeyCode::VolumeUp => todo!(),
            VirtualKeyCode::Wake => todo!(),
            VirtualKeyCode::WebBack => todo!(),
            VirtualKeyCode::WebFavorites => todo!(),
            VirtualKeyCode::WebForward => todo!(),
            VirtualKeyCode::WebHome => todo!(),
            VirtualKeyCode::WebRefresh => todo!(),
            VirtualKeyCode::WebSearch => todo!(),
            VirtualKeyCode::WebStop => todo!(),
            VirtualKeyCode::Yen => todo!(),
            VirtualKeyCode::Copy => todo!(),
            VirtualKeyCode::Paste => todo!(),
            VirtualKeyCode::Cut => todo!(),
        }
    }
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            keys: HashMap::default(),
        }
    } 

    pub fn process(&mut self, event: &glutin::event::WindowEvent) {
        let input = match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };
        let pressed = input.state == glutin::event::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };

        self.keys.insert(Key::from(key), pressed);
    }

    pub fn get_key_down(&mut self, key: Key) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }
}