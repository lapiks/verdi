use crate::{
    shader::Shader, 
    assets::AssetId
};

pub struct Program {
    pub vs: AssetId,
    pub fs: AssetId,
}

impl Program {
    pub fn new(vs: AssetId, fs: AssetId) -> Self {
        Self {
            vs,
            fs,
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