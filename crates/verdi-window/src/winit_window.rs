use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput},
    window::WindowBuilder,
};

pub fn create_and_run() -> winit::window::Window
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
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