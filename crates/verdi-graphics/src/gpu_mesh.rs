use glium::{VertexBuffer, IndexBuffer};
use verdi_database::Resource;

use crate::{gpu_assets::GpuAsset, vertex::Vertex};

pub struct GpuMesh {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: Option<IndexBuffer<u32>>,
}

impl GpuMesh {
    pub fn new(vertex_buffer: VertexBuffer<Vertex>, index_buffer: Option<IndexBuffer<u32>>) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn get_vertex_buffer(&self) -> &VertexBuffer<Vertex> {
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &Option<IndexBuffer<u32>> {
        &self.index_buffer
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