use std::ops::Deref;

use mlua::{UserData, UserDataMethods};
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::node::Node;

pub type ModelId = ResourceId;

#[derive(Clone)]
pub struct Model {
    pub nodes: Vec<Node>,
    pub id: ModelId,
}

impl Resource for Model {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Model {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            id: ModelId::null(),
        }
    }

    pub fn get_node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    // pub fn draw(&self, gpu: &mut GraphicsChip) {
    //     for node in self.nodes.iter() {
    //         node.draw(gpu);
    //     }
    // }
}

#[derive(Clone)]
pub struct ModelHandle(Handle);

impl Deref for ModelHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ModelHandle {
    pub fn new(assets: Assets, id: ModelId) -> Self{
        ModelHandle(assets.new_handle(id))
    }

    pub fn get_node(&self, index: usize) -> Node { // TODO: returns a copy -> is it ok?
        // NodeHandle {
        //     assets: self.assets.clone(),
        //     model: self.clone(),
        //     node_index: index as u64,
        // }

        self
        .get_datas()
        .get::<Model>(self.get_id())
        .expect("Model not found")
        .get_node(index)
        .expect("Node not found")
        .clone()
    }

    pub fn get_len(&self) -> Option<u64> {
        Some(
            self
            .get_datas()
            .get::<Model>(self.get_id())
            .unwrap()
            .nodes
            .len() as u64)
    }
}

impl UserData for ModelHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("getNumNodes", |_, model, ()| {
            Ok(model.get_len())
        });

        methods.add_method("getNode", |_, model, index: usize| {
            Ok(model.get_node(index)) // TODO: warning -> copy of the node
        });
    }
}