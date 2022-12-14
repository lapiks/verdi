#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        scripts::{Scripts, Script},
        game::{Game, GameError},
    };
}

mod game;
mod time_step;
mod scripts;
mod lua_context;
mod file_watcher;