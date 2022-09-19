use rlua::UserData;
use gltf::Mesh as GltfMesh;

use crate::{assets::AssetId, vertex::Vertex};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Reading gltf file failed")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
}

type VertexBuffer = Vec<Vertex>;
type Primitives = Vec<VertexBuffer>;

pub struct Mesh {
    pub primitives: Primitives,
}

impl Mesh {
    pub fn new(primitives: Primitives) -> Self {
        Self {
            primitives,
        }
    }
}

// impl From<GltfMesh<'_>> for Mesh {
//     fn from(gltf_mesh: GltfMesh) -> Self {
        
//         Self { primitives }
//     }
// }

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