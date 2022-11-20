use std::{rc::Rc, cell::RefCell, path::{Path, PathBuf}, time::Duration};

use rlua::Lua;
use verdi_utils::make_relative_path;

use crate::{
    lua_context::LuaContext, 
    prelude::Scripts, 
    time_step::TimeStep, file_watcher::FileWatcher
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Reading lua script failed")]
    ReadLuaScriptFailed(#[from] std::io::Error),
    #[error("Cannot evaluate lua code")]
    LuaError(#[from] rlua::Error),
    #[error("File watcher error")]
    FileWatcherError(#[from] notify::Error),
}

#[derive(PartialEq)]
pub enum GameState {
    Start,
    Running,
    Paused,
    Stopped,
}

pub struct Game {
    path: PathBuf,
    pub state: GameState,
    scripts: Rc<RefCell<Scripts>>,
    file_watcher: FileWatcher,
    pub time_step: TimeStep,
    last_error: String,
}

impl Game {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, GameError> {
        Ok(Self { 
            path: path.as_ref().to_path_buf(),
            state: GameState::Stopped,
            scripts: Rc::new(RefCell::new(Scripts::new())),
            file_watcher: FileWatcher::new(path, Duration::from_secs(5))?,
            time_step: TimeStep::new(),
            last_error: String::new(),
        })
    }

    pub fn load(&mut self) -> Result<(), GameError> {
        Ok(self.scripts.borrow_mut().load_dir(&self.path)?)
    }

    // called at the start of the game execution
    pub fn boot(&mut self, lua: &Lua) -> Result<(), GameError>{
        LuaContext::create_verdi_table(lua)?;
        LuaContext::load_internal_scripts(lua)?;
        LuaContext::load_scripts(lua, &self.scripts.borrow())?;
        LuaContext::call_boot(lua)?;

        Ok(())
    }

    pub fn run(&mut self, lua: &Lua) {
        let delta_time = self.time_step.tick();
        
        // script hot-reload
        if let Some(watcher_event) = self.file_watcher.get_event() {
            if let notify::EventKind::Modify(_) = watcher_event.kind {
                for path in watcher_event.paths.iter() {
                    if let Ok(relative_path) = make_relative_path(path) {
                        if let Some(script) = self.scripts.borrow_mut().get_script_mut(&relative_path) {
                            // reload script
                            script
                                .reload_from(relative_path)
                                .expect("Reload script file failed");

                            // update lua context
                            LuaContext::load_script(
                                lua, 
                                script
                            ).expect("Reload script failed");
                        }
                    }
                }
            }
        }

        // callbacks
        if let Err(err) = LuaContext::call_run(lua, delta_time) {
            let current_error = err.to_string();
            if self.last_error != current_error {
                println!("{}", err);
                self.last_error = current_error;
            }
        }
    }

    pub fn get_scripts(&self) -> Rc<RefCell<Scripts>> {
        self.scripts.clone()
    }
}