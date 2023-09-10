#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        graphics_chip::GraphicsChip,
        bind_graphics_chip::BindGraphicsChip,
        renderer::Renderer,
        render_target::RenderTarget,
        globals::Globals,
        pass::PassHandle,
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
mod model;
mod assets;
mod gpu_assets;
mod gpu_mesh;
mod gpu_image;
mod gpu_program;
mod gpu_pipeline;
mod node;
mod camera;
mod material;
mod uniform;
mod globals;
mod program;
mod shader;
mod gltf_loader;
mod render_state;
mod pass;
mod pipeline;
mod render_cmds;
mod render_graph;
mod sprite;