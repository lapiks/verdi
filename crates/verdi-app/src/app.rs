use glium::{glutin, Surface};
use rlua::Lua;
use std::{sync::Mutex};

use verdi_window::prelude::*;
use verdi_graphics::prelude::*;
use verdi_gui::prelude::*;

use crate::{
    error::AppError, 
    lua_context::LuaContext, 
    inputs::Inputs, 
    bind_inputs::BindInputs
};

pub struct App;

impl App {
    pub fn run(gpu: &'static Mutex<GraphicsChip>, inputs: &'static Mutex<Inputs>) -> Result<(), AppError> {
        let mut window = Window::new(1024, 768);
        
        let mut renderer = Renderer::new();
    
        let lua = Lua::new();
    
        BindGraphicsChip::bind(&lua, gpu)?;
        BindInputs::bind(&lua, inputs)?;

        LuaContext::load_scipts(&lua)?;
        LuaContext::call_boot(&lua)?;

        let event_loop = window.take_event_loop().expect("No event loop in the window");

        let egui_glium = egui_glium::EguiGlium::new(&window.get_display(), &event_loop);
        let mut gui = Gui::new(egui_glium);

        let mut last_error: String = String::new();
    
        event_loop.run(move |ev, _, control_flow| {
            if let Err(err) = LuaContext::call_run(&lua) {
                let current_error = err.to_string();
                if last_error != current_error {
                    println!("{}", err);
                    last_error = current_error;
                }
            }
            
            // request a new frame
            let mut target = window.get_display().draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            // prepare assets for rendering
            renderer.prepare_assets(window.get_display(), &gpu.lock().unwrap());
            
            // draw game
            renderer.render(&mut target, &mut gpu.lock().unwrap());

            // draw GUI
            gui.run(window.get_display());
            gui.render(window.get_display(), &mut target);

            // ends frame
            target.finish().unwrap();
            
            gpu.lock().unwrap().pipeline.render_passes.clear();
    
            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
    
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            // events handling
            match ev {
                glutin::event::Event::WindowEvent { event, .. } =>  {
                    use glutin::event::WindowEvent;
                    if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }

                    // relays events to the gui
                    if gui.on_event(&event) {
                        
                    }
                    else {
                        inputs.lock().unwrap().process(&event)
                    }
                },
                _ => (),
            }
        });
    }
}
