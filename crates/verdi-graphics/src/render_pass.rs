use crate::{program::Program, vertex_buffer::VertexBuffer, index_buffer::IndexBuffer};

pub struct RenderPass<'a> {
    pub program: &'a Program,
    pub vertex_buffer: &'a VertexBuffer,
    pub index_buffer: &'a IndexBuffer
}

impl<'a> RenderPass<'a> {
    pub fn new(program: &'a Program, vertex_buffer: &'a VertexBuffer, index_buffer: &'a IndexBuffer) -> Self {
        Self {
            program, 
            vertex_buffer, 
            index_buffer
        }
    }
}