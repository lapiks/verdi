use std::collections::HashMap;

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
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Semicolon,
    Equal,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Enter,
    Tab,
    Backspace,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
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

impl From<miniquad::KeyCode> for Key {
    fn from(k: miniquad::KeyCode) -> Self {
        match k {
            miniquad::KeyCode::Unknown => Key::Unknown,
            miniquad::KeyCode::Key1 => Key::Key1,
            miniquad::KeyCode::Key2 => Key::Key2,
            miniquad::KeyCode::Key3 => Key::Key3,
            miniquad::KeyCode::Key4 => Key::Key4,
            miniquad::KeyCode::Key5 => Key::Key5,
            miniquad::KeyCode::Key6 => Key::Key6,
            miniquad::KeyCode::Key7 => Key::Key7,
            miniquad::KeyCode::Key8 => Key::Key8,
            miniquad::KeyCode::Key9 => Key::Key9,
            miniquad::KeyCode::Key0 => Key::Key0,
            miniquad::KeyCode::A => Key::A,
            miniquad::KeyCode::B => Key::B,
            miniquad::KeyCode::C => Key::C,
            miniquad::KeyCode::D => Key::D,
            miniquad::KeyCode::E => Key::E,
            miniquad::KeyCode::F => Key::F,
            miniquad::KeyCode::G => Key::G,
            miniquad::KeyCode::H => Key::H,
            miniquad::KeyCode::I => Key::I,
            miniquad::KeyCode::J => Key::J,
            miniquad::KeyCode::K => Key::K,
            miniquad::KeyCode::L => Key::L,
            miniquad::KeyCode::M => Key::M,
            miniquad::KeyCode::N => Key::N,
            miniquad::KeyCode::O => Key::O,
            miniquad::KeyCode::P => Key::P,
            miniquad::KeyCode::Q => Key::Q,
            miniquad::KeyCode::R => Key::R,
            miniquad::KeyCode::S => Key::S,
            miniquad::KeyCode::T => Key::T,
            miniquad::KeyCode::U => Key::U,
            miniquad::KeyCode::V => Key::V,
            miniquad::KeyCode::W => Key::W,
            miniquad::KeyCode::X => Key::X,
            miniquad::KeyCode::Y => Key::Y,
            miniquad::KeyCode::Z => Key::Z,
            miniquad::KeyCode::Escape => Key::Escape,
            miniquad::KeyCode::F1 => Key::F1,
            miniquad::KeyCode::F2 => Key::F2,
            miniquad::KeyCode::F3 => Key::F3,
            miniquad::KeyCode::F4 => Key::F4,
            miniquad::KeyCode::F5 => Key::F5,
            miniquad::KeyCode::F6 => Key::F6,
            miniquad::KeyCode::F7 => Key::F7,
            miniquad::KeyCode::F8 => Key::F8,
            miniquad::KeyCode::F9 => Key::F9,
            miniquad::KeyCode::F10 => Key::F10,
            miniquad::KeyCode::F11 => Key::F11,
            miniquad::KeyCode::F12 => Key::F12,
            miniquad::KeyCode::F13 => Key::F13,
            miniquad::KeyCode::F14 => Key::F14,
            miniquad::KeyCode::F15 => Key::F15,
            miniquad::KeyCode::F16 => Key::F16,
            miniquad::KeyCode::F17 => Key::F17,
            miniquad::KeyCode::F18 => Key::F18,
            miniquad::KeyCode::F19 => Key::F19,
            miniquad::KeyCode::F20 => Key::F20,
            miniquad::KeyCode::F21 => Key::F21,
            miniquad::KeyCode::F22 => Key::F22,
            miniquad::KeyCode::F23 => Key::F23,
            miniquad::KeyCode::F24 => Key::F24,
            miniquad::KeyCode::Pause => Key::Pause,
            miniquad::KeyCode::Insert => Key::Insert,
            miniquad::KeyCode::Home => Key::Home,
            miniquad::KeyCode::Delete => Key::Delete,
            miniquad::KeyCode::End => Key::End,
            miniquad::KeyCode::PageDown => Key::PageDown,
            miniquad::KeyCode::PageUp => Key::PageUp,
            miniquad::KeyCode::Left => Key::Left,
            miniquad::KeyCode::Up => Key::Up,
            miniquad::KeyCode::Right => Key::Right,
            miniquad::KeyCode::Down => Key::Down,
            miniquad::KeyCode::Space => Key::Space,
            miniquad::KeyCode::Apostrophe => Key::Apostrophe,
            miniquad::KeyCode::Comma => Key::Comma,
            miniquad::KeyCode::Minus => Key::Minus,
            miniquad::KeyCode::Period => Key::Period,
            miniquad::KeyCode::Slash => Key::Slash,
            miniquad::KeyCode::Semicolon => Key::Semicolon,
            miniquad::KeyCode::Equal => Key::Equal,
            miniquad::KeyCode::LeftBracket => Key::LeftBracket,
            miniquad::KeyCode::Backslash => Key::Backslash,
            miniquad::KeyCode::RightBracket => Key::RightBracket,
            miniquad::KeyCode::GraveAccent => Key::GraveAccent,
            miniquad::KeyCode::World1 => Key::World1,
            miniquad::KeyCode::World2 => Key::World2,
            miniquad::KeyCode::Enter => Key::Enter,
            miniquad::KeyCode::Tab => Key::Tab,
            miniquad::KeyCode::Backspace => Key::Backspace,
            miniquad::KeyCode::CapsLock => Key::CapsLock,
            miniquad::KeyCode::ScrollLock => Key::ScrollLock,
            miniquad::KeyCode::NumLock => Key::NumLock,
            miniquad::KeyCode::PrintScreen => Key::PrintScreen,
            miniquad::KeyCode::F25 => Key::F25,
            miniquad::KeyCode::Kp0 => Key::Kp0,
            miniquad::KeyCode::Kp1 => Key::Kp1,
            miniquad::KeyCode::Kp2 => Key::Kp2,
            miniquad::KeyCode::Kp3 => Key::Kp3,
            miniquad::KeyCode::Kp4 => Key::Kp4,
            miniquad::KeyCode::Kp5 => Key::Kp5,
            miniquad::KeyCode::Kp6 => Key::Kp6,
            miniquad::KeyCode::Kp7 => Key::Kp7,
            miniquad::KeyCode::Kp8 => Key::Kp8,
            miniquad::KeyCode::Kp9 => Key::Kp9,
            miniquad::KeyCode::KpDecimal => Key::KpDecimal,
            miniquad::KeyCode::KpDivide => Key::KpDivide,
            miniquad::KeyCode::KpMultiply => Key::KpMultiply,
            miniquad::KeyCode::KpSubtract => Key::KpSubtract,
            miniquad::KeyCode::KpAdd => Key::KpAdd,
            miniquad::KeyCode::KpEnter => Key::KpEnter,
            miniquad::KeyCode::KpEqual => Key::KpEqual,
            miniquad::KeyCode::LeftShift => Key::LeftShift,
            miniquad::KeyCode::LeftControl => Key::LeftControl,
            miniquad::KeyCode::LeftAlt => Key::LeftAlt,
            miniquad::KeyCode::LeftSuper => Key::LeftSuper,
            miniquad::KeyCode::RightShift => Key::RightShift,
            miniquad::KeyCode::RightControl => Key::RightControl,
            miniquad::KeyCode::RightAlt => Key::RightAlt,
            miniquad::KeyCode::RightSuper => Key::RightSuper,
            miniquad::KeyCode::Menu => Key::Menu,
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

impl From<miniquad::KeyMods> for Modifiers {
    fn from(b: miniquad::KeyMods) -> Self {
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

impl From<miniquad::MouseButton> for MouseButton {
    fn from(b: miniquad::MouseButton) -> Self {
        match b {
            miniquad::MouseButton::Left => MouseButton::Left,
            miniquad::MouseButton::Right => MouseButton::Right,
            miniquad::MouseButton::Middle => MouseButton::Middle,
            miniquad::MouseButton::Unknown => MouseButton::Unknown,
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

    // pub fn process_win_events(&mut self, event: &glutin::event::WindowEvent) {
    //     match *event {
    //         glutin::event::WindowEvent::KeyboardInput { input, .. } => {
    //             let pressed = input.state == glutin::event::ElementState::Pressed;
    //             let key = match input.virtual_keycode {
    //                 Some(key) => key,
    //                 None => return,
    //             };
   
    //             self.keys.insert(Key::from(key), pressed);
    //         },
    //         glutin::event::WindowEvent::ModifiersChanged(modifiers_state) => {
    //             // todo: à revoir ?
    //             self.modifiers.alt = modifiers_state.alt();
    //             self.modifiers.shift = modifiers_state.shift();
    //             self.modifiers.ctrl = modifiers_state.ctrl();
    //         },
    //         glutin::event::WindowEvent::MouseInput { button, state, .. } => {
    //             let pressed = state == glutin::event::ElementState::Pressed;
    //             self.mouse.insert(MouseButton::from(button), pressed);
    //         },
    //         _ => return,
    //     };
    // }

    // pub fn process_device_events(&mut self, event: &glutin::event::DeviceEvent) {
    //     match *event {
    //         glutin::event::DeviceEvent::MouseMotion { delta, .. } => {
    //             self.mouse_delta += Vec2::new(delta.0 as f32, delta.1 as f32);
    //         },
    //         // glutin::event::DeviceEvent::Motion { axis, value } => {
    //         //     if axis < 2 {
    //         //         self.mouse_delta[axis as usize] += value as f32;
    //         //     }
    //         // },
    //         _ => return,
    //     };
    // }

    pub fn on_mouse_move(&mut self, x: f32, y: f32) {
        self.mouse_delta += Vec2::new(x, y);
    }

    pub fn on_mouse_wheel(&mut self, x: f32, y: f32) {

    }

    pub fn on_mouse_button_down(&mut self, button: MouseButton, x: f32, y: f32) {
        self.mouse.insert(MouseButton::from(button), true);
    }

    pub fn on_mouse_button_up(&mut self, button: MouseButton, x: f32, y: f32) {
        self.mouse.insert(MouseButton::from(button), false);
    }

    pub fn on_key_down(&mut self, keycode: Key, repeat: bool) {
        self.keys.insert(keycode, true);
    }

    pub fn on_key_up(&mut self, keycode: Key) {
        self.keys.insert(keycode, false);
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