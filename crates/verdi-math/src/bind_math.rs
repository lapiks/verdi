use glam::{Vec3, Vec2};
use mlua::{Lua, Result};

use crate::{transform::Transform, types::{LuaVec3, LuaVec2}};

pub struct BindMath;

impl<'lua> BindMath {
    fn new_transform() -> Transform {
        Transform::IDENTITY
    }

    fn new_vec2() -> LuaVec2 {
        LuaVec2(Vec2::ZERO)
    }

    fn new_vec3(x: f32, y: f32, z: f32) -> LuaVec3 {
        LuaVec3(Vec3{x, y, z})
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

            let func = lua.create_function(move |_, ()| Ok(BindMath::new_vec2()))?;
            module_table.set("vec2", func)?;

            let func = lua.create_function(move |_, ()| Ok(BindMath::new_vec3(0.0, 0.0, 0.0)))?;
            module_table.set("vec3", func)?;

            let func = lua.create_function(move |_, (x, y, z): (f32, f32, f32)| Ok(BindMath::new_vec3(x, y, z)))?;
            module_table.set("vec3", func)?;
        }

        // add table to globals
        globals.set("math", module_table)?;

        Ok(())
    }
}