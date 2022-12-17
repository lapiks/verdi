use glium::{
    glutin, 
    Surface, 
    Frame,
};

use rlua::Lua;

use std::path::Path;

use verdi_window::prelude::*;
use verdi_game::prelude::{
    Game, 
    GameError
};

use crate::{
    error::AppError, 
    gui::Gui, 
    app_commands::Load, 
    commands::Command, 
};

#[derive(PartialEq)]
pub enum GameState {
    Loaded,
    Start,
    Running,
    Paused,
    Stopped,
}

pub struct App {
    window: Window,
    game: Option<Game>,
    pub game_state: GameState,
    pub shutdown: bool
}

impl App {
    pub fn new() -> Self {
        Self {
            window: Window::new(1920, 1080),
            game: None,
            game_state: GameState::Loaded,
            shutdown: false,
        }
    }

    pub fn run() -> Result<(), AppError> {
        let mut app = App::new();

        let event_loop = app.window.take_event_loop().expect("No event loop in the window");

        // gui initialisation
        let egui_glium = egui_glium::EguiGlium::new(
            app.window.get_display(), 
            &event_loop
        );
        let mut gui = Gui::new(egui_glium);
        gui.init();

        let lua = Lua::new();

        // for accelerating debug
        let load_cmd = Load {folder: "game_example".to_string()}; 
        load_cmd.execute(&mut app);
    
        event_loop.run(move |ev, _, control_flow| {
            // request a new frame
            let mut target = app.window.get_display().draw();

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
                    // start game
                    game.boot(&lua).expect("Game boot failed");
                    app.game_state = GameState::Running;
                }
            }
            else if app.game_state == GameState::Stopped {
                if let Some(game) = app.game.as_mut() {
                    // stop game
                    game.shutdown();
                    app.game_state = GameState::Loaded;
                }
            }

            if app.game_state == GameState::Running {
                if let Some(game) = app.game.as_mut() {
                    game.run(&lua);

                    game.frame_starts();
                    game.render(app.window.get_display(), &mut target);
                    game.frame_ends();
                }
            }

            // draw GUI
            if let Some(cmd) = gui.ui(&app) {
                // eventually execute a command
                cmd.execute(&mut app);
            }
            gui.paint(app.window.get_display(), &mut target);

            // ends frame
            target.finish().unwrap();
    
            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);

            if app.shutdown {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
            else {
                *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            }

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
                        if app.game_state == GameState::Running {
                            if let Some(game) = app.game.as_mut() {
                                game.on_window_event(&event);
                            }
                        }
                    }
                },
                glutin::event::Event::DeviceEvent { event, .. } => {
                    // relays event to the game inputs
                    if app.game_state == GameState::Running {
                        if let Some(game) = app.game.as_mut() {
                            game.on_device_event(&event);
                        }
                    }
                },
                _ => (),
            }
        });
    }

    pub fn get_game(&self) -> Option<&Game> {
        self.game.as_ref()
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    } 

    pub fn draw_gui(&mut self, target: &mut Frame) {

    }

    pub fn load_game<P: AsRef<Path>>(&mut self, path: P) -> Result<(), GameError> {
        if !path.as_ref().exists() {
            return Err(GameError::GameFolderError);
        }

        if let Some(game) = self.game.as_mut() {
           game.shutdown();
        }

        self.game = Some(
            Game::new(path, self.window.get_display())?
        );

        if let Some(game) = self.game.as_mut() {
            game.load()?;
        }

        self.game_state = GameState::Loaded;

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.shutdown = true;
    }
}
