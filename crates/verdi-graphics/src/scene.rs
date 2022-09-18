use std::{fs, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneError {
    #[error("Reading gltf file failed")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
}

pub struct Scene {

}

impl Scene {
    pub fn new(path: &String) -> Result<Self, SceneError> {
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
                Scene::process_node(&node);
            }
        }
        
        Ok(Self {})
    }

    fn process_node(node: &gltf::Node) {
        for child in node.children() {
            let mesh = child.mesh();
            Scene::process_node(&child);
        }
    }
}