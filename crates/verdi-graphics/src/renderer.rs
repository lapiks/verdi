use glium::{Surface, uniform, Frame, Display};

use crate::camera::Camera;
use crate::gpu_mesh::GpuMesh;
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

    pub fn prepare_assets(&mut self, display: &Display, gpu: &GraphicsChip) {
        for render_pass in gpu.render_passes.iter() {
            let mesh_ref = render_pass.node.mesh.unwrap();
            if self.gpu_assets.get_mesh(mesh_ref.id).is_none() {
                if let Some(mesh) = gpu.assets.get_mesh(mesh_ref.id) {
                    // construct gpu vertex buffers
                    for primitive in mesh.primitives.iter() {
                        let vertex_buffer = glium::VertexBuffer::new(display, &primitive.vertex_buffer).unwrap();

                        if let Some(index_buffer) = &primitive.index_buffer {
                            let indices = glium::IndexBuffer::new(
                                display, 
                                glium::index::PrimitiveType::from(render_pass.current_primitive),
                                index_buffer
                            ).unwrap();

                            let gpu_mesh = GpuMesh::new(vertex_buffer, Some(indices));
                            self.gpu_assets.add_mesh(mesh_ref.id, gpu_mesh);
                        }
                        else {
                            // let indices = glium::index::NoIndices(glium::index::PrimitiveType::from(render_pass.current_primitive));

                            let gpu_mesh = GpuMesh::new(vertex_buffer, None);
                            self.gpu_assets.add_mesh(mesh_ref.id, gpu_mesh);
                        }

                    }
                    
                }                    
            }

            if let Some(texture_ref) = render_pass.current_texture {
                if self.gpu_assets.get_texture(texture_ref.id).is_none() {
                    if let Some(texture) = gpu.assets.get_texture(texture_ref.id) {
                        self.gpu_assets.add_texture(display, texture_ref.id, texture);
                    }
                }
            }
        }
    }

    pub fn render(&mut self, target: &mut Frame, gpu: &GraphicsChip) {
        // the direction of the light
        let light = [-1.0, 0.4, 0.9f32];

        for render_pass in gpu.render_passes.iter() {
            if self.gpu_assets.get_mesh(render_pass.node.mesh.unwrap().id).is_none() {
                // there sould be a gpu mesh for this id
                // todo: return error
                return;
            }

            let mesh = self.gpu_assets.get_mesh(render_pass.node.mesh.unwrap().id).unwrap();

            let model_matrix = render_pass.node.transform.to_matrix().to_cols_array_2d();

            let view_matrix = Camera::view_matrix(&[0.0, 0.0, 5.0], &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);

            let perspective_matrix = {
                let (width, height) = target.get_dimensions();
                let aspect_ratio = height as f32 / width as f32;
            
                let fov: f32 = 3.141592 / 3.0;
                let zfar = 1024.0;
                let znear = 0.1;
            
                let f = 1.0 / (fov / 2.0).tan();
            
                [
                    [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                    [         0.0         ,     f ,              0.0              ,   0.0],
                    [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                    [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                ]
            };

            if let Some(tex_ref) = render_pass.current_texture {
                if let Some(gpu_tex) = self.gpu_assets.get_texture(tex_ref.id) {
                    let uniforms = uniform! {
                        model: model_matrix,
                        view: view_matrix,
                        perspective: perspective_matrix,
                        u_light: light,
                        tex: gpu_tex,
                    };

                    if let Some(index_buffer) = &mesh.index_buffer {
                        target.draw(
                            &mesh.vertex_buffer,
                            index_buffer, 
                            &self.program, 
                            &uniforms,
                            &Default::default()
                        ).unwrap();
                    }
                    else {
                        target.draw(
                            &mesh.vertex_buffer,
                            &glium::index::NoIndices(glium::index::PrimitiveType::from(render_pass.current_primitive)), 
                            &self.program, 
                            &uniforms,
                            &Default::default()
                        ).unwrap();
                    }
                }
            }
            else {
                let uniforms = uniform! {
                    model: model_matrix,
                    view: view_matrix,
                    perspective: perspective_matrix,
                    u_light: light,
                };

                if let Some(index_buffer) = &mesh.index_buffer {
                    target.draw(
                        &mesh.vertex_buffer,
                        index_buffer, 
                        &self.program, 
                        &uniforms,
                        &Default::default()
                    ).unwrap();
                }
                else {
                    target.draw(
                        &mesh.vertex_buffer,
                        &glium::index::NoIndices(glium::index::PrimitiveType::from(render_pass.current_primitive)), 
                        &self.program, 
                        &uniforms,
                        &Default::default()
                    ).unwrap();
                }
            }
        }
    }
}