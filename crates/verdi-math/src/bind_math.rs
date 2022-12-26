use rlua::{Lua, Result};

use crate::transform::TransformRef;

pub struct BindMath;

impl<'lua> BindMath {
    fn new_transform() -> TransformRef {
        TransformRef {}
    }

    pub fn bind(lua: &Lua) -> Result<()> {
        lua.context(move |lua_ctx| {
            let globals = lua_ctx.globals();
    
            // create inputs module table
            let module_table = lua_ctx.create_table()?;
            
            // add functions
            {
                //let inputs = inputs.clone();
                let func = lua_ctx.create_function(move |_, key: String| Ok(BindMath::new_transform()))?;
                module_table.set("newTransform", func)?;
            }

            // add table to globals
            globals.set("math", module_table)?;
    
            Ok(())
        })?;

        Ok(())
    }
}