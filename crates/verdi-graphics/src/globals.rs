use verdi_math::{Vec2, Mat4};

use crate::{
    uniforms::{UniformId, Uniforms}, 
    program::{ProgramId, Program}, 
    assets::Assets, 
    shader::Shader, database::DataBase
};

#[derive(Clone)]
pub struct Globals {
    pub global_shaders: GlobalShaders,
    pub global_uniforms: GlobalUniforms,
}

impl Globals {
    pub fn new(database: &mut DataBase) -> Result<Self, std::io::Error> {
        Ok(Self {
            global_shaders: GlobalShaders::new(&mut database.assets)?,
            global_uniforms: GlobalUniforms::new(&mut database.uniforms),
        })
    }
}

/// Indicates where to find the global uniforms in the uniform database.
#[derive(Clone)]
pub struct GlobalUniforms {
    pub model_matrix: UniformId,
    pub view_matrix: UniformId,
    pub perspective_matrix: UniformId,
    pub resolution: UniformId,
    pub enable_lighting: UniformId,
    pub enable_fog: UniformId,
    pub fog_start: UniformId,
    pub fog_end: UniformId,
}

impl GlobalUniforms {
    pub fn new(uniforms: &mut Uniforms) -> Self {
        Self {
            model_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            view_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            perspective_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            resolution: uniforms.add_vec2(Vec2::ZERO),
            enable_lighting: uniforms.add_boolean(true),
            enable_fog: uniforms.add_boolean(false),
            fog_start: uniforms.add_float(0.0),
            fog_end: uniforms.add_float(0.0),
        }
    }
}

/// Indicates where to find the global shaders in the database
#[derive(Clone)]
pub struct GlobalShaders {
    pub gouraud: ProgramId,
    pub gouraud_textured: ProgramId,
    pub std_2d: ProgramId,
}

impl GlobalShaders {
    pub fn new(assets: &mut Assets) -> Result<Self, std::io::Error> {
        Ok(
            Self {
                gouraud: GlobalShaders::init_gouraud(assets)?,
                gouraud_textured: GlobalShaders::init_gouraud_textured(assets)?,
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

    fn init_gouraud_textured(assets: &mut Assets) -> Result<ProgramId, std::io::Error> {
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
            match std::fs::read_to_string("./crates/verdi-graphics/shaders/gouraud_textured.fs") {
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