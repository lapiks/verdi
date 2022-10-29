use std::{path::Path, fs::File, io::Read};

use rlua::{Lua, Result, Function, Table};

pub struct LuaContext {}

impl LuaContext {
    pub fn load_scripts<P: AsRef<Path>>(lua: &Lua, folder: P) -> Result<()> {
        let paths = std::fs::read_dir(folder).unwrap();
        let mut scripts = Vec::default();

        for path in paths {
            let path = path.unwrap().path();
    
            match path.extension() {
                Some(p) if p == "lua" => {
                    println!("Loading script {:?}", path.file_name().unwrap());
                    scripts.push(
                        LuaContext::load_script(path).unwrap()
                    )
                },
                _ => (),
            }
        }

        let boot_lua = LuaContext::load_script("./crates/verdi-app/src/boot.lua").unwrap();
        let run_lua = LuaContext::load_script("./crates/verdi-app/src/run.lua").unwrap();

        lua.context(|lua_ctx| {   
            let globals = lua_ctx.globals();
            
            // create verdi table
            let verdi_table = lua_ctx.create_table()?;
            globals.set("verdi", verdi_table)?;

            // load game scripts
            for script_code in scripts.iter() {
                lua_ctx.load(&script_code).eval::<()>()?;
            }
    
            // load boot code
            lua_ctx.load(&boot_lua).eval::<()>()?;

            // load run code
            lua_ctx.load(&run_lua).eval::<()>()?;

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

    fn load_script<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
        // todo : gestion d'erreur
        let mut f = File::open(path)?;
        let mut content: String = String::new();
        f.read_to_string(&mut content)?;
        
        Ok(content)
    }
}