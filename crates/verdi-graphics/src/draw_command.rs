use crate::{
    vertex::Vertex, 
    graphics_chip::PrimitiveType, 
    material::MaterialId
};

type VertexBuffer = Vec<Vertex>;
type IndexBuffer = Vec<u32>;

pub struct DrawCommand {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: Option<IndexBuffer>,
    pub primitive_type: PrimitiveType,
    pub material: MaterialId,
}