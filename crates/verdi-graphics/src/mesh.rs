use rlua::UserData;
use gltf::Mesh as GltfMesh;

use crate::assets::AssetId;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Reading gltf file failed")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
}

pub struct Mesh {

}

impl Mesh {
    pub fn new(path: &String) -> Result<Self, MeshError> {
        Ok(Self {})
    }
}

impl From<GltfMesh<'_>> for Mesh {
    fn from(gltf_mesh: GltfMesh) -> Self {
        gltf_mesh.primitives();
        Self { }
    }
}

#[derive(Clone)]
pub struct MeshRef {
    pub id: AssetId,
}

impl MeshRef {
    pub fn new(id: AssetId) -> Self{
        Self { id }
    }
}

impl UserData for MeshRef {}