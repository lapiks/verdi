pub struct mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Self( vertices, indices )
    }
}
