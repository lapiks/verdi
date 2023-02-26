use mlua::{Lua, Result};

use crate::{
    world::{WorldHandle}, 
    entity::EntityRef
};

pub struct BindWorld;

impl<'lua> BindWorld {
    fn spawn(world: WorldHandle) -> EntityRef {
        world.spawn()
    }

    pub fn bind(lua: &Lua, world: WorldHandle) -> Result<()> {
        let globals = lua.globals();

        // create world module table
        let module_table = lua.create_table()?;
        
        // add functions
        {
            let func = lua.create_function(move |_, ()| Ok(BindWorld::spawn(world.clone())))?;
            module_table.set("spawn", func)?;
        }

        // add table to globals
        globals.set("world", module_table)?;

        Ok(())
    }
}