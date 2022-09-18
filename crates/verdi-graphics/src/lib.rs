#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        graphics_chip::GraphicsChip,
        bind_graphics_chip::BindGraphicsChip,
        renderer::Renderer,
    };
}

mod graphics_chip;
mod bind_graphics_chip;
mod vertex;
mod render_pass;
mod renderer;
mod image;
mod mesh;
mod assets;
mod gpu_assets;