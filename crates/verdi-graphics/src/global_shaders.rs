use crate::{
    assets::{AssetId, Assets}, 
    shader::Shader, 
    program::Program, 
    material::Material, render_pipeline::RenderPipeline
};

pub struct GlobalShaders {
    pub gouraud: AssetId,
    pub standard_material: AssetId,
}

impl GlobalShaders {
    pub fn new(assets: &mut Assets, pipeline: &RenderPipeline) -> Result<Self, std::io::Error> {
        let gouraud_vs = Shader::new(
            match std::fs::read_to_string( "./crates/verdi-graphics/shaders/gouraud.vs") {
                Ok(gouraud_vs) => gouraud_vs,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let vs_id = assets.add_shader(gouraud_vs);

        let gouraud_fs = Shader::new(
            match std::fs::read_to_string("./crates/verdi-graphics/shaders/gouraud.fs") {
                Ok(gouraud_fs) => gouraud_fs,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            }
        );
        let fs_id = assets.add_shader(gouraud_fs);

        let gouraud_program = assets.add_program(Program::new(vs_id, fs_id));

        let mut material = Material::new(gouraud_program);
        material.add_uniform("u_model", pipeline.model_matrix);
        material.add_uniform("u_view", pipeline.view_matrix);
        material.add_uniform("u_projection", pipeline.perspective_matrix);
        
        Ok(Self {
            gouraud: gouraud_program,
            standard_material: assets.add_material(material),
        })
    }
}

