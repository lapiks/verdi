use rlua::UserData;

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
    // fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    //     methods.add_method("draw", |_, scene, ()| {
    //         //draw
    //         // ajouter une render pass par mesh à rendre dans le graphics chip
    //         // scene aurait une ref vers GraphicsChip ?
    //         Ok(())
    //     });
    // }
}