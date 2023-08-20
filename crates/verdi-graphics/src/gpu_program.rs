use slotmap::{new_key_type, Key};
use verdi_database::Resource;

use crate::{shader::Shader, gpu_assets::GpuAsset};

new_key_type! {
    pub struct GpuProgramId;
}

pub struct GpuProgram {
    pub gl: glium::Program,
    pub id: GpuProgramId,
}

impl GpuProgram {
    pub fn new(display: &glium::Display, vs: &Shader, fs: &Shader) -> Self {
        Self {
            gl: glium::Program::from_source(
                display, 
                vs.get_source(), 
                fs.get_source(), 
                None
            ).unwrap(),
            id: GpuProgramId::null(),
        } 
    }
}

impl Resource for GpuProgram {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl GpuAsset for GpuProgram {}