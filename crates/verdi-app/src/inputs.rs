use std::collections::HashMap;

use glium::glutin::{
    self, 
    event::{
        VirtualKeyCode, 
        MouseButton as GlutinMouseButton
    }
};
use rlua::UserData;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Key {
    Unknown,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Space,
}

impl UserData for Key {}

impl From<String> for Key {
    fn from(s: String) -> Self {
        if s == "1" { Key::Key1 }
        else if s == "2" { Key::Key2 }
        else if s == "3" { Key::Key3 }
        else if s == "4" { Key::Key4 }
        else if s == "5" { Key::Key5 }
        else if s == "6" { Key::Key6 }
        else if s == "7" { Key::Key7 }
        else if s == "8" { Key::Key8 }
        else if s == "9" { Key::Key9 }
        else if s == "0" { Key::Key0 }
        else if s == "a" { Key::A }
        else if s == "b" { Key::B }
        else if s == "c" { Key::C }
        else if s == "d" { Key::D }
        else if s == "e" { Key::E }
        else if s == "f" { Key::F }
        else if s == "g" { Key::G }
        else if s == "h" { Key::H }
        else if s == "i" { Key::I }
        else if s == "j" { Key::J }
        else if s == "k" { Key::K }
        else if s == "l" { Key::L }
        else if s == "m" { Key::M }
        else if s == "n" { Key::N }
        else if s == "o" { Key::O }
        else if s == "p" { Key::P }
        else if s == "q" { Key::Q }
        else if s == "r" { Key::R }
        else if s == "s" { Key::S }
        else if s == "t" { Key::T }
        else if s == "u" { Key::U }
        else if s == "v" { Key::V }
        else if s == "w" { Key::W }
        else if s == "x" { Key::X }
        else if s == "y" { Key::Y }
        else if s == "z" { Key::Z }
        else if s == " " { Key::Space}
        else { Key::Unknown }
    }
}

impl From<VirtualKeyCode> for Key {
    fn from(k: VirtualKeyCode) -> Self {
        match k {
            VirtualKeyCode::Key1 => Key::Key1,
            VirtualKeyCode::Key2 => Key::Key2,
            VirtualKeyCode::Key3 => Key::Key3,
            VirtualKeyCode::Key4 => Key::Key4,
            VirtualKeyCode::Key5 => Key::Key5,
            VirtualKeyCode::Key6 => Key::Key6,
            VirtualKeyCode::Key7 => Key::Key7,
            VirtualKeyCode::Key8 => Key::Key8,
            VirtualKeyCode::Key9 => Key::Key9,
            VirtualKeyCode::Key0 => Key::Key0,
            VirtualKeyCode::A => Key::A,
            VirtualKeyCode::B => Key::B,
            VirtualKeyCode::C => Key::C,
            VirtualKeyCode::D => Key::D,
            VirtualKeyCode::E => Key::E,
            VirtualKeyCode::F => Key::F,
            VirtualKeyCode::G => Key::G,
            VirtualKeyCode::H => Key::H,
            VirtualKeyCode::I => Key::I,
            VirtualKeyCode::J => Key::J,
            VirtualKeyCode::K => Key::K,
            VirtualKeyCode::L => Key::L,
            VirtualKeyCode::M => Key::M,
            VirtualKeyCode::N => Key::N,
            VirtualKeyCode::O => Key::O,
            VirtualKeyCode::P => Key::P,
            VirtualKeyCode::Q => Key::Q,
            VirtualKeyCode::R => Key::R,
            VirtualKeyCode::S => Key::S,
            VirtualKeyCode::T => Key::T,
            VirtualKeyCode::U => Key::U,
            VirtualKeyCode::V => Key::V,
            VirtualKeyCode::W => Key::W,
            VirtualKeyCode::X => Key::X,
            VirtualKeyCode::Y => Key::Y,
            VirtualKeyCode::Z => Key::Z,
            VirtualKeyCode::Escape => Key::D,
            VirtualKeyCode::F1 => Key::D,
            VirtualKeyCode::F2 => Key::D,
            VirtualKeyCode::F3 => Key::D,
            VirtualKeyCode::F4 => Key::D,
            VirtualKeyCode::F5 => Key::D,
            VirtualKeyCode::F6 => Key::D,
            VirtualKeyCode::F7 => Key::D,
            VirtualKeyCode::F8 => Key::D,
            VirtualKeyCode::F9 => Key::D,
            VirtualKeyCode::F10 => Key::D,
            VirtualKeyCode::F11 => Key::D,
            VirtualKeyCode::F12 => Key::D,
            VirtualKeyCode::F13 => Key::D,
            VirtualKeyCode::F14 => Key::D,
            VirtualKeyCode::F15 => Key::D,
            VirtualKeyCode::F16 => Key::D,
            VirtualKeyCode::F17 => Key::D,
            VirtualKeyCode::F18 => Key::D,
            VirtualKeyCode::F19 => Key::D,
            VirtualKeyCode::F20 => Key::D,
            VirtualKeyCode::F21 => Key::D,
            VirtualKeyCode::F22 => Key::D,
            VirtualKeyCode::F23 => Key::D,
            VirtualKeyCode::F24 => Key::D,
            VirtualKeyCode::Snapshot => Key::D,
            VirtualKeyCode::Scroll => Key::D,
            VirtualKeyCode::Pause => Key::D,
            VirtualKeyCode::Insert => Key::D,
            VirtualKeyCode::Home => Key::D,
            VirtualKeyCode::Delete => Key::D,
            VirtualKeyCode::End => Key::D,
            VirtualKeyCode::PageDown => Key::D,
            VirtualKeyCode::PageUp => Key::D,
            VirtualKeyCode::Left => Key::D,
            VirtualKeyCode::Up => Key::D,
            VirtualKeyCode::Right => Key::D,
            VirtualKeyCode::Down => Key::D,
            VirtualKeyCode::Back => Key::D,
            VirtualKeyCode::Return => Key::D,
            VirtualKeyCode::Space => Key::Space,
            VirtualKeyCode::Compose => Key::D,
            VirtualKeyCode::Caret => Key::D,
            VirtualKeyCode::Numlock => Key::D,
            VirtualKeyCode::Numpad0 => Key::D,
            VirtualKeyCode::Numpad1 => Key::D,
            VirtualKeyCode::Numpad2 => Key::D,
            VirtualKeyCode::Numpad3 => Key::D,
            VirtualKeyCode::Numpad4 => Key::D,
            VirtualKeyCode::Numpad5 => Key::D,
            VirtualKeyCode::Numpad6 => Key::D,
            VirtualKeyCode::Numpad7 => Key::D,
            VirtualKeyCode::Numpad8 => Key::D,
            VirtualKeyCode::Numpad9 => Key::D,
            VirtualKeyCode::NumpadAdd => Key::D,
            VirtualKeyCode::NumpadDivide => Key::D,
            VirtualKeyCode::NumpadDecimal => Key::D,
            VirtualKeyCode::NumpadComma => Key::D,
            VirtualKeyCode::NumpadEnter => Key::D,
            VirtualKeyCode::NumpadEquals => Key::D,
            VirtualKeyCode::NumpadMultiply => Key::D,
            VirtualKeyCode::NumpadSubtract => Key::D,
            VirtualKeyCode::AbntC1 => Key::D,
            VirtualKeyCode::AbntC2 => Key::D,
            VirtualKeyCode::Apostrophe => Key::D,
            VirtualKeyCode::Apps => Key::D,
            VirtualKeyCode::Asterisk => Key::D,
            VirtualKeyCode::At => Key::D,
            VirtualKeyCode::Ax => Key::D,
            VirtualKeyCode::Backslash => Key::D,
            VirtualKeyCode::Calculator => Key::D,
            VirtualKeyCode::Capital => Key::D,
            VirtualKeyCode::Colon => Key::D,
            VirtualKeyCode::Comma => Key::D,
            VirtualKeyCode::Convert => Key::D,
            VirtualKeyCode::Equals => Key::D,
            VirtualKeyCode::Grave => Key::D,
            VirtualKeyCode::Kana => Key::D,
            VirtualKeyCode::Kanji => Key::D,
            VirtualKeyCode::LAlt => Key::D,
            VirtualKeyCode::LBracket => Key::D,
            VirtualKeyCode::LControl => Key::D,
            VirtualKeyCode::LShift => Key::D,
            VirtualKeyCode::LWin => Key::D,
            VirtualKeyCode::Mail => Key::D,
            VirtualKeyCode::MediaSelect => Key::D,
            VirtualKeyCode::MediaStop => Key::D,
            VirtualKeyCode::Minus => Key::D,
            VirtualKeyCode::Mute => Key::D,
            VirtualKeyCode::MyComputer => Key::D,
            VirtualKeyCode::NavigateForward => Key::D,
            VirtualKeyCode::NavigateBackward => Key::D,
            VirtualKeyCode::NextTrack => Key::D,
            VirtualKeyCode::NoConvert => Key::D,
            VirtualKeyCode::OEM102 => Key::D,
            VirtualKeyCode::Period => Key::D,
            VirtualKeyCode::PlayPause => Key::D,
            VirtualKeyCode::Plus => Key::D,
            VirtualKeyCode::Power => Key::D,
            VirtualKeyCode::PrevTrack => Key::D,
            VirtualKeyCode::RAlt => Key::D,
            VirtualKeyCode::RBracket => Key::D,
            VirtualKeyCode::RControl => Key::D,
            VirtualKeyCode::RShift => Key::D,
            VirtualKeyCode::RWin => Key::D,
            VirtualKeyCode::Semicolon => Key::D,
            VirtualKeyCode::Slash => Key::D,
            VirtualKeyCode::Sleep => Key::D,
            VirtualKeyCode::Stop => Key::D,
            VirtualKeyCode::Sysrq => Key::D,
            VirtualKeyCode::Tab => Key::D,
            VirtualKeyCode::Underline => Key::D,
            VirtualKeyCode::Unlabeled => Key::D,
            VirtualKeyCode::VolumeDown => Key::D,
            VirtualKeyCode::VolumeUp => Key::D,
            VirtualKeyCode::Wake => Key::D,
            VirtualKeyCode::WebBack => Key::D,
            VirtualKeyCode::WebFavorites => Key::D,
            VirtualKeyCode::WebForward => Key::D,
            VirtualKeyCode::WebHome => Key::D,
            VirtualKeyCode::WebRefresh => Key::D,
            VirtualKeyCode::WebSearch => Key::D,
            VirtualKeyCode::WebStop => Key::D,
            VirtualKeyCode::Yen => Key::D,
            VirtualKeyCode::Copy => Key::D,
            VirtualKeyCode::Paste => Key::D,
            VirtualKeyCode::Cut => Key::D,
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum MouseButton {
    Unknown,
    Right,
    Left,
    Middle,
}

impl From<String> for MouseButton {
    fn from(s: String) -> Self {
        if s == "r" { MouseButton::Right }
        else if s == "l" { MouseButton::Left }
        else if s == "m" { MouseButton::Middle }
        else { MouseButton::Unknown }
    }
}

impl From<GlutinMouseButton> for MouseButton {
    fn from(b: GlutinMouseButton) -> Self {
        match b {
            GlutinMouseButton::Left => MouseButton::Left,
            GlutinMouseButton::Right => MouseButton::Right,
            GlutinMouseButton::Middle => MouseButton::Middle,
            GlutinMouseButton::Other(_) => MouseButton::Unknown,
        }
    }
}

pub struct Inputs {
    keys: HashMap<Key, bool>,
    mouse: HashMap<MouseButton, bool>,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            keys: HashMap::default(),
            mouse: HashMap::default(),
        }
    } 

    pub fn process(&mut self, event: &glutin::event::WindowEvent) {
        match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                let pressed = input.state == glutin::event::ElementState::Pressed;
                let key = match input.virtual_keycode {
                    Some(key) => key,
                    None => return,
                };
   
                self.keys.insert(Key::from(key), pressed);
            },
            glutin::event::WindowEvent::MouseInput { button, state, .. } => {
                let pressed = state == glutin::event::ElementState::Pressed;
                self.mouse.insert(MouseButton::from(button), pressed);
            }
            _ => return,
        };
    }

    pub fn get_key_down(&mut self, key: Key) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }

    pub fn get_button_down(&mut self, button: MouseButton) -> bool {
        *self.mouse.get(&button).unwrap_or(&false)
    }
}