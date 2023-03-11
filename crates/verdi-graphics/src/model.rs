use std::{rc::Rc, cell::RefCell};

use mlua::{UserData, UserDataMethods};
use slotmap::{new_key_type, Key};

use crate::{
    node::{Node, NodeHandle}, 
    graphics_chip::GraphicsChip,
};

new_key_type! {
    pub struct ModelId;
}

#[derive(Clone)]
pub struct Model {
    pub nodes: Vec<Node>,
    pub id: ModelId,
}

impl Model {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            id: ModelId::null(),
        }
    }

    pub fn get_node(&self, index: u64) -> Option<&Node> {
        self.nodes.get(index as usize)
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn draw(&self, gpu: &mut GraphicsChip) {
        for node in self.nodes.iter() {
            node.draw(gpu);
        }
    }
}

#[derive(Clone)]
pub struct ModelHandle {
    pub gpu: Rc<RefCell<GraphicsChip>>,
    pub id: ModelId,
}

impl ModelHandle {
    pub fn new(gpu: Rc<RefCell<GraphicsChip>>, id: ModelId) -> Self{
        Self {
            gpu,
            id
        }
    }

    pub fn draw(&self) {
        self.gpu.borrow_mut().draw_model(self.id);
    }

    pub fn get_node(&self, index: usize) -> NodeHandle {
        NodeHandle {
            gpu: self.gpu.clone(),
            model: self.clone(),
            node_index: index as u64,
        }
    }

    pub fn get_len(&self) -> Option<u64> {
        let gpu = self.gpu.borrow();
        let db = gpu.database.borrow();
        let model = db.assets.get_model(self.id).unwrap();
        Some(model.nodes.len() as u64)
    }
}

impl UserData for ModelHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("getNumNodes", |_, model, ()| {
            Ok(model.get_len())
        });

        methods.add_method("getNode", |_, model, index: usize| {
            Ok(model.get_node(index))
        });

        methods.add_method("draw", |_, model, ()| {
            Ok(model.draw())
        });
    }
}