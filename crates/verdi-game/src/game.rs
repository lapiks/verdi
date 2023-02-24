use std::{rc::Rc, cell::RefCell, path::{Path, PathBuf}, sync::{Arc, Mutex}};

use glium::{Display, Frame, glutin::event::{WindowEvent, DeviceEvent}};
use mlua::Lua;
use verdi_graphics::prelude::{
    GraphicsChip, 
    Renderer, 
    BindGraphicsChip, 
    RenderTarget,
    DataBase, 
    Globals,
};
use verdi_input::prelude::{Inputs, BindInputs};
use verdi_math::prelude::BindMath;

use crate::{
    lua_context::LuaContext, 
    prelude::Scripts, 
    time_step::TimeStep, 
    file_watcher::FileWatcherError,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Reading lua script failed")]
    ReadLuaScriptFailed(#[from] std::io::Error),
    #[error("Cannot evaluate lua code")]
    LuaError(#[from] mlua::Error),
    #[error("File watcher error")]
    FileWatcherError(#[from] FileWatcherError),
    #[error("Game folder doesn't exists")]
    GameFolderError,
}

/// The Game system.
pub struct Game {
    gpu: Arc<Mutex<GraphicsChip>>,
    renderer: Renderer,
    render_target: RenderTarget,
    inputs: Arc<Mutex<Inputs>>,
    path: PathBuf,
    scripts: Rc<RefCell<Scripts>>,
    pub time_step: TimeStep,
    last_error: String,
}

impl Game {
    pub fn new<P: AsRef<Path>>(path: P, display: &Display, database: Rc<RefCell<DataBase>>, globals: Rc<Globals>) -> Result<Self, GameError> {
            let gpu = Arc::new(
                Mutex::new(
                    GraphicsChip::new(database, globals)
                        .expect("GraphicsChip initialisation failed")
                )
            );
    
            let inputs = Arc::new(
                Mutex::new(
                    Inputs::new()
                )
            );

            let renderer = Renderer::new();

            let render_target = RenderTarget::new(
                display, 
                320, 
                240)
                .expect("Render target creation failed");

        Ok(Self { 
            gpu,
            renderer,
            render_target,
            inputs,
            path: path.as_ref().to_path_buf(),
            scripts: Rc::new(RefCell::new(Scripts::new(path)?)),
            time_step: TimeStep::new(),
            last_error: String::new(),
        })
    }

    pub fn load(&mut self) -> Result<(), GameError> {
        Ok(self.scripts.borrow_mut().load_dir(&self.path)?)
    }

    /// called at the start of the game execution
    pub fn boot(&mut self, lua: &Lua) -> Result<(), GameError>{
        LuaContext::create_verdi_table(lua)?;
        LuaContext::load_internal_scripts(lua)?;
        LuaContext::load_scripts(lua, &self.scripts.borrow())?;

        BindGraphicsChip::bind(&lua, self.gpu.clone())?;
        BindInputs::bind(&lua, self.inputs.clone())?;
        BindMath::bind(&lua)?;

        self.gpu.lock().unwrap().on_game_start();

        LuaContext::call_boot(lua)?;

        Ok(())
    }

    /// Called every frame 
    pub fn run(&mut self, lua: &Lua) {
        let delta_time = self.time_step.tick();
        
        self.scripts.borrow_mut().hot_reload(lua);

        // callbacks
        if let Err(err) = LuaContext::call_run(lua, delta_time) {
            let current_error = err.to_string();
            if self.last_error != current_error {
                println!("{}", err);
                self.last_error = current_error;
            }
        }
    }

    /// Called every frame. Draw as requested during the run call.
    pub fn render(&mut self, display: &Display, target: &mut Frame) {
        self.gpu.lock().unwrap().new_frame();
    
        // prepare assets for rendering
        self.renderer.prepare_assets(display, &self.gpu.lock().unwrap());

        // draw game in framebuffer
        self.renderer.render(&self.render_target, display, target, &mut self.gpu.lock().unwrap());

        self.renderer.post_render(&mut self.gpu.lock().unwrap());
    }

    pub fn frame_starts(&mut self) {
        self.gpu.lock().unwrap().flush_stream_buffer();
    }

    pub fn frame_ends(&mut self) {
        // prepare next frame
        self.gpu.lock().unwrap().frame_ends();
    }

    pub fn on_window_event(&mut self, event: &WindowEvent) {
        self.inputs.lock().unwrap().process_win_events(event)
    }

    pub fn on_device_event(&mut self, event: &DeviceEvent) {
        self.inputs.lock().unwrap().process_device_events(event);
    }

    pub fn shutdown(&mut self) {
        self.gpu.lock().unwrap().on_game_shutdown();
        self.renderer.on_game_shutdown();
    }

    pub fn get_scripts(&self) -> Rc<RefCell<Scripts>> {
        self.scripts.clone()
    }
}