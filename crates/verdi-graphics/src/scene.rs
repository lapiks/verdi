use std::sync::{Mutex, Arc};

use rlua::{UserData, UserDataMethods};

use crate::{
    node::{Node, NodeRef}, 
    assets::AssetId, 
    graphics_chip::GraphicsChip
};

pub type SceneId = AssetId;

#[derive(Clone)]
pub struct Scene {
    pub nodes: Vec<Node>,
    pub id: SceneId,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            id: uuid::Uuid::nil(),
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
        self.gpu.lock().unwrap().draw(self.clone());
    }

    pub fn get_node(&self, index: usize) -> NodeRef {
        let gpu = self.gpu.lock().unwrap();
        let scene = gpu.assets.get_scene(self.id).unwrap();
        let node = scene.nodes.get(index).unwrap();
        NodeRef {
            scene: self.clone(),
            node_index: index as u64,
        }
    }

    // pub fn get_node(&self, index: usize) -> Option<NodeRef> {
    //     let scene = self.get_scene()?;
    //     scene.nodes.get(index)
    //         .and_then(|node| {
    //             Some(
    //                 NodeRef::new(
    //                     *self,
    //                     index as u64
    //                 )
    //             )
    //         }
    //     )
    // }

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