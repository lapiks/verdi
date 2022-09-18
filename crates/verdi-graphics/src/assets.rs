use std::collections::HashMap;
use uuid::Uuid;
use crate::{image::{Image, ImageRef}, mesh::{Mesh, MeshRef}};

pub type AssetId = Uuid;

#[derive(PartialEq)]
pub enum AssetState {
    Created,
    Loaded,
}

pub struct Assets {
    textures: HashMap<AssetId, Image>,
    meshes: HashMap<AssetId, Mesh>,
}

impl Assets {
    pub fn new() -> Self {
        Self { 
            textures: HashMap::default(),
            meshes: HashMap::default(),
        }
    }

    pub fn add_texture(&mut self, image: Image) -> ImageRef {
        let id = Uuid::new_v4();
        self.textures.insert(id, image);

        ImageRef::new(id)
    }

    pub fn get_texture(&self, id: AssetId) -> Option<&Image> {
        self.textures.get(&id)
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshRef {
        let id = Uuid::new_v4();
        self.meshes.insert(id, mesh);

        MeshRef::new(id)
    }

    pub fn get_mesh(&self, id: AssetId) -> Option<&Mesh> {
        self.meshes.get(&id)
    }
}