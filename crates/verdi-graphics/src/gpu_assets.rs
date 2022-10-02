use std::collections::HashMap;

use glium::{
    texture::SrgbTexture2d as GpuTexture, 
    Display,
};

use crate::{
    assets::AssetId, 
    image::Image, 
    gpu_mesh::GpuMesh, 
    program::GpuProgram
};

pub struct GpuAssets {
    meshes: HashMap<AssetId, GpuMesh>,
    textures: HashMap<AssetId, GpuTexture>,
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

    pub fn add_texture(&mut self, display: &Display, id: AssetId, image: &Image) -> &GpuTexture {
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.get_data().as_raw(), image.get_dimensions());
        let texture = glium::texture::SrgbTexture2d::new(display, raw_image).unwrap();
        self.textures.insert(id, texture);

        self.textures.get(&id).unwrap()
    }

    pub fn get_texture(&self, id: AssetId) -> Option<&GpuTexture> {
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