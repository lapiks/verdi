use mlua::{Lua, Result, prelude::LuaValue, Table, AnyUserData};

use crate::{
    world::WorldHandle, 
    entity::EntityId
};

pub struct BindWorld;

impl<'lua> BindWorld {
    fn register_component(world: WorldHandle, value: LuaValue) {
        match value {
            LuaValue::Nil => (),
            LuaValue::Boolean(_) => world.register_component::<bool>(),
            LuaValue::LightUserData(_) => todo!(),
            LuaValue::Integer(_) => world.register_component::<u64>(),
            LuaValue::Number(_) => todo!(),
            LuaValue::String(_) => world.register_component::<String>(),
            LuaValue::Table(_) => world.register_component::<Table>(),
            LuaValue::Function(_) => todo!(),
            LuaValue::Thread(_) => todo!(),
            LuaValue::UserData(_) => world.register_component::<AnyUserData>(),
            LuaValue::Error(_) => todo!(),
        }
    }

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
        {
            let world = world.clone();
            let func = lua.create_function(move |_, value: LuaValue| Ok(BindWorld::register_component(world.clone(), value)))?;
            module_table.set("newComponent", func)?;
        }

        // add table to globals
        globals.set("world", module_table)?;

        Ok(())
    }
}