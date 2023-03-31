use mlua::{Lua, Result, Function, Table};
use verdi_graphics::prelude::PassHandle;

use crate::prelude::{Scripts, Script};

// a sortir de la crate game ?
pub struct LuaContext {}

impl LuaContext {
    pub fn create_verdi_table(lua: &Lua) -> Result<()> {
        let globals = lua.globals();

        // create verdi table
        let verdi_table = lua.create_table()?;
        globals.set("verdi", verdi_table)?;

        Ok(())
    }

    pub fn load_internal_scripts(lua: &Lua) -> Result<()> {
        let boot_script = Scripts::load_file("crates/verdi-system/src/boot.lua").expect("Unable to load boot.lua file");
        let run_script = Scripts::load_file("crates/verdi-system/src/run.lua").expect("Unable to load run.lua file");

        LuaContext::load_script(&lua, &boot_script)?;
        LuaContext::load_script(&lua, &run_script)?;

        Ok(())
    }

    pub fn load_scripts(lua: &Lua, scripts: &Scripts) -> Result<()> {
        // load lua scripts
        for script in scripts.get_scripts().iter() {
            LuaContext::load_script(lua, script.1)?;
        }

        Ok(())
    }

    pub fn load_script(lua: &Lua, script: &Script) -> Result<()> {
        // load lua script
        lua.load(&script.code).eval::<()>()?;

        Ok(())
    }

    pub fn call_boot(lua: &Lua) -> Result<()> {
        // run callbacks
        lua.load("verdi.boot()").exec()?;

        Ok(())
    }

    pub fn call_run(lua: &Lua, delta_time: f32, pass: PassHandle) -> Result<()> {
        let globals = lua.globals();
        let verdi_table: Table = globals.get("verdi")?;

        // run callbacks
        let run_func: Function = verdi_table.get("run")?;
        run_func.call::<_,()>((delta_time, pass))?;

        Ok(())
    }
}