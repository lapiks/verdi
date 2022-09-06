use glium::uniform;
use std::sync::Mutex;
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::{prelude::GraphicsChip, graphics_chip::PrimitiveType};

pub struct BindGraphicsChip;

impl BindGraphicsChip {
    pub fn bind(lua: &Lua, gpu: &'static Mutex<GraphicsChip>) -> Result<()> {
        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
    
            // create graphics module table
            let map_table = lua_ctx.create_table()?;
                    
            // add functions
            let func = lua_ctx.create_function_mut(|_, ()| Ok(gpu.lock().unwrap().end()))?;
            map_table.set("endObject", func)?;
    
            // add table to globals
            globals.set("graphics", map_table)?;
    
            Ok(())
        })?;

        Ok(())
    }
    
}