use glium::{glutin};
use rlua::Lua;
use std::{sync::Mutex};

use verdi_window::prelude::*;
use verdi_graphics::prelude::*;

use crate::{error::AppError, lua_context::LuaContext};

pub struct App;

impl App {
    pub fn run(gpu: &'static Mutex<GraphicsChip>) -> Result<(), AppError> {
        let mut window = Window::new(1024, 768);
        
        let mut renderer = Renderer::new(&window).unwrap();
    
        let lua = Lua::new();
    
        BindGraphicsChip::bind(&lua, gpu)?;

        LuaContext::load_scipts(&lua)?;
        LuaContext::call_boot(&lua)?;

        let event_loop = window.take_event_loop().expect("No event loop in the window");

        let mut last_error: String = String::new();
    
        event_loop.run(move |ev, _, control_flow| {
            if let Err(err) = LuaContext::call_run(&lua) {
                let current_error = err.to_string();
                if last_error != current_error {
                    println!("{}", err);
                    last_error = current_error;
                }
            }
            
            renderer.render(&window, &gpu.lock().unwrap());
            gpu.lock().unwrap().render_passes.clear();
    
            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
    
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                _ => (),
            }
        });
    }
}
