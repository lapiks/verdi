use glium::{Surface, Frame, Display};

use crate::{
    camera::Camera,
    prelude::GraphicsChip, 
    gpu_assets::GpuAssets, 
};

pub struct Renderer {
    gpu_assets: GpuAssets,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            gpu_assets: GpuAssets::new(),
        }
    }

    pub fn prepare_assets(&mut self, display: &Display, gpu: &GraphicsChip) {
        // à rendre générique
        for render_pass in gpu.pipeline.render_passes.iter() {
            if let Some(mesh_ref) = render_pass.node.mesh {
                let mesh = gpu.assets.get_mesh(mesh_ref.id).expect("Missing mesh asset");
                // construct gpu primitives
                for primitive in mesh.primitives.iter() {
                    primitive.prepare_rendering(display, &mut self.gpu_assets);

                    // construct gpu objects needed by the material
                    if let Some(material) = gpu.assets.get_material(primitive.material) {
                        material.prepare_rendering(display, &gpu.uniforms, &gpu.assets, &mut self.gpu_assets);
                    }             
                }   
            }

            // // construct gpu textures
            // if let Some(texture_ref) = render_pass.current_texture {
            //     if let Some(texture) = gpu.assets.get_texture(texture_ref.id) {
            //         texture.prepare_rendering(display, &gpu.assets, &mut self.gpu_assets);
            //     }
            // }
        }
        
        // construct gpu programs
        if self.gpu_assets.get_program(gpu.globals.gouraud).is_none() {
            if let Some(program) = gpu.assets.get_program(gpu.globals.gouraud) {
                program.prepare_rendering(display, &gpu.assets, &mut self.gpu_assets)
            }   
        }
    }

    pub fn render(&mut self, target: &mut Frame, gpu: &mut GraphicsChip) {
        // the direction of the light
        let light = [-1.0, 0.4, 0.9f32];

        // view matrix
        let view_matrix = Camera::view_matrix(&[-2.5, 10.0, 15.0], &[0.0, -0.5, -1.0], &[0.0, 1.0, 0.0]);
        *gpu.uniforms
            .get_mat4_mut(gpu.pipeline.view_matrix)
            .expect("View matrix uniform missing") = view_matrix;

        // perspective matrix
        let perspective_matrix = Camera::perspective_matrix(
            target.get_dimensions().0, 
            target.get_dimensions().1
        );
        *gpu.uniforms
            .get_mat4_mut(gpu.pipeline.perspective_matrix)
            .expect("Perspective matrix uniform missing") = perspective_matrix;

        for render_pass in gpu.pipeline.render_passes.iter() {
            // model matrix
            let model_matrix = render_pass.node.transform.to_matrix();
            *gpu.uniforms
                .get_mat4_mut(gpu.pipeline.model_matrix)
                .expect("Model matrix uniform missing") = model_matrix;

            let mesh_ref = render_pass.node.mesh.unwrap();
            let mesh = gpu.assets.get_mesh(mesh_ref.id).unwrap();

            for primitive in mesh.primitives.iter() {
                let gpu_primitive = self.gpu_assets.get_primitive(primitive.id).unwrap();
                let material = gpu.assets.get_material(primitive.material).expect("Material not found");
                let material_ref = material.get_ref(&gpu.uniforms, &self.gpu_assets).expect("Unable to create MaterialRef from uniforms");

                let program = self.gpu_assets.get_program(gpu.globals.gouraud).expect("Gouraud program not found");

                let vertex_buffer = &gpu_primitive.vertex_buffer;

                let draw_params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    .. Default::default()
                };

                if let Some(index_buffer) = &gpu_primitive.index_buffer {
                    target.draw(
                        vertex_buffer,
                        index_buffer,
                        &program.gl, 
                        &material_ref,
                        &draw_params
                    ).unwrap();
                }
                else {
                    target.draw(
                        vertex_buffer,
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                        &program.gl, 
                        &material_ref,
                        &draw_params
                    ).unwrap();
                }
            }
        }
    }
}