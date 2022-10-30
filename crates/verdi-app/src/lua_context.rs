use rlua::{Lua, Result, Function, Table};

use verdi_game::prelude::*;

pub struct LuaContext {}

impl LuaContext {
    pub fn load_scripts(lua: &Lua, scripts: &Scripts) -> Result<()> {
        lua.context(|lua_ctx| {   
            let globals = lua_ctx.globals();

            // create verdi table
            let verdi_table = lua_ctx.create_table()?;
            globals.set("verdi", verdi_table)?;

            // load lua scripts
            for script in scripts.get_scripts().iter() {
                LuaContext::load_script(lua, script.1)?;
            }

            Ok(())
        })?;

        Ok(())
    }

    pub fn load_script(lua: &Lua, script: &Script) -> Result<()> {
        lua.context(|lua_ctx| {   
            // load lua script
            lua_ctx.load(&script.code).eval::<()>()?;

            Ok(())
        })?;

        Ok(())
    }

    pub fn call_boot(lua: &Lua) -> Result<()> {
        lua.context(|lua_ctx| {
            // run callbacks
            lua_ctx.load("verdi.boot()").exec()?;

            Ok(())
        })?;

        Ok(())
    }

    pub fn call_run(lua: &Lua, delta_time: f32) -> Result<()> {
        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
            let verdi_table: Table = globals.get("verdi")?;

            // run callbacks
            let run_func: Function = verdi_table.get("run")?;
            run_func.call::<_,()>(delta_time)?;

            Ok(())
        })?;

        Ok(())
    }
}