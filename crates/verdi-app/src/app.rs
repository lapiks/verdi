use glium::{
    glutin, 
    Surface,
};

use rlua::Lua;

use std::{sync::{Mutex, Arc}, time::Duration, rc::Rc, cell::RefCell};

use verdi_utils::make_relative_path;
use verdi_window::prelude::*;
use verdi_graphics::prelude::*;
use verdi_gui::prelude::*;
use verdi_game::prelude::Scripts;
use verdi_input::prelude::*;

use crate::{
    error::AppError, 
    lua_context::LuaContext,
    time_step::TimeStep, 
    file_watcher::FileWatcher
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

        let scripts = Rc::new(RefCell::new(Scripts::new()));
        scripts.borrow_mut().load_dir("game_example/")?;
        scripts.borrow_mut().load_file("crates/verdi-app/src/boot.lua")?;
        scripts.borrow_mut().load_file("crates/verdi-app/src/run.lua")?;

        LuaContext::load_scripts(&lua, &scripts.borrow())?;
        LuaContext::call_boot(&lua)?;

        let file_watcher = FileWatcher::new(
            "./game_example", 
            Duration::from_secs(5))
        .expect("File watcher initialisation failed");

        let event_loop = window.take_event_loop().expect("No event loop in the window");

        let egui_glium = egui_glium::EguiGlium::new(&window.get_display(), &event_loop);
        let mut gui = Gui::new(egui_glium, scripts.clone());

        //gui.get_code_editor_mut().code = 

        let mut last_error: String = String::new();
        let mut time_step = TimeStep::new();
    
        event_loop.run(move |ev, _, control_flow| {
            // hot-reload
            if let Some(watcher_event) = file_watcher.get_event() {
                if let notify::EventKind::Modify(_) = watcher_event.kind {
                    for path in watcher_event.paths.iter() {
                        if let Ok(relative_path) = make_relative_path(path) {
                            if let Some(script) = scripts.borrow_mut().get_script_mut(&relative_path) {
                                // reload script
                                script
                                    .reload(relative_path)
                                    .expect("Reload script file failed");

                                // update lua context
                                LuaContext::load_script(
                                    &lua, 
                                    script
                                ).expect("Reload script failed");
                            }
                        }
                    }
                }
            }

            let delta_time = time_step.tick();

            // callbacks
            if let Err(err) = LuaContext::call_run(&lua, delta_time) {
                let current_error = err.to_string();
                if last_error != current_error {
                    println!("{}", err);
                    last_error = current_error;
                }
            }

            // prepare renderer for rendering
            //renderer.prepare_rendering(&target, &mut gpu.lock().unwrap());

            // request a new frame
            let mut target = window.get_display().draw();
            
            target.clear_color_and_depth(
                (
                    0.0, 
                    0.0, 
                    0.0, 
                    1.0
                ),
                1.0
            );

            gpu.lock().unwrap().new_frame();

            // prepare assets for rendering
            renderer.prepare_assets(window.get_display(), &gpu.lock().unwrap());

            // draw game in framebuffer
            renderer.render(window.get_display(), &render_target, &mut target, &mut gpu.lock().unwrap());

            renderer.post_render(&mut gpu.lock().unwrap());

            // update GUI
            gui.update(inputs.clone());

            // draw GUI
            gui.render(window.get_display(),  &mut target, time_step.get_fps());

            // ends frame
            target.finish().unwrap();
            
            gpu.lock().unwrap().next_frame();
    
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
