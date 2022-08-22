use crate::{vertex::Vertex};

pub struct VertexBuffer {
    pub internal_buffer: glium::VertexBuffer<Vertex>,
}

impl VertexBuffer {
    pub fn new(display: &glium::Display, shape: &[Vertex]) -> Self {
        Self { 
            internal_buffer: glium::VertexBuffer::new(display, shape).unwrap() 
        }
    }
}
