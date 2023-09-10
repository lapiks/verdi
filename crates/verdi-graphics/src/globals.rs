use verdi_database::Assets;
use verdi_math::{Vec2, Mat4};

use crate::{
    program::{Program, ProgramId}, 
    shader::Shader, 
    uniform::{Uniform, UniformHandle, UniformValue}, 
    pipeline::{Pipeline, PipelineHandle},
};

/// Indicates where to find some globals (pipelines, shader and uniforms) in the database
#[derive(Clone)]
pub struct Globals {
    pub global_pipelines: GlobalPipelines,
    pub global_programs: GlobalPrograms,
    pub global_uniforms: GlobalUniforms,
}

impl Globals {
    pub fn new(assets: &mut Assets) -> Result<Self, std::io::Error> {
        let global_uniforms = GlobalUniforms::new(assets);
        let global_programs = GlobalPrograms::new(assets)?;
        let global_pipelines = GlobalPipelines::new(assets, &global_programs);
        Ok(Self {
            global_pipelines,
            global_programs,
            global_uniforms,
        })
    }
}

#[derive(Clone)]
pub struct GlobalPipelines {
    pub default_pipeline: PipelineHandle,
}

impl GlobalPipelines {
    pub fn new(assets: &mut Assets, global_programs: &GlobalPrograms) -> Self {
        Self {
            default_pipeline: PipelineHandle::new(
                assets.clone(), 
                assets.add(
                    Box::new(
                        Pipeline::new(
                            global_programs.gouraud_textured
                        )
                    )
                )
            ),
        }
    }
}

/// Indicates where to find the global uniforms in the uniform database.
#[derive(Clone)]
pub struct GlobalUniforms {
    pub model_matrix: UniformHandle,
    pub view_matrix: UniformHandle,
    pub projection_matrix: UniformHandle,
    pub resolution: UniformHandle,
    pub enable_lighting: UniformHandle,
    pub enable_fog: UniformHandle,
    pub fog_start: UniformHandle,
    pub fog_end: UniformHandle,
    pub identity_mat: UniformHandle, // TODO: temporary
}

impl GlobalUniforms {
    pub fn new(assets: &mut Assets) -> Self {
        let model_matrix = UniformHandle::new(
            assets.clone(), 
            assets.add(Box::new(Uniform::new(UniformValue::Mat4(Mat4::IDENTITY))))
        );
        let view_matrix = UniformHandle::new(
            assets.clone(), 
            assets.add(Box::new(Uniform::new(UniformValue::Mat4(Mat4::IDENTITY))))
        );
        let projection_matrix = UniformHandle::new(
            assets.clone(), 
            assets.add(Box::new(Uniform::new(UniformValue::Mat4(Mat4::IDENTITY))))
        );
        let resolution = UniformHandle::new(
            assets.clone(), 
            assets.add(Box::new(Uniform::new(UniformValue::Vec2(Vec2::ZERO))))
        );
        let enable_lighting = UniformHandle::new(
            assets.clone(), 
            assets.add(Box::new(Uniform::new(UniformValue::Bool(true))))
        );
        let enable_fog = UniformHandle::new(
            assets.clone(), 
            assets.add(Box::new(Uniform::new(UniformValue::Bool(false))))
        );
        let fog_start = UniformHandle::new(
            assets.clone(), 
            assets.add(Box::new(Uniform::new(UniformValue::Float(0.0))))
        );
        let fog_end = UniformHandle::new(
            assets.clone(), 
            assets.add(Box::new(Uniform::new(UniformValue::Float(0.0))))
        );
        let identity_mat = UniformHandle::new(
            assets.clone(), 
            assets.add(Box::new(Uniform::new(UniformValue::Mat4(Mat4::IDENTITY))))
        );

        Self {
            model_matrix,
            view_matrix,
            projection_matrix,
            resolution,
            enable_lighting,
            enable_fog,
            fog_start,
            fog_end,
            identity_mat,
        }
    }
}

/// Indicates where to find the global shaders in the database
#[derive(Clone)]
pub struct GlobalPrograms {
    pub gouraud: ProgramId,
    pub gouraud_textured: ProgramId,
    pub std_2d: ProgramId,
    pub simple: ProgramId,
}

impl GlobalPrograms {
    pub fn new(assets: &mut Assets) -> Result<Self, std::io::Error> {
        Ok(
            Self {
                gouraud: GlobalPrograms::init_gouraud(assets)?,
                gouraud_textured: GlobalPrograms::init_gouraud_textured(assets)?,
                std_2d: GlobalPrograms::init_std_2d(assets)?,
                simple: GlobalPrograms::init_simple(assets)?,
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

        Ok(
            assets.add(Box::new(Program::new(vs_id, fs_id)))
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
            assets.add(Box::new(Program::new(vs_id, fs_id)))
        )
    }

    fn init_simple(assets: &mut Assets) -> Result<ProgramId, std::io::Error> {
        let vs = Shader::new(
            match std::fs::read_to_string( "./crates/verdi-graphics/shaders/simple.vs") {
                Ok(src) => src,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let vs_id = assets.add(Box::new(vs));

        let fs = Shader::new(
            match std::fs::read_to_string("./crates/verdi-graphics/shaders/simple.fs") {
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
}