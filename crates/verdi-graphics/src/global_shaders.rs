use crate::{
    assets::Assets, 
    shader::Shader, 
    program::{Program, ProgramId}, 
    render_pipeline::RenderPipeline, 
};

pub struct GlobalShaders {
    pub gouraud: ProgramId,
    pub std_2d: ProgramId,
}

impl GlobalShaders {
    pub fn new(assets: &mut Assets, pipeline: &RenderPipeline) -> Result<Self, std::io::Error> {
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

