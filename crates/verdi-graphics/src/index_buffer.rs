pub struct IndexBuffer {
    internal_buffer: glium::IndexBuffer<u32>,
}

impl IndexBuffer {
    pub fn new(display: &glium::Display, indices: &[u32]) -> Self {
        Self { 
            internal_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, indices ).unwrap() 
        }
    }
}
