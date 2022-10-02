use rlua::UserData;

use crate::{
    assets::AssetId, 
    vertex::Vertex,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Reading gltf file failed")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
}

type VertexBuffer = Vec<Vertex>;
type IndexBuffer = Vec<u32>;

pub struct Primitive {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: Option<IndexBuffer>,
    pub material: AssetId,
}

pub struct Mesh {
    pub primitives: Vec<Primitive>
}

impl Mesh {
    pub fn new(primitives: Vec<Primitive>) -> Self {
        Self {
            primitives,
        }
    }
}

#[derive(Clone, Copy)]
pub struct MeshRef {
    pub id: AssetId,
}

impl MeshRef {
    pub fn new(id: AssetId) -> Self{
        Self { id }
    }
}

impl UserData for MeshRef {}