use std::sync::{Mutex, Arc};

use rlua::{Lua, Result};

use crate::inputs::{Inputs, Key, MouseButton};

pub struct BindInputs;

impl<'lua> BindInputs {
    fn get_key_down(inputs: Arc<Mutex<Inputs>>, key: &String) -> bool {
        inputs.lock().unwrap().get_key_down(Key::from(key.clone()))
    }

    fn get_button_down(inputs: Arc<Mutex<Inputs>>, button: &String) -> bool {
        inputs.lock().unwrap().get_button_down(MouseButton::from(button.clone()))
    }

    pub fn bind(lua: &Lua, inputs: Arc<Mutex<Inputs>>) -> Result<()> {
        lua.context(move |lua_ctx| {
            let globals = lua_ctx.globals();
    
            // create inputs module table
            let module_table = lua_ctx.create_table()?;
            
            // add functions
            {
                let inputs = inputs.clone();
                let func = lua_ctx.create_function(move |_, key: String| Ok(BindInputs::get_key_down(inputs.clone(), &key)))?;
                module_table.set("getKeyDown", func)?;
            }
            {
                let inputs = inputs.clone();
                let func = lua_ctx.create_function(move |_, button: String| Ok(BindInputs::get_button_down(inputs.clone(), &button)))?;
                module_table.set("getButtonDown", func)?;
            }

            // add table to globals
            globals.set("input", module_table)?;
    
            Ok(())
        })?;

        Ok(())
    }
}