use rlua::UserData;
use std::{fs, io};

use crate::assets::AssetId;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Reading gltf file failed")]
    ReadFileError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
}

pub struct Mesh {

}

impl Mesh {
    pub fn new(path: &String) -> Result<Self, MeshError> {
        // mesh loading
        let file = fs::File::open(&path)?;
        let reader = io::BufReader::new(file);
        let gltf = gltf::Gltf::from_reader(reader)?;

        for scene in gltf.scenes() {
            print!("Scene {}", scene.index());
            #[cfg(feature = "names")]
            print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
            println!();
            for node in scene.nodes() {
                Mesh::process_node(&node);
            }
        }
        Ok(Self {})
    }

    fn process_node(node: &gltf::Node) {
        for child in node.children() {
            Mesh::process_node(&child);
        }
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