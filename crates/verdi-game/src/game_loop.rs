use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput},
};

use futures::executor::block_on;

use verdi_window::Window;
use verdi_renderer::Renderer;
use verdi_graphics;

pub fn run() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop, 1024, 768);
    let mut renderer = block_on(Renderer::new(&window.internal_window));

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
                renderer.on_window_resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                renderer.on_window_resize(**new_inner_size);
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.internal_window.id() => {
            renderer.render();
        }
        Event::RedrawEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.internal_window.request_redraw();
        }
        _ => {}
    });
}

pub fn run2() {
    verdi_graphics::run();
}