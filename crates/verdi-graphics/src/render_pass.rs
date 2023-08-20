use verdi_math::prelude::TransformHandle;

use crate::mesh::MeshHandle;

/// A render command defining what to draw.
pub struct RenderPass {
    // plutôt node qui contient mesh + transform ?
    pub mesh: MeshHandle,
    pub transform: TransformHandle
}
