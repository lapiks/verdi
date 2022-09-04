// use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

// pub struct Module {
//     pub name: String,
//     pub functions: Vec<fn()>
// }

// pub fn register_module(lua: &Lua, module: &Module) -> Result<()> {
//     lua.context(|lua_ctx| {
//         let globals = lua_ctx.globals();

//         // create module table
//         let map_table = lua_ctx.create_table()?;

//         // add functions
//         // for function in module.functions {
//         //     let func = lua_ctx.create_function_mut(|_, ()| Ok(function))?;
//         //     map_table.set(func_name, func)?;
//         // }

                
//         let func = lua_ctx.create_function_mut(|_, ()| Ok(function))?;
//         map_table.set();

//         // add table to globals
//         globals.set(module.name, map_table)?;

//         Ok(())
//     })?;
// }