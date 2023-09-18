#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::app::App;
}

mod window;
mod app;
mod error;
mod gui;
mod toolbar;
mod code_editor;
mod syntax_highlighting;
mod console;
mod commands;
mod app_commands;
mod modeler;
mod viewport;