use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

implement_vertex!(Vertex, position, uv);

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            uv: [0.0, 0.0],
        }
    }
}