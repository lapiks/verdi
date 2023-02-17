use glium::Display;
use slotmap::{new_key_type, Key};

use crate::{
    shader::ShaderId, 
    assets::Assets, 
    gpu_assets::GpuAssets, 
    gpu_program::GpuProgram
};

new_key_type! {
    pub struct ProgramId;
}

pub struct Program {
    pub vs: ShaderId,
    pub fs: ShaderId,
    pub id: ProgramId,
}

impl Program {
    pub fn new(vs: ShaderId, fs: ShaderId) -> Self {
        Self {
            vs,
            fs,
            id: ProgramId::null(),
        }
    }

    pub fn prepare_rendering(&self, display: &Display, assets: &Assets, gpu_assets: &mut GpuAssets) {
        if gpu_assets.get_program(self.id).is_none() {
            if let Some(vs) = assets.get_shader(self.vs) {
                if let Some(fs) = assets.get_shader(self.fs) {
                    let gpu_program = GpuProgram::new(display, vs, fs);
                    gpu_assets.add_program(self.id, gpu_program);
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct ProgramHandle {
    pub id: ProgramId,
}

impl ProgramHandle {
    pub fn new(id: ProgramId) -> Self{
        Self { id }
    }
}