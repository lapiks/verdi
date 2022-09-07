use glium::uniform;
use std::sync::Mutex;
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::{prelude::GraphicsChip, graphics_chip::PrimitiveType};

pub struct BindGraphicsChip;

impl BindGraphicsChip {
    fn beginObject(gpu: &Mutex<GraphicsChip>, primitive_type: String) {
        let mut enum_val = PrimitiveType::triangles;
        if primitive_type == "triangles" { enum_val = PrimitiveType::triangles; }
        else if primitive_type == "points" { enum_val = PrimitiveType::points; }
        else if primitive_type == "lines" { enum_val = PrimitiveType::lines; }

        gpu.lock().unwrap().begin(enum_val);
    }

    fn endObject(gpu: &Mutex<GraphicsChip>) {
        gpu.lock().unwrap().end();
    }

    pub fn bind(lua: &Lua, gpu: &'static Mutex<GraphicsChip>) -> Result<()> {
        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
    
            // create graphics module table
            let module_table = lua_ctx.create_table()?;
            
            // add functions
            {
                let func = lua_ctx.create_function_mut(|_, primitive_type: String| Ok(BindGraphicsChip::beginObject(gpu, primitive_type)))?;
                module_table.set("beginObject", func)?;
            }
            {
                let func = lua_ctx.create_function_mut(|_, ()| Ok(BindGraphicsChip::endObject(gpu)))?;
                module_table.set("endObject", func)?;
            }
            
    
            // add table to globals
            globals.set("graphics", module_table)?;
    
            Ok(())
        })?;

        Ok(())
    }
    
}