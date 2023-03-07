use mlua::{Lua, Result};

use crate::{
    audio::AudioHandle, 
    source::SourceHandle
};

pub struct BindAudio;

impl<'lua> BindAudio {
    pub fn bind(lua: &Lua, audio: AudioHandle) -> Result<()> {
        let globals = lua.globals();

        // create world module table
        let module_table = lua.create_table()?;
        
        // add functions
        {
            let audio = audio.clone();
            let func = lua.create_function(move |_, path: String| Ok(audio.new_source(path)))?;
            module_table.set("newSource", func)?;
        }
        {
            let audio = audio.clone();
            let func = lua.create_function(move |_, source: SourceHandle| Ok(audio.play_source(source)))?;
            module_table.set("play", func)?;
        }

        // add table to globals
        globals.set("audio", module_table)?;

        Ok(())
    }
}