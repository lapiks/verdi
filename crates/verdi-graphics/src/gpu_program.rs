use verdi_database::Resource;

use crate::gpu_assets::GpuAsset;

pub struct GpuProgram(glium::Program);

impl GpuProgram {
    pub fn new(gl_program: glium::Program) -> Self {
        Self(
            gl_program
        ) 
    }
}

impl GpuProgram {
    pub fn get_gl_program(&self) -> &glium::Program {
        &self.0
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