use verdi_math::{Vec2, Vec4, Mat4};

use crate::{
    uniforms::{UniformId, Uniforms}, 
    program::{ProgramId, Program}, 
    assets::Assets, 
    shader::Shader
};

pub struct Globals {
    pub global_shaders: GlobalShaders,
    pub global_uniforms: GlobalUniforms, 
    pub clear_color: Vec4,
}

impl Globals {
    pub fn new(assets: &mut Assets, uniforms: &mut Uniforms) -> Result<Self, std::io::Error> {
        Ok(Self {
            global_shaders: GlobalShaders::new(assets)?,
            global_uniforms: GlobalUniforms::new(uniforms),
            clear_color: Vec4::new(0.0, 0.0, 0.0, 1.0),
        })
    }
}

pub struct GlobalUniforms {
    pub model_matrix: UniformId,
    pub view_matrix: UniformId,
    pub perspective_matrix: UniformId,
    pub resolution: UniformId,
    pub fog_start: UniformId,
    pub fog_end: UniformId,
}

impl GlobalUniforms {
    pub fn new(uniforms: &mut Uniforms) -> Self{
        Self {
            model_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            view_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            perspective_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            resolution: uniforms.add_vec2(Vec2::ZERO),
            fog_start: uniforms.add_float(0.0),
            fog_end: uniforms.add_float(0.0),
        }
    }    
}

pub struct GlobalShaders {
    pub gouraud: ProgramId,
    pub std_2d: ProgramId,
}

impl GlobalShaders {
    pub fn new(assets: &mut Assets) -> Result<Self, std::io::Error> {
        Ok(
            Self {
                gouraud: GlobalShaders::init_gouraud(assets)?,
                std_2d: GlobalShaders::init_std_2d(assets)?,
            }
        )
    }

    fn init_gouraud(assets: &mut Assets) -> Result<ProgramId, std::io::Error> {
        let vs = Shader::new(
            match std::fs::read_to_string( "./crates/verdi-graphics/shaders/gouraud.vs") {
                Ok(src) => src,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let vs_id = assets.add_shader(vs);

        let fs = Shader::new(
            match std::fs::read_to_string("./crates/verdi-graphics/shaders/gouraud.fs") {
                Ok(src) => src,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let fs_id = assets.add_shader(fs);

        Ok(assets.add_program(Program::new(vs_id, fs_id)))
    }

    fn init_std_2d(assets: &mut Assets) -> Result<ProgramId, std::io::Error> {
        let vs = Shader::new(
            match std::fs::read_to_string( "./crates/verdi-graphics/shaders/std2d.vs") {
                Ok(src) => src,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let vs_id = assets.add_shader(vs);

        let fs = Shader::new(
            match std::fs::read_to_string("./crates/verdi-graphics/shaders/std2d.fs") {
                Ok(src) => src,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let fs_id = assets.add_shader(fs);

        Ok(assets.add_program(Program::new(vs_id, fs_id)))
    }
}