use mlua::{Lua, Result};

use crate::audio::AudioHandle;

pub struct BindAudio;

impl<'lua> BindAudio {
    pub fn bind(lua: &Lua, audio: AudioHandle) -> Result<()> {
        let globals = lua.globals();

        // create world module table
        let module_table = lua.create_table()?;
        
        // add functions
        {
            let audio = audio.clone();
            let func = lua.create_function(move |_, path: String| Ok(audio.new_clip(path)))?;
            module_table.set("newClip", func)?;
        }

        // add table to globals
        globals.set("audio", module_table)?;

        Ok(())
    }
}