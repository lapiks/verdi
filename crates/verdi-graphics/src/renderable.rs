use glium::{
    VertexBuffer, 
    IndexBuffer, 
    DrawParameters
};

use crate::material::MaterialRef;

pub struct Renderable<'a, T> where T: Copy {
    pub vertex_buffer: &'a VertexBuffer<T>,
    pub index_buffer: Option<&'a IndexBuffer<u32>>,
    pub program: &'a glium::Program,
    pub material_ref: MaterialRef<'a>,
    pub draw_parameters: DrawParameters<'a>,
}