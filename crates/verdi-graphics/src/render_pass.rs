use verdi_math::Mat4;

use crate::{mesh::MeshHandle, uniform::UniformHandle};

/// A render command defining what to draw.
pub struct RenderPass {
    // plutôt node qui contient mesh + transform ?
    pub mesh: MeshHandle,
    pub transform_matrix: UniformHandle<Mat4>
}
