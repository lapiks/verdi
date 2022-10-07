use glium::Display;
use uuid::Uuid;

use crate::{
    shader::Shader, 
    assets::{AssetId, Assets}, 
    gpu_assets::{GpuAssets}
};

pub struct Program {
    pub vs: AssetId,
    pub fs: AssetId,
    pub id: AssetId,
}

impl Program {
    pub fn new(vs: AssetId, fs: AssetId) -> Self {
        Self {
            vs,
            fs,
            id: Uuid::nil(),
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

pub struct GpuProgram {
    pub gl: glium::Program
}

impl GpuProgram {
    pub fn new(display: &glium::Display, vs: &Shader, fs: &Shader) -> Self {
        Self {
            gl: glium::Program::from_source(
                display, 
                vs.get_source(), 
                fs.get_source(), 
                None
            ).unwrap()
        } 
    }
}