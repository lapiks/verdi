use glium::{
    glutin, 
    Surface,
};

use rlua::Lua;

use std::{sync::{Mutex, Arc}, path::Path};

use verdi_window::prelude::*;
use verdi_graphics::prelude::*;
use verdi_game::prelude::{Game, GameState, GameError};
use verdi_input::prelude::*;

use crate::{
    error::AppError, 
    gui::Gui, 
};

pub struct App {
    game: Option<Game>,
    pub game_state: GameState,
}

impl App {
    pub fn new() -> Self {
        Self {
            game: None,
            game_state: GameState::Stopped,
        }
    }

    pub fn run() -> Result<(), AppError> {
        let mut app = App::new();

        let mut window = Window::new(1024, 768);

        // à mettre dans game
        let render_target = RenderTarget::new(
            window.get_display(),
            320, 
            240
        ).expect("Render target creation failed");
        
        // à mettre dans game
        let gpu = Arc::new(
            Mutex::new(
                GraphicsChip::new()
                    .expect("GraphicsChip initialisation failed")
            )
        );

        // à mettre dans game
        let inputs = Arc::new(
            Mutex::new(
                Inputs::new()
            )
        );

        // à mettre dans game
        let mut renderer = Renderer::new();

        let event_loop = window.take_event_loop().expect("No event loop in the window");

        // gui initialisation
        let egui_glium = egui_glium::EguiGlium::new(
            &window.get_display(), 
            &event_loop
        );
        let mut gui = Gui::new(egui_glium);
        gui.init();

        let lua = Lua::new();
    
        BindGraphicsChip::bind(&lua, gpu.clone())?;
        BindInputs::bind(&lua, inputs.clone())?;
        
        // if app.load_game("game_example/").is_err() {
        //     println!("Loading game failed");
        // }
    
        event_loop.run(move |ev, _, control_flow| {
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

            if app.game_state == GameState::Start {
                if let Some(game) = app.game.as_mut() {
                    gpu.lock().unwrap().on_game_start();
                    game.boot(&lua).expect("Game boot failed");
                    app.game_state = GameState::Running;
                }
            }
            else if app.game_state == GameState::Stopped {
                // stop game
                gpu.lock().unwrap().on_game_shutdown();
                renderer.on_game_shutdown();
            }

            if app.game_state == GameState::Running {
                if let Some(game) = app.game.as_mut() {
                    game.run(&lua);

                    gpu.lock().unwrap().new_frame();
    
                    // prepare assets for rendering
                    renderer.prepare_assets(window.get_display(), &gpu.lock().unwrap());
    
                    // draw game in framebuffer
                    renderer.render(window.get_display(), &render_target, &mut target, &mut gpu.lock().unwrap());
    
                    renderer.post_render(&mut gpu.lock().unwrap());
                }
            }

            // draw GUI
            gui.render(window.get_display(),  &mut target, &mut app);

            // ends frame
            target.finish().unwrap();
            
            // prepare next frame
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

    pub fn get_game(&self) -> Option<&Game> {
        self.game.as_ref()
    }

    pub fn load_game<P: AsRef<Path>>(&mut self, path: P) -> Result<(), GameError> {
        self.game = Some(
            Game::new(path)?
        );

        if let Some(game) = self.game.as_mut() {
            game.load()?;
        }

        Ok(())
    }
}
