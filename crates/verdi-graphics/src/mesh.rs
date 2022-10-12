use rlua::UserData;
use slotmap::{new_key_type, Key};

use crate::{
    primitive::PrimitiveId,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Reading gltf file failed")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
}

new_key_type! {
    pub struct MeshId;
}

pub struct Mesh {
    pub primitives: Vec<PrimitiveId>,
    pub id: MeshId,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            id: MeshId::null(),
        }
    }

    pub fn add_primitive(&mut self, primitive: PrimitiveId) {
        self.primitives.push(primitive);
    }
}

#[derive(Clone, Copy)]
pub struct MeshRef {
    pub id: MeshId,
}

impl MeshRef {
    pub fn new(id: MeshId) -> Self{
        Self { id }
    }
}

impl UserData for MeshRef {}