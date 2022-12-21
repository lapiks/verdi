use verdi_math::prelude::Transform;

use crate::{  
    mesh::MeshId,
};

pub struct RenderPass {
    // plutôt node qui contient mesh + transform ?
    pub mesh_id: MeshId,
    pub transform: Transform
}
