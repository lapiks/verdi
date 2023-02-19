use mlua::{Lua, Result};

use crate::transform::Transform;

pub struct BindMath;

impl<'lua> BindMath {
    fn new_transform() -> Transform {
        Transform::IDENTITY
    }

    pub fn bind(lua: &Lua) -> Result<()> {
        let globals = lua.globals();

        // create inputs module table
        let module_table = lua.create_table()?;
        
        // add functions
        {
            //let inputs = inputs.clone();
            let func = lua.create_function(move |_, ()| Ok(BindMath::new_transform()))?;
            module_table.set("newTransform", func)?;
        }

        // add table to globals
        globals.set("math", module_table)?;

        Ok(())
    }
}