use mlua::{Lua, Result};

use crate::{
    world::WorldHandle, 
    entity::EntityId
};

pub struct BindWorld;

impl<'lua> BindWorld {
    pub fn bind(lua: &Lua, world: WorldHandle) -> Result<()> {
        let globals = lua.globals();

        // create world module table
        let module_table = lua.create_table()?;
        
        // add functions
        {
            let world = world.clone();
            let func = lua.create_function(move |_, ()| Ok(world.spawn()))?;
            module_table.set("spawn", func)?;
        }
        {
            let world = world.clone();
            let func = lua.create_function(move |_, entity: EntityId| Ok(world.despawn(entity)))?;
            module_table.set("despawn", func)?;
        }
        {
            let world = world.clone();
            let func = lua.create_function(move |_, entity: EntityId| Ok(world.entity(entity)))?;
            module_table.set("entity", func)?;
        }

        // add table to globals
        globals.set("world", module_table)?;

        Ok(())
    }
}