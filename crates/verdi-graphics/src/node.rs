use std::{cell::RefCell, rc::Rc};

use mlua::{UserData, UserDataMethods};

use verdi_math::prelude::Transform;

use crate::{
    scene::SceneHandle, 
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
    pub fn draw(&self, gpu: &mut GraphicsChip) {
        gpu.draw_node(&self);
    }
}

#[derive(Clone)]
pub struct NodeHandle {
    pub gpu: Rc<RefCell<GraphicsChip>>,
    pub scene: SceneHandle,
    pub node_index: NodeId,
}

impl NodeHandle {
    pub fn new(gpu: Rc<RefCell<GraphicsChip>>, scene: SceneHandle, node_index: NodeId) -> Self {
        Self {
            gpu,
            scene,
            node_index,
        }
    }

    pub fn draw(&self) {
        // a revoir
        let gpu = self.gpu.borrow();
        let db = gpu.database.borrow();
        let scene = db.assets.get_scene(self.scene.id);
        if let Some(scene) = scene {
            let node = scene.get_node(self.node_index);
            if let Some(node) = node {
                node.draw(&mut self.gpu.borrow_mut());
            }
        }
    }
}

impl UserData for NodeHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("draw", |_, node, ()| {
            Ok(node.draw())
        });
    }
}