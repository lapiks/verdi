#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        gui::Gui
    };
}

mod gui;
mod code_editor;
mod syntax_highlighting;
mod console;
mod commands;
mod toolbar;