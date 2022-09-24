use crate::{transform::Transform, mesh::MeshRef};

#[derive(Clone)]
pub struct Node {
    pub mesh: Option<MeshRef>,
    pub transform: Transform,
    pub children: Vec<Node>,
}