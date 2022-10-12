use std::sync::{Mutex, Arc};

use rlua::{UserData, UserDataMethods};
use slotmap::{new_key_type, Key};

use crate::{
    node::{Node, NodeRef}, 
    graphics_chip::GraphicsChip
};

new_key_type! {
    pub struct SceneId;
}

#[derive(Clone)]
pub struct Scene {
    pub nodes: Vec<Node>,
    pub id: SceneId,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            id: SceneId::null(),
        }
    }

    pub fn get_node(&self, index: u64) -> Option<&Node> {
        self.nodes.get(index as usize)
    }

    pub fn draw(&self, gpu: Arc<Mutex<GraphicsChip>>) {
        for node in self.nodes.iter() {
            node.draw(gpu.clone());
        }
    }
}

#[derive(Clone)]
pub struct SceneRef {
    pub gpu: Arc<Mutex<GraphicsChip>>,
    pub id: SceneId,
}

impl SceneRef {
    pub fn new(gpu: Arc<Mutex<GraphicsChip>>, id: SceneId) -> Self{
        Self {
            gpu,
            id
        }
    }

    pub fn draw(&self) {
        self.gpu.lock().unwrap().draw(self.id);
    }

    pub fn get_node(&self, index: usize) -> NodeRef {
        NodeRef {
            scene: self.clone(),
            node_index: index as u64,
        }
    }

    pub fn get_len(&self) -> Option<u64> {
        let gpu = self.gpu.lock().unwrap();
        let scene = gpu.assets.get_scene(self.id).unwrap();
        Some(scene.nodes.len() as u64)
    }

}

impl UserData for SceneRef {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("getNumNodes", |_, scene, ()| {
            Ok(scene.get_len())
        });

        methods.add_method("getNode", |_, scene, index: usize| {
            Ok(scene.get_node(index))
        });

        methods.add_method("draw", |_, scene, ()| {
            Ok(scene.draw())
        });
    }
}