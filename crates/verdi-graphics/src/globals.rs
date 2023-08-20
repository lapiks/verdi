use verdi_database::Assets;
use verdi_math::{Vec2, Mat4};

use crate::{
    program::{ProgramId, Program}, 
    shader::Shader, uniform::{UniformId, Uniform},
};

/// Indicates where to find some globals (shader and uniforms) in the database
#[derive(Clone)]
pub struct Globals {
    pub global_shaders: GlobalShaders,
    pub global_uniforms: GlobalUniforms,
}

impl Globals {
    pub fn new(assets: &mut Assets) -> Result<Self, std::io::Error> {
        Ok(Self {
            global_shaders: GlobalShaders::new(assets)?,
            global_uniforms: GlobalUniforms::new(assets),
        })
    }
}

/// Indicates where to find the global uniforms in the uniform database.
#[derive(Clone)]
pub struct GlobalUniforms {
    pub model_matrix: UniformId,
    pub view_matrix: UniformId,
    pub projection_matrix: UniformId,
    pub resolution: UniformId,
    pub enable_lighting: UniformId,
    pub enable_fog: UniformId,
    pub fog_start: UniformId,
    pub fog_end: UniformId,
    pub identity_mat: UniformId, // TODO: temporary
}

impl GlobalUniforms {
    pub fn new(assets: &mut Assets) -> Self {
        Self {
            model_matrix: assets.add(Box::new(Uniform::new(Mat4::IDENTITY))),
            view_matrix: assets.add(Box::new(Uniform::new(Mat4::IDENTITY))),
            projection_matrix: assets.add(Box::new(Uniform::new(Mat4::IDENTITY))),
            resolution: assets.add(Box::new(Uniform::new(Vec2::ZERO))),
            enable_lighting: assets.add(Box::new(Uniform::new(true))),
            enable_fog: assets.add(Box::new(Uniform::new(false))),
            fog_start: assets.add(Box::new(Uniform::new(0.0))),
            fog_end: assets.add(Box::new(Uniform::new(0.0))),
            identity_mat: assets.add(Box::new(Uniform::new(Mat4::IDENTITY))),
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
        let vs_id = assets.add(Box::new(vs));

        let fs = Shader::new(
            match std::fs::read_to_string("./crates/verdi-graphics/shaders/gouraud.fs") {
                Ok(src) => src,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let fs_id = assets.add(Box::new(fs));

        Ok(
            assets.add(Box::new(Program::new(vs_id, fs_id)))
        )
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
        let vs_id = assets.add(Box::new(vs));

        let fs = Shader::new(
            match std::fs::read_to_string("./crates/verdi-graphics/shaders/gouraud_textured.fs") {
                Ok(src) => src,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let fs_id = assets.add(Box::new(fs));

        Ok(assets.add(
                Box::new(
                    Program::new(vs_id, fs_id)
                )
            )
        )
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
        let vs_id = assets.add(Box::new(vs));

        let fs = Shader::new(
            match std::fs::read_to_string("./crates/verdi-graphics/shaders/std2d.fs") {
                Ok(src) => src,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let fs_id = assets.add(Box::new(fs));

        Ok(
            assets.add(
                Box::new(
                    Program::new(
                        vs_id, fs_id
                    )
                )
            )
        )
    }
}