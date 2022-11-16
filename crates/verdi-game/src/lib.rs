#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        scripts::{Scripts, Script},
        game::Game,
    };
}

mod scripts;
mod game;