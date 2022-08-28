use glium::uniform;
use std::sync::Mutex;
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::{prelude::GraphicsChip, graphics_chip::PrimitiveType};

pub struct BindGraphicsChip;

impl BindGraphicsChip {
    pub fn bind(lua: &Lua, gpu: &'static Mutex<GraphicsChip>) -> Result<()> {
        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
    
            // let begin = lua_ctx.create_function(|_, primitive_type: PrimitiveType| Ok(gpu.begin(primitive_type)))?;
            // globals.set("beginObject", begin)?;
    
            let end = lua_ctx.create_function_mut(|_, ()| Ok(gpu.lock().unwrap().end()))?;
            globals.set("endObject", end)?;
            Ok(())
        })?;
    
        Ok(())
    }
}