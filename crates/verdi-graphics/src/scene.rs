use rlua::{UserData, UserDataMethods};

use crate::{
    node::Node, 
};

#[derive(Clone)]
pub struct Scene {
    pub nodes: Vec<Node>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }
}

impl UserData for Scene {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("getNumNodes", |_, scene, ()| {
            Ok(scene.nodes.len())
        });
    }
}