use crate::{  
    transform::Transform, 
    mesh::MeshId,
};

pub struct RenderPass {
    // plutôt node qui contient mesh + transform ?
    pub mesh_id: MeshId,
    pub transform: Transform
}
