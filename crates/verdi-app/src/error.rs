use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Reading lua script failed")]
    ReadLuaScriptFailed(#[from] std::io::Error),
    #[error("Cannot evaluate lua code")]
    LuaError(#[from] mlua::Error),
}