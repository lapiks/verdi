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

pub struct DrawCmd2 {
    pub bindings: miniquad::Bindings,
    pub pipeline: miniquad::Pipeline,
    pub uniforms: miniquad::UniformsSource<'static>,
    pub indices_count: u32,
}

impl RenderCmd for DrawCmd {
    fn execute(&self) {
        todo!()
    }
}