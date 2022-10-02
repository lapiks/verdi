use std::collections::HashMap;

use glium::Display;

use crate::{
    assets::AssetId, 
    gpu_mesh::GpuMesh, 
    program::GpuProgram, 
    gpu_image::GpuImage
};

pub struct GpuAssets {
    meshes: HashMap<AssetId, GpuMesh>,
    textures: HashMap<AssetId, GpuImage>,
    programs: HashMap<AssetId, GpuProgram>,
}

impl GpuAssets {
    pub fn new() -> Self {
        Self { 
            meshes: HashMap::default(),
            textures: HashMap::default(),
            programs: HashMap::default(),
        }
    }

    pub fn add_texture(&mut self, display: &Display, id: AssetId, gpu_image: GpuImage) {
        self.textures.insert(id, gpu_image);
    }

    pub fn get_texture(&self, id: AssetId) -> Option<&GpuImage> {
        self.textures.get(&id)
    }

    pub fn add_mesh(&mut self, id: AssetId, gpu_mesh: GpuMesh) {
        self.meshes.insert(id, gpu_mesh);
    }

    pub fn get_mesh(&self, id: AssetId) -> Option<&GpuMesh> {
        self.meshes.get(&id)
    }

    pub fn add_program(&mut self, id: AssetId, program: GpuProgram) {
        self.programs.insert(id, program);
    }

    pub fn get_program(&self, id: AssetId) -> Option<&GpuProgram> {
        self.programs.get(&id)
    }
}