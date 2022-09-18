use glium::{Surface, uniform, Frame, Display};
use glium::{texture::SrgbTexture2d as GpuTexture};

use crate::{prelude::GraphicsChip, gpu_assets::GpuAssets};

pub struct Renderer {
    program: glium::Program,
    gpu_assets: GpuAssets,
}

impl Renderer {
    pub fn new(display: &Display) -> Result<Self, std::io::Error> {
        // TODO gérer erreurs avec GraphicsChipError
        let gouraud_vs = match std::fs::read_to_string( "./crates/verdi-graphics/shaders/gouraud.vs") {
            Ok(gouraud_vs)  => gouraud_vs,
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        };

        let gouraud_fs = match std::fs::read_to_string("./crates/verdi-graphics/shaders/gouraud.fs") {
            Ok(gouraud_fs)  => gouraud_fs,
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        };
        
        let program = glium::Program::from_source(
            display, 
            gouraud_vs.as_str(), 
            gouraud_fs.as_str(), 
            None
        ).unwrap();

        Ok(Self {
            program,
            gpu_assets: GpuAssets::new(),
        })
    }

    pub fn render(&mut self, display: &Display, target: &mut Frame, gpu: &GraphicsChip) {
        // let mut target = window.get_display().draw();
        // target.clear_color(0.0, 0.0, 0.0, 1.0);

        // uniforms (à bouger)
        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        // the direction of the light
        let light = [-1.0, 0.4, 0.9f32];

        for render_pass in gpu.render_passes.iter() {
            let vertex_buffer = glium::VertexBuffer::new(display, &render_pass.vertex_buffer).unwrap();
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::from(render_pass.current_primitive));

            match render_pass.current_texture {
                Some(tex_id) => {
                    if let Some(tex) = gpu.assets.get_texture(tex_id) {
                        let gpu_tex: &GpuTexture;
                        if self.gpu_assets.get_texture(tex_id).is_none() {
                            gpu_tex = self.gpu_assets.add_texture(display, tex_id, tex);
                        }
                        else {
                            gpu_tex = self.gpu_assets.get_texture(tex_id).unwrap();
                        }

                        let uniforms = uniform! {
                            matrix: matrix,
                            u_light: light,
                            tex: gpu_tex,
                        };

                        target.draw(
                            &vertex_buffer,
                            &indices, 
                            &self.program, 
                            &uniforms,
                            &Default::default()
                        ).unwrap();
                    }
                    else {
                        let uniforms = uniform! {
                            matrix: matrix,
                            u_light: light
                        };
    
                        target.draw(
                            &vertex_buffer,
                            &indices, 
                            &self.program, 
                            &uniforms,
                            &Default::default()
                        ).unwrap();
                    }   
                }
                None => {
                    let uniforms = uniform! {
                        matrix: matrix,
                        u_light: light,
                    };

                    target.draw(
                        &vertex_buffer,
                        &indices, 
                        &self.program, 
                        &uniforms,
                        &Default::default()
                    ).unwrap();
                }
            }        
        }        
        //target.finish().unwrap();
    }
}