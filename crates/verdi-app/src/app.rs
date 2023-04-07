use glium::{
    glutin, 
    Surface,
};

use std::{path::Path, cell::RefCell, rc::Rc};

use verdi_graphics::prelude::{
    Database, 
    Globals
};
use verdi_window::prelude::*;
use verdi_system::prelude::{
    System, 
    SystemError, 
    SystemState
};

use crate::{
    error::AppError, 
    gui::Gui, 
    app_commands::Load, 
    commands::Command, 
};

pub enum AppMode {
    Game,
    WorldEditor,
    Modeler,
}

/// The global application. Render the Game, the WorldEditor and the UI.
/// Handle events and disptach them to the different systems.
pub struct App {
    window: Window,
    database: Rc<RefCell<Database>>,
    globals: Rc<Globals>,
    gui: Gui,
    game: Option<System>,
    editor: Option<System>,
    modeler: Option<System>,
    pub current_mode: AppMode, 
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

        let window = Window::new(1920, 1080);

        // gui initialisation
        let egui_glium = egui_glium::EguiGlium::new(
            window.get_display(), 
            &window.get_event_loop().unwrap()
        );
        let mut gui = Gui::new(egui_glium);
        gui.init();

        Self {
            window,
            database,
            globals,
            gui,
            game: None,
            editor: None,
            modeler: None,
            current_mode: AppMode::Game,
            shutdown: false,
        }
    }

    pub fn run() -> Result<(), AppError> {
        let mut app = App::new();

        let event_loop = app.window.take_event_loop().expect("No event loop in the window");

        // for accelerating debug
        let load_cmd = Load {folder: "game_example".to_string()}; 
        load_cmd.execute(&mut app);

        app.load_editor("editor");
        app.load_modeler("modeler");
    
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

            {
                let mut current_system = match app.current_mode {
                    AppMode::Game => app.game.as_mut(),
                    AppMode::WorldEditor => app.editor.as_mut(),
                    AppMode::Modeler => app.modeler.as_mut(),
                };
    
                if let Some(current_system) = current_system.as_mut() {
                    if current_system.state == SystemState::Starting {
                        // start system
                        match current_system.boot() {
                            Ok(_) => current_system.state = SystemState::Running,
                            Err(error) => {
                                current_system.state = SystemState::Loaded;
                                println!("{}", error);
                            }
                        }
                    }
                    else if current_system.state == SystemState::Stopped {
                        // stop system
                        current_system.shutdown();
                        current_system.state = SystemState::Loaded;
                    }
        
                    if current_system.state == SystemState::Running {
                        // run system
                        match current_system.run() {
                            Ok(_) => {
                                current_system.frame_starts();
                                current_system.render(app.window.get_display(), &mut target);
                                current_system.frame_ends();
                            },
                            Err(error) => {
                                current_system.state = SystemState::Loaded;
                                println!("{}", error);
                            }
                            
                        }
                    }
                }
            }
            
            // draw GUI
            if let Some(cmd) = app.gui.ui(app.window.get_display()) {
                // eventually execute a command
                cmd.execute(&mut app);
            }
            app.gui.paint(app.window.get_display(), &mut target);

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

            // bof
            let mut current_system = match app.current_mode {
                AppMode::Game => app.game.as_mut(),
                AppMode::WorldEditor => app.editor.as_mut(),
                AppMode::Modeler => app.modeler.as_mut(),
            };

            // events handling
            match ev {
                glutin::event::Event::WindowEvent { event, .. } =>  {
                    use glutin::event::WindowEvent;
                    if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }

                    // relays event to the gui
                    if app.gui.on_event(&event) == false {
                        // relays event to the system inputs
                        if let Some(current_system) = current_system.as_mut() {
                            if current_system.state == SystemState::Running {
                                current_system.on_window_event(&event);
                            }
                        }
                    }
                },
                glutin::event::Event::DeviceEvent { event, .. } => {
                    // relays event to the system inputs
                    if let Some(current_system) = current_system.as_mut() {
                        if current_system.state == SystemState::Running {
                            current_system.on_device_event(&event);
                        }
                    }
                },
                _ => (),
            }
        });
    }

    pub fn get_game(&self) -> Option<&System> {
        self.game.as_ref()
    }

    pub fn get_game_mut(&mut self) -> Option<&mut System> {
        self.game.as_mut()
    }

    pub fn get_editor(&self) -> Option<&System> {
        self.editor.as_ref()
    }

    pub fn get_editor_mut(&mut self) -> Option<&mut System> {
        self.editor.as_mut()
    }

    pub fn get_modeler(&self) -> Option<&System> {
        self.modeler.as_ref()
    }

    pub fn get_modeler_mut(&mut self) -> Option<&mut System> {
        self.modeler.as_mut()
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn load_game<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SystemError> {
        if let Some(game) = self.game.as_mut() {
            game.shutdown();
        }
 
        self.game = self.load_system(path)?;

        if let Some(game) = &self.game {
            // Allocate egui's texture id for GL texture
            let texture_id = self.gui
                .get_egui_glium_mut()
                .painter
                .register_native_texture(
                    game.get_render_target().get_color_target(), 
                    Default::default()
                );

            self.gui.get_viewport_mut().set_texture(texture_id);
            self.gui.get_code_editor_mut().set_scripts(game.get_scripts());
        }
 
         Ok(())
    }

    pub fn load_editor<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SystemError> {
        if let Some(editor) = self.editor.as_mut() {
           editor.shutdown();
        }

        self.editor = self.load_system(path)?;

        Ok(())
    }

    pub fn load_modeler<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SystemError> {
        if let Some(modeler) = self.modeler.as_mut() {
           modeler.shutdown();
        }

        self.modeler = self.load_system(path)?;

        Ok(())
    }
    
    pub fn load_system<P: AsRef<Path>>(&mut self, path: P) -> Result<Option<System>, SystemError> {
        if !path.as_ref().exists() {
            return Err(SystemError::FolderError);
        }

        let mut system = System::new(
            path, 
            self.window.get_display(), 
            self.database.clone(), 
            self.globals.clone()
        )?;

        system.load()?;

        Ok(Some(system))
    }


    pub fn shutdown(&mut self) {
        self.shutdown = true;
    }
}
