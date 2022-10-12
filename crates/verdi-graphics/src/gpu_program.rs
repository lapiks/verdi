use slotmap::{new_key_type, Key};

use crate::shader::Shader;

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