use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput},
    window::WindowBuilder,
};

pub struct Window {
    pub internal_window: winit::window::Window,
    pub event_loop: EventLoop<()>,
}

impl Window {
    pub fn new() -> Window {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        Self { internal_window: window, event_loop: event_loop }
    }

    pub fn get_id(&self) -> winit::window::WindowId {
        self.internal_window.id()
    }

    pub fn run(event_loop: EventLoop<()>, window: Window) {
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.get_id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        });
    }
}