use crate::{vertex::Vertex, graphics_chip::PrimitiveType};

pub struct RenderPass {
    pub vertex_buffer: Vec<Vertex>,
    pub current_vertex_state: Vertex,
    pub current_primitive: PrimitiveType
}

impl RenderPass {
    pub fn new(vertex_buffer: Vec<Vertex>, current_vertex_state: Vertex, current_primitive: PrimitiveType) -> Self {
        Self { 
            vertex_buffer, 
            current_vertex_state,
            current_primitive
        }
    }
}