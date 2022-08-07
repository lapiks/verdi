use crate::vertex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Self { vertices, indices }
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex>) {
        self.vertices = vertices;
    }

    pub fn set_indices(&mut self, indices: Vec<u16>) {
        self.indices = indices;
    }
}
