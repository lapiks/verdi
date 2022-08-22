use crate::{shader::Shader};

pub struct Program {
    pub internal_program: glium::Program
}

impl Program {
    pub fn new(display: &glium::Display, vs: &Shader, fs: &Shader) -> Self {
        Self {
            internal_program: glium::Program::from_source(
                display, 
                vs.src.as_str(), 
                fs.src.as_str(), 
                None
            ).unwrap()
        } 
    }
}