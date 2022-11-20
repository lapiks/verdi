use slotmap::SecondaryMap;

use crate::{
    gpu_primitive::GpuPrimitive, 
    gpu_image::GpuImage, 
    gpu_program::GpuProgram, 
    primitive::PrimitiveId, 
    image::ImageId, 
    program::ProgramId
};

pub struct GpuAssets {
    primitives: SecondaryMap<PrimitiveId, GpuPrimitive>,
    textures: SecondaryMap<ImageId, GpuImage>,
    programs: SecondaryMap<ProgramId, GpuProgram>,
}

impl GpuAssets {
    pub fn new() -> Self {
        Self { 
            primitives: SecondaryMap::default(),
            textures: SecondaryMap::default(),
            programs: SecondaryMap::default(),
        }
    }

    pub fn clear(&mut self) {
        self.primitives.clear();
        self.textures.clear();
        self.programs.clear();
    }

    pub fn add_texture(&mut self, id: ImageId, gpu_image: GpuImage) {
        self.textures.insert(id, gpu_image);
    }

    pub fn get_texture(&self, id: ImageId) -> Option<&GpuImage> {
        self.textures.get(id)
    }

    pub fn add_primitive(&mut self, id: PrimitiveId, gpu_primitive: GpuPrimitive) {
        self.primitives.insert(id, gpu_primitive);
    }

    pub fn get_primitive(&self, id: PrimitiveId) -> Option<&GpuPrimitive> {
        self.primitives.get(id)
    }

    pub fn add_program(&mut self, id: ProgramId, program: GpuProgram) {
        self.programs.insert(id, program);
    }

    pub fn get_program(&self, id: ProgramId) -> Option<&GpuProgram> {
        self.programs.get(id)
    }
}