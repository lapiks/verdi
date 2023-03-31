use std::{rc::Rc, cell::RefCell};

use glium::{Display, Frame};
use mlua::Lua;

use verdi_game::prelude::{TimeStep, LuaContext};
use verdi_graphics::prelude::{
    GraphicsChip, 
    Renderer, 
    Database, 
    Globals, 
    PassHandle, 
    BindGraphicsChip, 
    RenderTarget,
};
use verdi_input::prelude::{BindInputs, Inputs};
use verdi_math::prelude::BindMath;

pub struct WorldEditor {
    gpu: Rc<RefCell<GraphicsChip>>,
    renderer: Renderer,
    render_target: RenderTarget,
    inputs: Rc<RefCell<Inputs>>,
    lua: Lua,
    time_step: TimeStep,
    last_error: String,
}

impl WorldEditor {
    pub fn new(display: &Display, db: Rc<RefCell<Database>>, globals: Rc<Globals>) -> Self {
        let gpu = GraphicsChip::new(db, globals)
            .expect("World Editor GraphicsChip initialisation failed");

        let render_target = RenderTarget::new(
            display, 
            800, 
            600)
            .expect("Render target creation failed");

        Self {
            gpu: Rc::new(RefCell::new(gpu)),
            renderer: Renderer::new(),
            render_target,
            inputs: Rc::new(RefCell::new(Inputs::new())),
            lua: Lua::new(),
            time_step: TimeStep::new(),
            last_error: String::new(),
        }
    }

    /// called at the start of the Editor execution
    pub fn boot(&mut self) {
        LuaContext::create_verdi_table(&self.lua).expect("msg");

        BindGraphicsChip::bind(&self.lua, self.gpu.clone()).expect("msg");
        BindInputs::bind(&self.lua, self.inputs.clone()).expect("msg");
        BindMath::bind(&self.lua).expect("msg");
        
        LuaContext::load_internal_scripts(&self.lua).expect("msg");
        //LuaContext::load_scripts(&self.lua, &self.scripts.borrow());

        self.gpu.borrow_mut().on_game_start();

        LuaContext::call_boot(&self.lua).expect("msg");
    }

    /// Called every frame 
    pub fn run(&mut self) {
        let delta_time = self.time_step.tick();

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
    }

    /// Called every frame. Draw as requested during the run call.
    pub fn render(&mut self, display: &Display, target: &mut Frame) {
        self.gpu.borrow_mut().new_frame();
    
        // prepare assets for rendering
        self.renderer.prepare_assets(display, &self.gpu.borrow());

        // draw game in framebuffer
        self.renderer.render(&self.render_target, display, target, &mut self.gpu.borrow_mut());

        self.renderer.post_render(&mut self.gpu.borrow_mut());
    }

    pub fn frame_ends(&mut self) {
        // prepare next frame
        self.gpu.borrow_mut().frame_ends();
    }
}

