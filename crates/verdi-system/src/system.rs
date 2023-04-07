use std::{rc::Rc, cell::RefCell, path::{Path, PathBuf}};

use glium::{
    Display, 
    Frame, 
    glutin::event::{WindowEvent, DeviceEvent}, 
    framebuffer::SimpleFrameBuffer
};
use mlua::Lua;
use verdi_audio::prelude::{AudioHandle, Audio, BindAudio};
use verdi_ecs::prelude::{WorldHandle, World, BindWorld};
use verdi_graphics::prelude::{
    GraphicsChip, 
    Renderer, 
    BindGraphicsChip, 
    RenderTarget,
    Database, 
    Globals, 
    PassHandle,
};
use verdi_input::prelude::{Inputs, BindInputs};
use verdi_math::prelude::BindMath;

use crate::{
    lua_context::LuaContext, 
    prelude::Scripts, 
    time_step::TimeStep, 
    scripts::ScriptError, 
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SystemError {
    #[error("Script error")]
    ScriptError(#[from] ScriptError),
    #[error("Game folder doesn't exists")]
    FolderError,
    #[error("Cannot evaluate lua code")]
    LuaError(#[from] mlua::Error),
}

#[derive(PartialEq)]
pub enum SystemState {
    Unloaded,
    Loaded,
    Starting,
    Running,
    Paused,
    Stopped,
}

/// The Game system.
pub struct System {
    pub state: SystemState,
    lua: Lua,
    world: WorldHandle,
    gpu: Rc<RefCell<GraphicsChip>>,
    renderer: Renderer,
    render_target: RenderTarget,
    inputs: Rc<RefCell<Inputs>>,
    audio: AudioHandle,
    path: PathBuf,
    scripts: Rc<RefCell<Scripts>>,
    pub time_step: TimeStep,
    last_error: String,
}

impl System {
    pub fn new<P: AsRef<Path>>(path: P, display: &Display, database: Rc<RefCell<Database>>, globals: Rc<Globals>) -> Result<Self, SystemError> {
            let gpu = Rc::new(
                RefCell::new(
                    GraphicsChip::new(database, globals)
                        .expect("GraphicsChip initialisation failed")
                )
            );

            let renderer = Renderer::new();

            let render_target = RenderTarget::new(
                display, 
                320, 
                240)
                .expect("Render target creation failed");

            let world = Rc::new(
                RefCell::new(
                    World::new()
                )
            );

            let audio = Rc::new(
                RefCell::new(
                    Audio::new()
                )
            );

        Ok(Self { 
            state: SystemState::Unloaded,
            lua: Lua::new(),
            world: WorldHandle::new(world),
            gpu,
            renderer,
            render_target,
            inputs: Rc::new(RefCell::new(Inputs::new())),
            audio: AudioHandle::new(audio),
            path: path.as_ref().to_path_buf(),
            scripts: Rc::new(RefCell::new(Scripts::new(path)?)),
            time_step: TimeStep::new(),
            last_error: String::new(),
        })
    }

    pub fn load(&mut self) -> Result<(), SystemError> {
        self.scripts.as_ref().borrow_mut().load_dir(&self.path)?;
        self.state = SystemState::Loaded;

        Ok(())
    }

    /// called at the start of the game execution
    pub fn boot(&mut self) -> Result<(), SystemError> {
        LuaContext::create_verdi_table(&self.lua)?;

        BindWorld::bind(&self.lua, self.world.clone())?;
        BindGraphicsChip::bind(&self.lua, self.gpu.clone())?;
        BindInputs::bind(&self.lua, self.inputs.clone())?;
        BindMath::bind(&self.lua)?;
        BindAudio::bind(&self.lua, self.audio.clone())?;
        
        LuaContext::load_internal_scripts(&self.lua)?;
        LuaContext::load_scripts(&self.lua, &self.scripts.borrow())?;

        self.gpu.borrow_mut().on_game_start();

        LuaContext::call_boot(&self.lua)?;

        Ok(())
    }

    /// Called every frame 
    pub fn run(&mut self) -> Result<(), SystemError> {
        let delta_time = self.time_step.tick();
        
        self.scripts.as_ref().borrow_mut().hot_reload(&self.lua)?;

        let pass = PassHandle {
            graph: self.gpu.borrow().render_graph.clone(),
            id: self.gpu.borrow().render_graph.borrow_mut().create_pass(),
        };

        // callbacks
        if let Err(err) = LuaContext::call_run(&self.lua, delta_time, pass) {
            let current_error = err.to_string();
            if self.last_error != current_error {
                println!("{}", err);
                self.last_error = current_error;
            }
        }

        Ok(())
    }

    /// Called every frame. Draw as requested during the run call.
    pub fn render(&mut self, display: &Display, frame: &mut Frame) {
        self.gpu.borrow_mut().new_frame();
    
        // prepare assets for rendering
        self.renderer.prepare_assets(display, &self.gpu.borrow());

        // create a framebuffer to draw into 
        let color_target = self.render_target.get_color_target();
        let mut framebuffer = SimpleFrameBuffer::with_depth_buffer(
            display, 
            color_target.as_ref(), 
            self.render_target.get_depth_target()
        ).unwrap();

        // draw system in framebuffer
        self.renderer.render(&mut framebuffer, &mut self.gpu.borrow_mut());

        // blit in frame
        self.renderer.blit_buffers_to_frame(&mut framebuffer, frame);

        self.renderer.post_render(&mut self.gpu.borrow_mut());
    }

    pub fn frame_starts(&mut self) {
        self.gpu.borrow_mut().flush_stream_buffer();
    }

    pub fn frame_ends(&mut self) {
        // prepare next frame
        self.gpu.borrow_mut().frame_ends();
    }

    pub fn on_window_event(&mut self, event: &WindowEvent) {
        self.inputs.borrow_mut().process_win_events(event)
    }

    pub fn on_device_event(&mut self, event: &DeviceEvent) {
        self.inputs.borrow_mut().process_device_events(event);
    }

    pub fn shutdown(&mut self) {
        self.gpu.borrow_mut().on_game_shutdown();
        self.renderer.on_game_shutdown();
    }

    pub fn get_scripts(&self) -> Rc<RefCell<Scripts>> {
        self.scripts.clone()
    }

    pub fn get_render_target(&self) -> &RenderTarget {
        &self.render_target
    }
}