use verdi_math::prelude::Transform;

use crate::mesh::MeshId;

pub trait RenderCmd {
    
}

pub struct DrawCmd {
    pub mesh: MeshId,
    pub transform: Transform,
}

impl RenderCmd for DrawCmd {
    
}