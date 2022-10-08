use std::sync::Mutex;

use rlua::{Lua, Result};

use crate::inputs::{Inputs, Key};

pub struct BindInputs;

impl<'lua> BindInputs {
    fn get_key_down(inputs: &Mutex<Inputs>, key: &String) -> bool {
        inputs.lock().unwrap().get_key_down(Key::from(key.clone()))
    }

    pub fn bind(lua: &Lua, inputs: &'static Mutex<Inputs>) -> Result<()> {
        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
    
            // create inputs module table
            let module_table = lua_ctx.create_table()?;
            
            // add functions
            {
                let func = lua_ctx.create_function(|_, key: String| Ok(BindInputs::get_key_down(inputs, &key)))?;
                module_table.set("getKeyDown", func)?;
            }

            // add table to globals
            globals.set("input", module_table)?;
    
            Ok(())
        })?;

        Ok(())
    }
}