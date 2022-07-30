use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput},
    window::WindowBuilder,
};

pub fn create(event_loop: &EventLoop<()>) -> winit::window::Window {
    WindowBuilder::new().build(event_loop).unwrap()
}
