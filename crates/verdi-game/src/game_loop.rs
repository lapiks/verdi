use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput},
};

use futures::executor::block_on;

use verdi_window::Window;
use verdi_renderer::Renderer;

pub fn run() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop, 1024, 768);
    let renderer = block_on(Renderer::new(&window.internal_window));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.internal_window.id() => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                // fire event
            }
            _ => {}
        },
        _ => {}
    });
}