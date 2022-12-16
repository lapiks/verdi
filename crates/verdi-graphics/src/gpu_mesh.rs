use glium::{VertexBuffer, IndexBuffer};

use crate::vertex::Vertex;

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