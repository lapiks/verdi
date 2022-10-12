use glium::{
    VertexBuffer, 
    IndexBuffer, 
    DrawParameters
};

use crate::material::UniformValues;

pub struct Renderable<'a, T> where T: Copy {
    pub vertex_buffer: &'a VertexBuffer<T>,
    pub index_buffer: Option<&'a IndexBuffer<u32>>,
    pub program: &'a glium::Program,
    pub uniform_values: UniformValues<'a>,
    pub draw_parameters: DrawParameters<'a>,
}