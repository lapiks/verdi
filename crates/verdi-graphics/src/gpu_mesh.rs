use verdi_database::Resource;

use crate::gpu_assets::GpuAsset;

pub struct GpuMesh {
    pub vertex_buffer: miniquad::BufferId,
    pub index_buffer: miniquad::BufferId,
}

impl GpuMesh {
    pub fn new(vertex_buffer: miniquad::BufferId, index_buffer: miniquad::BufferId) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Resource for GpuMesh {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl GpuAsset for GpuMesh {}