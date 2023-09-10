use std::{rc::Rc, cell::RefCell, path::Path};

use mlua::Lua;
use verdi_audio::prelude::{AudioHandle, Audio, BindAudio};
use verdi_ecs::prelude::{WorldHandle, World, BindWorld};
use verdi_graphics::prelude::{
    GraphicsChip, 
    Renderer, 
    BindGraphicsChip, 
    RenderTarget,
    PassHandle,
};
use verdi_input::prelude::{Inputs, BindInputs, MouseButton, Key};
use verdi_math::prelude::{BindMath, Math};

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
    ctx: Box<dyn miniquad::RenderingBackend>,
    pub state: SystemState,
    lua: Lua,
    world: WorldHandle,
    gpu: Rc<RefCell<GraphicsChip>>,
    renderer: Renderer,
    render_target: RenderTarget,
    inputs: Rc<RefCell<Inputs>>,
    audio: AudioHandle,
    math: Rc<RefCell<Math>>,
    scripts: Rc<RefCell<Scripts>>,
    pub time_step: TimeStep,
    last_error: String,
}

impl System {
    pub fn new() -> Result<Self, SystemError> {
        let mut ctx = miniquad::window::new_rendering_backend();

        let math = Rc::new(RefCell::new(Math::new()));

        let gpu = Rc::new(
            RefCell::new(
                GraphicsChip::new(math.clone()).expect("GraphicsChip initialisation failed")
            )
        );

        let renderer = Renderer::new();

        let render_target = RenderTarget::new(
            &mut *ctx,
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
            ctx,
            state: SystemState::Unloaded,
            lua: Lua::new(),
            world: WorldHandle::new(world),
            gpu,
            renderer,
            render_target,
            inputs: Rc::new(RefCell::new(Inputs::new())),
            audio: AudioHandle::new(audio),
            math,
            scripts: Rc::new(RefCell::new(Scripts::new()?)),
            time_step: TimeStep::new(),
            last_error: String::new(),
        })
    }

    pub fn load_scripts<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SystemError> {
        self.scripts.as_ref().borrow_mut().load_dir(path)?;
        self.state = SystemState::Loaded;

        Ok(())
    }

    /// called at the start of the game execution
    pub fn boot(&mut self) -> Result<(), SystemError> {
        LuaContext::create_verdi_table(&self.lua)?;

        BindWorld::bind(&self.lua, self.world.clone())?;
        BindGraphicsChip::bind(&self.lua, self.gpu.clone())?;
        BindInputs::bind(&self.lua, self.inputs.clone())?;
        BindMath::bind(&self.lua, self.math.clone())?;
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
    pub fn draw(&mut self) {
        if self.state == SystemState::Running {
            // run system
            match self.run() {
                Ok(_) => {
                    self.frame_starts();

                    self.gpu.borrow_mut().new_frame();
    
                    // prepare resources for rendering
                    self.gpu.borrow_mut().prepare_gpu_assets(&mut *self.ctx);
            
                    // draw system in framebuffer
                    self.renderer.render(&mut *self.ctx, &self.render_target, &mut self.gpu.borrow_mut());
            
                    // blit in frame
                    //self.renderer.blit_buffers_to_frame(&mut framebuffer, frame);
            
                    self.renderer.post_render(&mut self.gpu.borrow_mut());
                    
                    self.frame_ends();
                },
                Err(error) => {
                    self.state = SystemState::Loaded;
                    println!("{}", error);
                }
                
            }
        }
    }

    pub fn frame_starts(&self) {
        self.gpu.borrow_mut().flush_stream_buffer();
        self.inputs.borrow_mut().reset();
    }

    pub fn frame_ends(&self) {
        // prepare next frame
        self.gpu.borrow_mut().frame_ends();
    }

    pub fn on_mouse_move(&mut self, x: f32, y: f32) {
        self.inputs.borrow_mut().on_mouse_move(x, y)
    }

    pub fn on_mouse_wheel(&mut self, x: f32, y: f32) {
        self.inputs.borrow_mut().on_mouse_wheel(x, y)
    }

    pub fn on_mouse_button_down(&mut self, button: MouseButton, x: f32, y: f32) {
        self.inputs.borrow_mut().on_mouse_button_down(button, x, y)
    }

    pub fn on_mouse_button_up(&mut self, button: MouseButton, x: f32, y: f32) {
        self.inputs.borrow_mut().on_mouse_button_up(button, x, y)
    }

    pub fn on_key_down(&mut self, keycode: Key, repeat: bool) {
        self.inputs.borrow_mut().on_key_down(keycode, repeat)
    }

    pub fn on_key_up(&mut self, keycode: Key) {
        self.inputs.borrow_mut().on_key_up(keycode)
    }

    pub fn shutdown(&mut self) {
        self.gpu.borrow_mut().on_game_shutdown();
    }

    pub fn get_scripts(&self) -> Rc<RefCell<Scripts>> {
        self.scripts.clone()
    }

    pub fn get_render_target(&self) -> &RenderTarget {
        &self.render_target
    }
}