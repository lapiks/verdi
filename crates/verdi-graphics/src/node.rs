use std::sync::{Arc, Mutex};

use rlua::{UserData, UserDataMethods};

use verdi_math::prelude::Transform;

use crate::{
    scene::SceneRef, 
    mesh::MeshId, 
    prelude::GraphicsChip
};

type NodeId = u64;

#[derive(Clone)]
pub struct Node {
    pub mesh: Option<MeshId>,
    pub transform: Transform,
    pub children: Vec<Node>,
}

impl Node {
    pub fn draw(&self, gpu: Arc<Mutex<GraphicsChip>>) {
        gpu.lock().unwrap().draw_node(&self);
    }
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

    pub fn draw(&self) {
        let gpu = self.scene.gpu.lock().unwrap();
        let scene = gpu.assets.get_scene(self.scene.id).unwrap();
        let node = scene.get_node(self.node_index).unwrap();

        let mut gpu_mut = self.scene.gpu.lock().unwrap();
        gpu_mut.draw_node(node);
    }
}

impl UserData for NodeRef {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("draw", |_, node, ()| {
            Ok(node.draw())
        });
    }
}