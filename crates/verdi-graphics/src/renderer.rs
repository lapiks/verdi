use glium::{Surface, Frame, Display};

use crate::{
    camera::Camera,
    gpu_mesh::GpuMesh,
    prelude::GraphicsChip, 
    gpu_assets::GpuAssets, 
    program::GpuProgram
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

                        //primitive.material;
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

        if self.gpu_assets.get_program(gpu.globals.gouraud).is_none() {
            if let Some(program) = gpu.assets.get_program(gpu.globals.gouraud) {
                if let Some(vs) = gpu.assets.get_shader(program.vs) {
                    if let Some(fs) = gpu.assets.get_shader(program.fs) {
                        let gpu_program = GpuProgram::new(display, vs, fs);
                        self.gpu_assets.add_program(gpu.globals.gouraud, gpu_program);
                    }
                    
                }
                
            }   
        }
    }

    pub fn render(&mut self, target: &mut Frame, gpu: &mut GraphicsChip) {
        // the direction of the light
        let light = [-1.0, 0.4, 0.9f32];

        // view matrix
        let view_matrix = Camera::view_matrix(&[0.0, 0.0, 5.0], &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);
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
            let gpu_mesh = self.gpu_assets.get_mesh(mesh_ref.id).unwrap();

            for primitive in mesh.primitives.iter() {
                let material = gpu.assets.get_material(primitive.material).expect("Material not found");
                let material_ref = material.get_ref(&gpu.uniforms, &self.gpu_assets).expect("Program not found");

                let program = self.gpu_assets.get_program(gpu.globals.gouraud).expect("Gouraud program not found");

                let vertex_buffer = &gpu_mesh.vertex_buffer;
                if let Some(index_buffer) = &gpu_mesh.index_buffer {
                    target.draw(
                        vertex_buffer,
                        index_buffer,
                        &program.gl, 
                        &material_ref,
                        &Default::default()
                    ).unwrap();
                }
                else {
                    target.draw(
                        vertex_buffer,
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                        &program.gl, 
                        &material_ref,
                        &Default::default()
                    ).unwrap();
                }
            }
        }
    }
}