use verdi_math::prelude::TransformHandle;

use crate::mesh::MeshHandle;

pub trait RenderCmd {
    fn execute(&self);
}

pub struct DrawCmd {
    // pourrait être plus bas niveau : buffers, primitive type, etc..
    // ici ça ne marche que pour un mesh
    pub mesh: MeshHandle,
    pub transform: TransformHandle,
    pub perspective: bool,
}

impl RenderCmd for DrawCmd {
    fn execute(&self) {
        todo!()
    }
}