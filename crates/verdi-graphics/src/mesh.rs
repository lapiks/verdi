use std::sync::{Mutex, Arc};

use rlua::{UserData, UserDataMethods};
use slotmap::{new_key_type, Key};

use crate::{
    primitive::PrimitiveId, graphics_chip::GraphicsChip,
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

#[derive(Clone)]
pub struct MeshRef {
    pub gpu: Arc<Mutex<GraphicsChip>>,
    pub id: MeshId,
}

impl MeshRef {
    pub fn new(gpu: Arc<Mutex<GraphicsChip>>, id: MeshId) -> Self{
        Self { 
            gpu,
            id,
         }
    }

    pub fn set_vertices(&self) {
        let gpu = self.gpu.lock().unwrap();
        let mesh = gpu.assets.get_mesh(self.id).unwrap();
    }
}

impl UserData for MeshRef {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("setVertices", |_, mesh, ()| {
            Ok(mesh.set_vertices())
        });
    }
}