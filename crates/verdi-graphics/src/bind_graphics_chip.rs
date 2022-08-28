use glium::uniform;
use std::sync::Arc;
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::{prelude::GraphicsChip, graphics_chip::PrimitiveType};

pub fn bind(lua: &mut Lua, gpu: &'static GraphicsChip) -> Result<()> {
    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        // let begin = lua_ctx.create_function(|_, primitive_type: PrimitiveType| Ok(gpu.begin(primitive_type)))?;
        // globals.set("begin", begin)?;

        let end = lua_ctx.create_function(|_, ()| Ok(gpu.end()))?;
        globals.set("endObject", end)?;
        Ok(())
    })?;

    Ok(())
}