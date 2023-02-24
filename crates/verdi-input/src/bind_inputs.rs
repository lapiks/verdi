use std::{rc::Rc, cell::RefCell};

use mlua::{Lua, Result};

use crate::inputs::{Inputs, Key, MouseButton};

pub struct BindInputs;

impl<'lua> BindInputs {
    fn get_key_down(inputs: &Inputs, key: &String) -> bool {
        inputs.get_key_down(Key::from(key.clone()))
    }

    fn get_button_down(inputs: &Inputs, button: &String) -> bool {
        inputs.get_button_down(MouseButton::from(button.clone()))
    }

    fn get_mouse_delta(inputs: &Inputs) -> (f32, f32) {
        let delta = inputs.get_mouse_delta();
        (delta.x, delta.y)
    }

    pub fn bind(lua: &Lua, inputs: Rc<RefCell<Inputs>>) -> Result<()> {
        let globals = lua.globals();

        // create inputs module table
        let module_table = lua.create_table()?;
        
        // add functions
        {
            let inputs = inputs.clone();
            let func = lua.create_function(move |_, key: String| Ok(BindInputs::get_key_down(&inputs.borrow(), &key)))?;
            module_table.set("getKeyDown", func)?;
        }
        {
            let inputs = inputs.clone();
            let func = lua.create_function(move |_, button: String| Ok(BindInputs::get_button_down(&inputs.borrow(), &button)))?;
            module_table.set("getButtonDown", func)?;
        }
        {
            let inputs = inputs.clone();
            let func = lua.create_function(move |_, ()| Ok(BindInputs::get_mouse_delta(&inputs.borrow())))?;
            module_table.set("getMouseDelta", func)?;
        }

        // add table to globals
        globals.set("input", module_table)?;

        Ok(())
    }
}