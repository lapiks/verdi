use verdi_math::prelude::Transform;

use crate::mesh::MeshId;

pub trait RenderCmd {
    fn execute(&self);
}

pub struct DrawCmd {
    // pourrait être plus bas niveau : buffers, primitive type, etc..
    // ici ça ne marche que pour un mesh
    pub mesh: MeshId,
    pub transform: Transform,
    pub perspective: bool,
}

impl RenderCmd for DrawCmd {
    fn execute(&self) {
        todo!()
    }
}