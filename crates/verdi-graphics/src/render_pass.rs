use crate::{vertex::Vertex, graphics_chip::PrimitiveType, image::ImageRef, node::Node};

pub struct RenderPass {
    pub node: Node,
    pub current_vertex_state: Vertex,
    pub current_primitive: PrimitiveType,
    pub current_texture: Option<ImageRef>,
}

impl RenderPass {
    pub fn new(node: Node, current_primitive: PrimitiveType) -> Self {
        Self {
            node,
            current_vertex_state: Vertex::default(),
            current_primitive,
            current_texture: Option::default()       
        }
    }
}

// impl Default for RenderPass {
//     fn default() -> Self {
//         Self {
//             mesh: Option::default(),
//             vertex_buffer: Vec::new(),
//             current_vertex_state: Vertex::default(),
//             current_primitive: PrimitiveType::Triangles,
//             current_texture: Option::default()        
//         }
//     }
// }