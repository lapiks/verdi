use glium::{Surface, glutin, Display};
use verdi_input::prelude::{Key, MouseButton};

use std::{path::Path, collections::HashMap};

use verdi_system::prelude::{
    System, 
    SystemError, 
    SystemState,
    EventHandler,
};

use crate::{
    app_commands::{Load, Run}, 
    commands::Command, 
    gui::Gui, window::Window, error::AppError, 
};

pub enum AppMode {
    Game,
    WorldEditor,
    Modeler,
}

type SystemId = u32;

const GAME: SystemId = 0;
const EDITOR: SystemId = 1;
const MODELER: SystemId = 2;

/// The global application. Render the Game, the WorldEditor and the UI.
/// Handle events and disptach them to the different systems.
pub struct App {
    window: Window,
    gui: Gui,
    systems: HashMap<SystemId, System>,
    current_system: SystemId,
    pub current_mode: AppMode, 
    pub shutdown: bool,
    boot: bool,
}

impl App {
    fn new() -> Self {
        let window = Window::new(1920, 1080, false);

        let mut gui = Gui::new();
        gui.init();

        Self {
            window,
            gui,
            systems: HashMap::default(),
            current_system: GAME,
            current_mode: AppMode::Game,    
            shutdown: false,
            boot: true,
        }
    }

    pub fn run() -> Result<(), AppError> {
        let mut app = App::new();

        let event_loop = app.window.take_event_loop().expect("No event loop in the window");

        let ctx = app.window.take_display().expect("No Glium Display in the window");

        // for accelerating debug
        let load_cmd = Load {folder: "game_example".to_string()}; 
        load_cmd.execute(&mut app);

        let run_cmd = Run {}; 
        run_cmd.execute(&mut app);

        app.load_editor("editor");
        app.load_modeler("modeler");

        event_loop.run(move |ev, _, control_flow| {
            app.update();
            app.draw(&ctx);

            if let Some(system) = app.get_current_system_mut() {
                // events handling
                match ev {
                    glutin::event::Event::WindowEvent { event, .. } =>  {
                        use glutin::event::WindowEvent;
                        if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                        }

                        match event {
                            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                                match input.state {
                                    glutin::event::ElementState::Pressed => {
                                        if let Some(keycode) = input.virtual_keycode {
                                            system.on_key_down(Key::from(keycode));
                                        }
                                        
                                    },
                                    glutin::event::ElementState::Released => {
                                        if let Some(keycode) = input.virtual_keycode {
                                            system.on_key_up(Key::from(keycode));
                                        }
                                    },
                                }
            
                                let key = match input.virtual_keycode {
                                    Some(key) => key,
                                    None => return,
                                };
                            },
                            glutin::event::WindowEvent::ModifiersChanged(modifiers_state) => {
                                // todo
                            },
                            glutin::event::WindowEvent::MouseInput { button, state, .. } => {
                                match state {
                                    glutin::event::ElementState::Pressed => system.on_mouse_button_down(MouseButton::from(button)),
                                    glutin::event::ElementState::Released => system.on_mouse_button_up(MouseButton::from(button)),
                                }
                            },
                            _ => return,
                        };
                    },
                    glutin::event::Event::DeviceEvent { event, .. } => {
                        match event {
                            glutin::event::DeviceEvent::MouseMotion { delta } => system.on_mouse_move(delta.0 as f32, delta.1 as f32),
                            //glutin::event::DeviceEvent::MouseWheel { delta } => system.on_mouse_wheel(delta),
                            _ => return,
                        }
                    },
                    _ => (),
                }
            }

            let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

            if app.shutdown {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
            else {
                *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            }
        });
    }

    pub fn init(&mut self) {
        // for accelerating debug
        let load_cmd = Load {folder: "game_example".to_string()}; 
        load_cmd.execute(self);

        self.load_editor("editor");
        self.load_modeler("modeler");

        // for accelerating debug
        Run {}.execute(self);
    }

    pub fn get_current_system(&self) -> Option<&System> {
       self.get_system(self.current_system)
    }

    pub fn get_current_system_mut(&mut self) -> Option<&mut System> {
        self.get_system_mut(self.current_system)
    }

    pub fn get_system(&self, id: SystemId) -> Option<&System> {
        self.systems.get(&id)
    }

    pub fn get_system_mut(&mut self, id: SystemId) -> Option<&mut System> {
        self.systems.get_mut(&id)
    }

    pub fn get_game(&self) -> Option<&System> {
        self.get_system(GAME)
    }

    pub fn get_game_mut(&mut self) -> Option<&mut System> {
        self.get_system_mut(GAME)
    }

    pub fn get_editor(&self) -> Option<&System> {
        self.get_system(EDITOR)
    }

    pub fn get_editor_mut(&mut self) -> Option<&mut System> {
        self.get_system_mut(EDITOR)
    }

    pub fn get_modeler(&self) -> Option<&System> {
        self.get_system(MODELER)
    }

    pub fn get_modeler_mut(&mut self) -> Option<&mut System> {
        self.get_system_mut(EDITOR)
    }

    pub fn load_system<P: AsRef<Path>>(&mut self, path: P, system_id: SystemId) -> Result<(), SystemError> {
        if !path.as_ref().exists() {
            return Err(SystemError::FolderError);
        }

        if let Some(system) = self.get_system_mut(system_id) {
            system.on_shutdown();
        }
        else {
            self.systems.insert(
                system_id,
                System::new()?,
            );
        }

        if let Some(system) = self.get_system_mut(system_id) {
            system.load_scripts(path)?;
        }

        //if let Some(game) = &self.game {
            // Allocate egui's texture id for GL texture
            // let texture_id = self.gui
            //     .get_egui_glium_mut()
            //     .painter
            //     .register_native_texture(
            //         game.get_render_target().get_color_target(), 
            //         Default::default()
            //     );

            // self.gui.get_viewport_mut().set_texture(texture_id);
            // self.gui.get_code_editor_mut().set_scripts(game.get_scripts());
        //}

        Ok(())
    }

    pub fn load_game<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SystemError> {
        self.load_system(path, GAME)
    }

    pub fn load_editor<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SystemError> {
        self.load_system(path, EDITOR)
    }

    pub fn load_modeler<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SystemError> {
        self.load_system(path, MODELER)
    }

    pub fn shutdown(&mut self) {
        self.shutdown = true;
    }

    fn update(&mut self) {
        if let Some(current_system) = self.get_current_system_mut() {
            match current_system.state {
                SystemState::Starting => {
                    match current_system.boot() {
                        Ok(_) => current_system.state = SystemState::Running,
                        Err(error) => {
                            current_system.state = SystemState::Loaded;
                            println!("{}", error);
                        }
                    }
                },
                SystemState::Running => {
                    match current_system.update() {
                        Ok(_) => {},
                        Err(error) => {
                            current_system.state = SystemState::Loaded;
                            println!("{}", error);
                        }
                    }
                },
                SystemState::Stopped => {
                    // stop system
                    current_system.on_shutdown();
                    current_system.state = SystemState::Loaded;
                },
                _ => return
            }
        }
    }

    fn draw(&mut self, ctx: &Display) {
        // request a new frame
        let mut target = ctx.draw();

        target.clear_color_and_depth(
            (
                0.0, 
                0.0, 
                0.0, 
                1.0
            ),
            1.0
        );
    
        if let Some(current_system) = self.get_current_system_mut() {
            current_system.frame_starts();
            current_system.draw(ctx, &mut target);
            current_system.frame_ends();
        }

        // ends frame
        target.finish().unwrap();

        // // draw GUI
        // if let Some(cmd) = self.gui.ui(self.window.get_display()) {
        //     // eventually execute a command
        //     cmd.execute(self);
        // }
        // self.gui.paint(self.window.get_display(), &mut target);

        // let next_frame_time = std::time::Instant::now() +
        //     std::time::Duration::from_nanos(16_666_667);
    }

    // fn mouse_motion_event(&mut self, x: f32, y: f32) {
    //     // relays event to the gui
    //     if self.gui.on_mouse_move(x, y) == false {
    //         if let Some(current_system) = self.get_current_system_mut() {
    //             // relays event to the system inputs
    //             if current_system.state == SystemState::Running {
    //                 current_system.on_mouse_move(x, y);
    //             }
    //         }
    //     }
    // }

    // fn mouse_wheel_event(&mut self, x: f32, y: f32) {
    //     // relays event to the gui
    //     if self.gui.on_mouse_wheel(x, y) == false {
    //         // relays event to the system's inputs
    //         if let Some(current_system) = self.get_current_system_mut() {
    //             if current_system.state == SystemState::Running {
    //                 current_system.on_mouse_wheel(x, y);
    //             }
    //         }
    //     }
    // }

    // fn mouse_button_down_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
    //     // relays event to the gui
    //     if self.gui.on_mouse_button_down(button, x, y) == false {
    //         // relays event to the system's inputs
    //         if let Some(current_system) = self.get_current_system_mut() {
    //             if current_system.state == SystemState::Running {
    //                 current_system.on_mouse_button_down(MouseButton::from(button), x, y);
    //             }
    //         }
    //     }
    // }

    // fn mouse_button_up_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
    //     // relays event to the gui
    //     if self.gui.on_mouse_button_up(button, x, y) == false {
    //         // relays event to the system's inputs
    //         if let Some(current_system) = self.get_current_system_mut() {
    //             if current_system.state == SystemState::Running {
    //                 current_system.on_mouse_button_up(MouseButton::from(button), x, y);
    //             }
    //         }
    //     }
    // }

    // fn key_down_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods, repeat: bool) {
    //     // relays event to the gui
    //     if self.gui.on_key_down(keycode, keymods, repeat) == false {
    //         // relays event to the system's inputs
    //         if let Some(current_system) = self.get_current_system_mut() {
    //             if current_system.state == SystemState::Running {
    //                 current_system.on_key_down(Key::from(keycode), repeat);
    //             }
    //         }
    //     }
    // }

    // fn key_up_event(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
    //     // relays event to the gui
    //     if self.gui.on_key_up(keycode, keymods) == false {
    //         // relays event to the system's inputs
    //         if let Some(current_system) = self.get_current_system_mut() {
    //             if current_system.state == SystemState::Running {
    //                 current_system.on_key_up(Key::from(keycode));
    //             }
    //         }
    //     }
    // }

    // fn quit_requested_event(&mut self) {
    //     if self.ctx.prevent_quit_event {
    //         miniquad::window::cancel_quit();
    //         self.ctx.quit_requested = true;s
    //     }
    // }
}
