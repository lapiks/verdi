#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        graphics_chip::GraphicsChip,
        bind_graphics_chip::BindGraphicsChip,
        renderer::Renderer,
        global_shaders::GlobalShaders,
    };
}

mod graphics_chip;
mod bind_graphics_chip;
mod vertex;
mod render_pipeline;
mod render_pass;
mod renderer;
mod image;
mod mesh;
mod scene;
mod assets;
mod gpu_assets;
mod gpu_mesh;
mod gpu_image;
mod node;
mod transform;
mod camera;
mod material;
mod uniforms;
mod global_shaders;
mod program;
mod shader;