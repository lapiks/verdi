use glium::{
    glutin, 
    Surface,
};

use verdi_editor::prelude::WorldEditor;

use std::{path::Path, cell::RefCell, rc::Rc};

use verdi_graphics::prelude::{
    Database, 
    Globals
};
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

#[derive(PartialEq)]
pub enum EditorState {
    Boot,
    Running,
    Stopped,
}

/// The global application. Render the Game, the WorldEditor and the UI.
/// Handle events and disptach them to the different systems.
pub struct App {
    window: Window,
    database: Rc<RefCell<Database>>,
    globals: Rc<Globals>,
    game: Option<Game>,
    pub game_state: GameState,
    editor: Option<WorldEditor>,
    editor_state: EditorState,
    pub show_editor: bool,
    pub shutdown: bool,
}

impl App {
    pub fn new() -> Self {
        let database = Rc::new(
            RefCell::new(
                Database::new()
            )
        );
        let globals = Rc::new(
            Globals::new(
                &mut database.borrow_mut()
            ).expect("Globals creation failed")
        );
        Self {
            window: Window::new(1920, 1080),
            database,
            globals,
            game: None,
            game_state: GameState::Loaded,
            editor: None,
            editor_state: EditorState::Boot,
            show_editor: false,
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

        // for accelerating debug
        let load_cmd = Load {folder: "game_example".to_string()}; 
        load_cmd.execute(&mut app);

        app.load_editor();
    
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

            if app.show_editor {
                if app.editor_state == EditorState::Boot {
                    if let Some(editor) = app.editor.as_mut() {
                        editor.boot();
                        app.editor_state = EditorState::Running;
                    }
                }
                else {
                    if let Some(editor) = app.editor.as_mut() {
                        editor.run();
                        editor.render(app.window.get_display(), &mut target);
                        editor.frame_ends();
                    }
                }
            }
            else {
                if app.game_state == GameState::Start {
                    if let Some(game) = app.game.as_mut() {
                        // start game
                        game.boot().expect("Game boot failed");
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
                        game.run();
    
                        game.frame_starts();
                        game.render(app.window.get_display(), &mut target);
                        game.frame_ends();
                    }
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

    pub fn load_game<P: AsRef<Path>>(&mut self, path: P) -> Result<(), GameError> {
        if !path.as_ref().exists() {
            return Err(GameError::GameFolderError);
        }

        if let Some(game) = self.game.as_mut() {
           game.shutdown();
        }

        self.game = Some(
            Game::new(path, self.window.get_display(), self.database.clone(), self.globals.clone())?
        );

        if let Some(game) = self.game.as_mut() {
            game.load()?;
        }

        self.game_state = GameState::Loaded;

        Ok(())
    }

    pub fn load_editor(&mut self) {
        self.editor = Some(
            WorldEditor::new(self.window.get_display(), self.database.clone(), self.globals.clone())
        );
    }

    pub fn shutdown(&mut self) {
        self.shutdown = true;
    }
}
