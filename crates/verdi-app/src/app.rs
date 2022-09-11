use glium::{glutin};
use rlua::{Lua, Result};
use std::{fs::File, sync::Mutex, path::Path, io::Read};

use verdi_window::prelude::*;
use verdi_graphics::prelude::*;

pub struct App;

impl App {
    pub fn run(gpu: &'static Mutex<GraphicsChip>) -> Result<()> {
        let mut window = Window::new(1024, 768);
        
        let mut renderer = Renderer::new(&window).unwrap();
    
        // lua scripting
        let script_code = App::load_script("./game_example/game.lua");
        let boot_lua = App::load_script("./crates/verdi-app/src/boot.lua");
        let run_lua = App::load_script("./crates/verdi-app/src/run.lua");

        let lua = Lua::new();
    
        BindGraphicsChip::bind(&lua, gpu)?;

        lua.context(|lua_ctx| {   
            let globals = lua_ctx.globals();

            // create verdi table
            let verdi_table = lua_ctx.create_table()?;
            globals.set("verdi", verdi_table)?;

            // load game code
            lua_ctx.load(&script_code).eval::<()>()?;
    
            // load boot code
            lua_ctx.load(&boot_lua).eval::<()>()?;

            // load run code
            lua_ctx.load(&run_lua).eval::<()>()?;

            // boot
            lua_ctx.load("verdi.boot()").exec().unwrap();

            Ok(())
        })?;

        let event_loop = window.take_event_loop().expect("No event loop in the window");
    
        event_loop.run(move |ev, _, control_flow| {
            lua.context(|lua_ctx| {
                // run callbacks
                lua_ctx.load("verdi.run()").exec().unwrap();
            });
    
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
    
    fn load_script<P: AsRef<Path>>(path: P) -> String {
        // todo : gestion d'erreur
        let mut f = File::open(path).unwrap();
        let mut content: String = String::new();
        f.read_to_string(&mut content).unwrap();
        content
    }
}
