use glium::uniform;
use std::sync::Mutex;
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::{prelude::GraphicsChip, graphics_chip::PrimitiveType};

pub struct BindGraphicsChip;

impl BindGraphicsChip {
    pub fn bind(lua: &Lua, gpu: &'static Mutex<GraphicsChip>) -> Result<()> {
        // lua.context(|lua_ctx| {
        //     let globals = lua_ctx.globals();
    
        //     // let begin = lua_ctx.create_function(|_, primitive_type: PrimitiveType| Ok(gpu.lock().unwrap().begin(primitive_type)))?;
        //     // globals.set("beginObject", begin)?;
    
        //     let end = lua_ctx.create_function_mut(|_, ()| Ok(gpu.lock().unwrap().end()))?;
        //     globals.set("endObject", end)?;
        //     Ok(())
        // })?;
    
        // Ok(())

        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
    
            // create module table
            let map_table = lua_ctx.create_table()?;
    
            // add functions
            // for function in module.functions {
            //     let func = lua_ctx.create_function_mut(|_, ()| Ok(function))?;
            //     map_table.set(func_name, func)?;
            // }
    
                    
            let func = lua_ctx.create_function_mut(|_, ()| Ok(gpu.lock().unwrap().end()))?;
            map_table.set("endObject", func)?;
    
            // add table to globals
            globals.set("graphics", map_table)?;
    
            Ok(())
        })?;

        Ok(())
    }
    
}