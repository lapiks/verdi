use std::collections::HashMap;

use glium::glutin;
use rlua::UserData;

pub struct Inputs {
    keys: HashMap<glutin::event::VirtualKeyCode, bool>
}

#[derive(Clone, Copy)]
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

        self.keys.insert(key, pressed);
    }

    pub fn get_key_down(&self, key: Key) -> bool {
        true
    }
}