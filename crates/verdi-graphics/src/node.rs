use rlua::UserData;

use crate::{
    mesh::MeshRef, 
    transform::Transform, 
    scene::SceneRef
};

type NodeId = u64;

#[derive(Clone)]
pub struct Node {
    pub mesh: Option<MeshRef>,
    pub transform: Transform,
    pub children: Vec<Node>,
}

#[derive(Clone)]
pub struct NodeRef {
    pub scene: SceneRef,
    pub node_index: NodeId,
}

impl NodeRef {
    pub fn new(scene: SceneRef, node_index: NodeId) -> Self {
        Self {
            scene,
            node_index,
        }
    }
}

impl UserData for NodeRef {}