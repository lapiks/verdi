#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        graphics_chip::GraphicsChip,
        bind_graphics_chip::BindGraphicsChip,
        renderer::Renderer,
        render_target::RenderTarget,
    };
}

mod graphics_chip;
mod bind_graphics_chip;
mod vertex;
mod sprite_vertex;
mod render_pass;
mod draw_command;
mod renderer;
mod renderable;
mod render_target;
mod image;
mod mesh;
mod primitive;
mod scene;
mod assets;
mod gpu_assets;
mod gpu_primitive;
mod gpu_image;
mod gpu_program;
mod node;
mod transform;
mod camera;
mod material;
mod uniforms;
mod globals;
mod program;
mod shader;
mod screen_quad;
mod gltf_loader;