use glium::implement_vertex;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2]
}

implement_vertex!(Vertex, position, normal, uv, color);

impl Default for Vertex {
    fn default() -> Self {
        Self { 
            position: [0.0, 0.0, 0.0], 
            normal: [0.0, 0.0, 1.0], 
            color: [1.0, 1.0, 1.0, 1.0], 
            uv: [0.0, 0.0],
        }
    }
} 