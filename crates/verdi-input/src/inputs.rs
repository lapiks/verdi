use std::collections::HashMap;

use egui_glium::egui_winit::egui::Modifiers as EguiModifiers;
use glium::glutin::{
    self, 
    event::{
        VirtualKeyCode, 
        MouseButton as GlutinMouseButton
    }
};
use mlua::UserData;
use verdi_math::Vec2;

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
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
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
        else if s == "escape" { Key::Escape }
        else if s == "f1" { Key::F1 }
        else if s == "f2" { Key::F2 }
        else if s == "f3" { Key::F3 }
        else if s == "f4" { Key::F4 }
        else if s == "f5" { Key::F5 }
        else if s == "f6" { Key::F6 }
        else if s == "f7" { Key::F7 }
        else if s == "f8" { Key::F8 }
        else if s == "f9" { Key::F9 }
        else if s == "f10" { Key::F10 }
        else if s == "f11" { Key::F11 }
        else if s == "f12" { Key::F12 }
        else if s == "f13" { Key::F13 }
        else if s == "f14" { Key::F14 }
        else if s == "f15" { Key::F15 }
        else if s == "f16" { Key::F16 }
        else if s == "f17" { Key::F17 }
        else if s == "f18" { Key::F18 }
        else if s == "f19" { Key::F19 }
        else if s == "f20" { Key::F20 }
        else if s == "f21" { Key::F21 }
        else if s == "f22" { Key::F22 }
        else if s == "f23" { Key::F23 }
        else if s == "f24" { Key::F24 }
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
            VirtualKeyCode::Escape => Key::Escape,
            VirtualKeyCode::F1 => Key::F1,
            VirtualKeyCode::F2 => Key::F2,
            VirtualKeyCode::F3 => Key::F3,
            VirtualKeyCode::F4 => Key::F4,
            VirtualKeyCode::F5 => Key::F5,
            VirtualKeyCode::F6 => Key::F6,
            VirtualKeyCode::F7 => Key::F7,
            VirtualKeyCode::F8 => Key::F8,
            VirtualKeyCode::F9 => Key::F9,
            VirtualKeyCode::F10 => Key::F10,
            VirtualKeyCode::F11 => Key::F11,
            VirtualKeyCode::F12 => Key::F12,
            VirtualKeyCode::F13 => Key::F13,
            VirtualKeyCode::F14 => Key::F14,
            VirtualKeyCode::F15 => Key::F15,
            VirtualKeyCode::F16 => Key::F16,
            VirtualKeyCode::F17 => Key::F17,
            VirtualKeyCode::F18 => Key::F18,
            VirtualKeyCode::F19 => Key::F19,
            VirtualKeyCode::F20 => Key::F20,
            VirtualKeyCode::F21 => Key::F21,
            VirtualKeyCode::F22 => Key::F22,
            VirtualKeyCode::F23 => Key::F23,
            VirtualKeyCode::F24 => Key::F24,
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

#[derive(Clone, Copy, Hash, Eq, PartialEq, Default)]
pub struct Modifiers {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
}

impl Modifiers {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<EguiModifiers> for Modifiers {
    fn from(b: EguiModifiers) -> Self {
        Self {
            alt: b.alt,
            ctrl: b.ctrl,
            shift: b.shift,
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
    mouse: HashMap<MouseButton, bool>,
    keys: HashMap<Key, bool>,
    modifiers: Modifiers,
    mouse_delta: Vec2,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            mouse: HashMap::default(),
            keys: HashMap::default(),
            modifiers: Modifiers::default(),
            mouse_delta: Vec2::ZERO,
        }
    } 

    pub fn reset(&mut self) {
        self.mouse_delta = Vec2::ZERO;
    }

    pub fn process_win_events(&mut self, event: &glutin::event::WindowEvent) {
        match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                let pressed = input.state == glutin::event::ElementState::Pressed;
                let key = match input.virtual_keycode {
                    Some(key) => key,
                    None => return,
                };
   
                self.keys.insert(Key::from(key), pressed);
            },
            glutin::event::WindowEvent::ModifiersChanged(modifiers_state) => {
                // todo: à revoir ?
                self.modifiers.alt = modifiers_state.alt();
                self.modifiers.shift = modifiers_state.shift();
                self.modifiers.ctrl = modifiers_state.ctrl();
            },
            glutin::event::WindowEvent::MouseInput { button, state, .. } => {
                let pressed = state == glutin::event::ElementState::Pressed;
                self.mouse.insert(MouseButton::from(button), pressed);
            },
            _ => return,
        };
    }

    pub fn process_device_events(&mut self, event: &glutin::event::DeviceEvent) {
        match *event {
            glutin::event::DeviceEvent::MouseMotion { delta, .. } => {
                self.mouse_delta += Vec2::new(delta.0 as f32, delta.1 as f32);
            },
            // glutin::event::DeviceEvent::Motion { axis, value } => {
            //     if axis < 2 {
            //         self.mouse_delta[axis as usize] += value as f32;
            //     }
            // },
            _ => return,
        };
    }

    pub fn get_key_down(&self, key: Key) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }

    pub fn get_modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn get_button_down(&self, button: MouseButton) -> bool {
        *self.mouse.get(&button).unwrap_or(&false)
    }

    pub fn get_mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }
}