use glium::{glutin, Surface, uniform};
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};
use std::{path::Path, fs::File, error::Error, io::Read};

use verdi_graphics::prelude::*;

pub struct App {

}

impl App {
    pub fn run() -> Result<()> {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        
        let mut gpu = GraphicsChip::new(display).unwrap();

        // lua scripting
        let script_code = App::load_script("./game_example/game.lua");

        let lua = Lua::new();

        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();

            lua_ctx.load(&script_code).eval::<()>()?;

            lua_ctx.load("start()").exec()?;

            Ok(())
        })?;

        event_loop.run(move |ev, _, control_flow| {
            lua.context(|lua_ctx| {
                // gestion erreur
                lua_ctx.load("update()").exec().unwrap();
                lua_ctx.load("draw()").exec().unwrap();
            });

            //gpu.render();

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