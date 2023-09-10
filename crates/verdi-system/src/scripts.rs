use std::{collections::HashMap, path::{PathBuf, Path}, fs::OpenOptions, io::Write, time::Duration};

use mlua::Lua;
use verdi_utils::{read_at_path, make_relative_path};

use thiserror::Error;

use crate::{
    file_watcher::{
        FileWatcher, 
        FileWatcherError
    }, 
    lua_context::LuaContext
};

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("Not a lua file")]
    NotLuaError(),
    #[error("Parsing error")]
    ParsingError(#[from] std::io::Error),
    #[error("Cannot evaluate lua code")]
    LuaError(#[from] mlua::Error),
    #[error("File watcher initialisation failed")]
    FileWatcherError(#[from] FileWatcherError),
}

pub struct Scripts {
    scripts: HashMap<PathBuf, Script>,
    file_watcher: Option<FileWatcher>,
}

impl Scripts {
    pub fn new() -> Result<Self, ScriptError> {
        Ok(Self {
            scripts: HashMap::default(),
            file_watcher: None,
        })
    }

    pub fn add_script(&mut self, script: Script) {
        self.scripts.insert(
            script.path.clone(),
            script,
        );
    }

    pub fn load_dir<P: AsRef<Path>>(&mut self, dir_path: P) -> Result<(), ScriptError>  {
        let paths = std::fs::read_dir(dir_path).unwrap();

        for path in paths {
            let file_path = path?.path();
            if let Ok(script) = Scripts::load_file(file_path) {
                self.add_script(script)
            }
        }

        Ok(())
    }

    pub fn add_file_watcher<P: AsRef<Path>>(&mut self, dir_path: P) -> Result<(), ScriptError> {
        self.file_watcher = Some(FileWatcher::new(dir_path, Duration::from_secs(5))?);

        Ok(())
    }

    pub fn load_file<P: AsRef<Path>>(file_path: P) -> Result<Script, ScriptError>  {
        match &file_path.as_ref().extension() {
            Some(p) if *p == "lua" => {
                println!("Loading script {:?}", file_path.as_ref().file_name().unwrap());
                return Ok(Script::new(file_path)?);
            },
            _ => (),
        }

        Err(ScriptError::NotLuaError())
    }

    pub fn save_script(&mut self, file_path: &PathBuf) -> Result<(), ScriptError> {
        if let Some(script) = self.get_script_mut(file_path) {
            script.save_at(file_path)?;
        }

        Ok(())
    }

    pub fn hot_reload(&mut self, lua: &Lua) -> Result<(), ScriptError> {
        if let Some(file_watcher) = &self.file_watcher {
            if let Some(watcher_event) = file_watcher.get_event() {
                if let notify::EventKind::Modify(_) = watcher_event.kind {
                    for path in watcher_event.paths.iter() {
                        if let Ok(relative_path) = make_relative_path(path) {
                            if let Some(script) = self.scripts.get_mut(&relative_path) {
                                // reload script
                                script.reload_from(relative_path)?;
    
                                // update lua context
                                LuaContext::load_script(
                                    lua, 
                                    script
                                )?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn get_scripts(&self) -> &HashMap<PathBuf, Script> {
        &self.scripts
    }

    pub fn get_scripts_mut(&mut self) -> &mut HashMap<PathBuf, Script> {
        &mut self.scripts
    }

    pub fn get_script(&self, path: &PathBuf) -> Option<&Script> {
        self.scripts.get(path)
    }

    pub fn get_script_mut(&mut self, path: &PathBuf) -> Option<&mut Script> {
        self.scripts.get_mut(path)
    }
}

pub struct Script {
    pub code: String,
    pub path: PathBuf,
}

impl Script {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self>  {
        Ok(
            Self {
                code: read_at_path(path.as_ref().clone())?,
                path: path.as_ref().to_path_buf(),
            }
        )
    }

    pub fn save_at<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        f.write_all(&self.code.as_bytes())
    }

    pub fn reload_from<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        self.code = read_at_path(path)?;

        Ok(())
    }
}