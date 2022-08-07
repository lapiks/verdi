mod renderer;
mod render_pipeline;
mod render_device;
mod vertex;
mod mesh;

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        mesh::Mesh,
        vertex::Vertex,
    };
}

pub use renderer::*;