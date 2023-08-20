use glium::{VertexBuffer, IndexBuffer};
use verdi_database::Resource;

use crate::{vertex::Vertex, gpu_assets::GpuAsset};

pub struct GpuMesh {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: Option<IndexBuffer<u32>>,
}

impl GpuMesh {
    pub fn new(vertex_buffer: VertexBuffer<Vertex>, index_buffer: Option<IndexBuffer<u32>>) -> Self {
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


// impl GpuAsset for GpuMesh {
//     type PreparedResource = GpuMesh;

//     fn prepare_rendering(&mut self, display: &glium::Display, db: &verdi_database::Database) -> Result<Self::PreparedResource, crate::gpu_assets::GpuAssetError> {
//         todo!()
//     }
// }