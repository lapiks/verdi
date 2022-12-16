use slotmap::SecondaryMap;

use crate::{
    gpu_image::GpuImage, 
    gpu_program::GpuProgram, 
    gpu_mesh::GpuMesh,
    image::ImageId, 
    program::ProgramId, 
    mesh::MeshId, 
};

pub struct GpuAssets {
    meshes: SecondaryMap<MeshId, GpuMesh>,
    textures: SecondaryMap<ImageId, GpuImage>,
    programs: SecondaryMap<ProgramId, GpuProgram>,
}

impl GpuAssets {
    pub fn new() -> Self {
        Self { 
            meshes: SecondaryMap::default(),
            textures: SecondaryMap::default(),
            programs: SecondaryMap::default(),
        }
    }

    pub fn clear(&mut self) {
        self.meshes.clear();
        self.textures.clear();
        self.programs.clear();
    }

    pub fn add_texture(&mut self, id: ImageId, gpu_image: GpuImage) {
        self.textures.insert(id, gpu_image);
    }

    pub fn get_texture(&self, id: ImageId) -> Option<&GpuImage> {
        self.textures.get(id)
    }

    pub fn add_mesh(&mut self, id: MeshId, gpu_mesh: GpuMesh) {
        self.meshes.insert(id, gpu_mesh);
    }

    pub fn get_mesh(&self, id: MeshId) -> Option<&GpuMesh> {
        self.meshes.get(id)
    }

    pub fn add_program(&mut self, id: ProgramId, program: GpuProgram) {
        self.programs.insert(id, program);
    }

    pub fn get_program(&self, id: ProgramId) -> Option<&GpuProgram> {
        self.programs.get(id)
    }
}