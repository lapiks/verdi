use glium::{
    glutin, 
    Surface,
};

use rlua::Lua;
use std::{sync::{Mutex, Arc}};

use verdi_window::prelude::*;
use verdi_graphics::prelude::*;
use verdi_gui::prelude::*;

use crate::{
    error::AppError, 
    lua_context::LuaContext, 
    inputs::Inputs, 
    bind_inputs::BindInputs, time_step::TimeStep
};

pub struct App;

impl App {
    pub fn run() -> Result<(), AppError> {
        let mut window = Window::new(1024, 768);

        let render_target = RenderTarget::new(
            window.get_display(),
            320, 
            240
        ).expect("Render target creation failed");
        
        let gpu = Arc::new(
            Mutex::new(
                GraphicsChip::new()
                    .expect("GraphicsChip initialisation failed")
            )
        );

        let inputs = Arc::new(
            Mutex::new(
                Inputs::new()
            )
        );

        let mut renderer = Renderer::new();
    
        let lua = Lua::new();
    
        BindGraphicsChip::bind(&lua, gpu.clone())?;
        BindInputs::bind(&lua, inputs.clone())?;

        LuaContext::load_scripts(&lua)?;
        LuaContext::call_boot(&lua)?;

        let event_loop = window.take_event_loop().expect("No event loop in the window");

        let egui_glium = egui_glium::EguiGlium::new(&window.get_display(), &event_loop);
        let mut gui = Gui::new(egui_glium);

        let mut last_error: String = String::new();
        let mut time_step = TimeStep::new();
    
        event_loop.run(move |ev, _, control_flow| {
            let delta_time = time_step.tick();

            if let Err(err) = LuaContext::call_run(&lua, delta_time) {
                let current_error = err.to_string();
                if last_error != current_error {
                    println!("{}", err);
                    last_error = current_error;
                }
            }
            
            // prepare assets for rendering
            renderer.prepare_assets(window.get_display(), &gpu.lock().unwrap());

            // prepare renderer for rendering
            //renderer.prepare_rendering(&target, &mut gpu.lock().unwrap());

            // request a new frame
            let mut target = window.get_display().draw();
            
            let clear_color = gpu.lock().unwrap().pipeline.clear_color;
            target.clear_color_and_depth(
                (
                    clear_color.x, 
                    clear_color.y, 
                    clear_color.z, 
                    clear_color.w
                ),
                1.0
            );

            // draw game in framebuffer
            renderer.render(window.get_display(), &render_target, &mut target, &mut gpu.lock().unwrap());

            renderer.post_render(&mut gpu.lock().unwrap());

            // draw GUI
            gui.run(window.get_display(), time_step.get_fps());
            
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

                    // relays event to the gui
                    if gui.on_event(&event) == false {
                        // relays event to the game inputs
                        inputs.lock().unwrap().process_win_events(&event)
                    }
                },
                glutin::event::Event::DeviceEvent { event, .. } => {
                    inputs.lock().unwrap().process_device_events(&event);
                },
                _ => (),
            }
        });
    }
}
