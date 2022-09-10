use glium::texture::SrgbTexture2d;
use crate::{vertex::Vertex, graphics_chip::PrimitiveType, image::Image};

pub struct RenderPass {
    pub vertex_buffer: Vec<Vertex>,
    pub current_vertex_state: Vertex,
    pub current_primitive: PrimitiveType,
    pub current_texture: Option<SrgbTexture2d>,
}

impl RenderPass {
    pub fn new(current_primitive: PrimitiveType) -> Self {
        Self {
            current_primitive: current_primitive,
            .. RenderPass::default()
        }
    }
}

impl Default for RenderPass {
    fn default() -> Self {
        Self {
            vertex_buffer: Vec::new(),
            current_vertex_state: Vertex::default(),
            current_primitive: PrimitiveType::triangles,
            current_texture: Option::default()        
        }
    }
}