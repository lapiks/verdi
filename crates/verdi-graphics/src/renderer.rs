use glium::{
    Surface, 
    Display, 
    framebuffer::{SimpleFrameBuffer, self}, 
    Frame, 
    Rect, 
    BlitTarget, 
    BlitMask, 
    uniforms
};
use verdi_math::{Mat4, Vec2};

use crate::{
    camera::Camera,
    prelude::GraphicsChip, 
    gpu_assets::GpuAssets, 
};

// Le renderer pourrait être plus bas niveau. 
// Une fonction render() pourrait prendre en paramètre un Renderable définissant toutes les infos nécessaire pour rendre un mesh.

/// Low level interface to GPU. 
/// Given some renderable GPU assets, the Renderer is able to draw them using the render function. 
pub struct Renderer {
    gpu_assets: GpuAssets, // devrait être sorti du renderer. Dans GpuDataBase ?
}

impl Renderer {
    pub fn new() -> Self {        
        Self {
            gpu_assets: GpuAssets::new(),
        }
    }

    pub fn on_game_shutdown(&mut self) {
        self.gpu_assets.clear();
    }

    pub fn prepare_assets(&mut self, display: &Display, gpu: &GraphicsChip) {
        let db_lock = gpu.database.borrow();

        // à rendre générique
        for pass in gpu.render_graph.borrow().get_passes().iter() {
            for cmd in pass.get_cmds() {
                let mesh = db_lock.assets
                .get_mesh(cmd.mesh)
                .expect("Missing primitive asset");

                // construct gpu primitive
                mesh.prepare_rendering(
                    display, 
                    &mut self.gpu_assets
                );

                // construct gpu objects needed by the material
                if let Some(material) = gpu.database.borrow().assets.get_material(mesh.material) {
                    material.prepare_rendering(
                        display,
                        &gpu.database.borrow().uniforms, 
                        &gpu.database.borrow().assets,
                        &mut self.gpu_assets
                    );
                }     
            }     
        }
        
        // construct gpu programs
        // Pas fou !
        if self.gpu_assets.get_program(gpu.globals.global_shaders.gouraud).is_none() {
            if let Some(program) = gpu.database.borrow().assets.get_program(gpu.globals.global_shaders.gouraud) {
                program.prepare_rendering(
                    display, 
                    &gpu.database.borrow().assets, 
                    &mut self.gpu_assets
                )
            }   
        }

        if self.gpu_assets.get_program(gpu.globals.global_shaders.gouraud_textured).is_none() {
            if let Some(program) = gpu.database.borrow().assets.get_program(gpu.globals.global_shaders.gouraud_textured) {
                program.prepare_rendering(
                    display, 
                    &gpu.database.borrow().assets, 
                    &mut self.gpu_assets
                )
            }   
        }

        if self.gpu_assets.get_program(gpu.globals.global_shaders.std_2d).is_none() {
            if let Some(program) = gpu.database.borrow().assets.get_program(gpu.globals.global_shaders.std_2d) {
                program.prepare_rendering(
                    display, 
                    &gpu.database.borrow().assets, 
                    &mut self.gpu_assets
                )
            }   
        }
    }

    // pub fn prepare_rendering<'a>(&'a self, target: &Frame, gpu: &'a mut GraphicsChip) -> Vec<Renderable<'a, Vertex>> {
    //     // view matrix
    //     let view_matrix = Camera::view_matrix(&[-5.0, 2.5, -10.0], &[0.0, -0.1, 1.0], &[0.0, 1.0, 0.0]);
    //     *gpu.uniforms
    //         .get_mat4_mut(gpu.pipeline.view_matrix)
    //         .expect("View matrix uniform missing") = view_matrix;

    //     // perspective matrix
    //     let perspective_matrix = Camera::perspective_matrix(
    //         target.get_dimensions().0, 
    //         target.get_dimensions().1
    //     );
    //     *gpu.uniforms
    //         .get_mat4_mut(gpu.pipeline.perspective_matrix)
    //         .expect("Perspective matrix uniform missing") = perspective_matrix;

    //     let mut renderables: Vec<Renderable<Vertex>> = Vec::with_capacity(gpu.pipeline.render_passes.len());
    //     for render_pass in gpu.pipeline.render_passes.iter() {
    //         // model matrix
    //         let model_matrix = render_pass.node.transform.to_matrix();
    //         *gpu.uniforms
    //             .get_mat4_mut(gpu.pipeline.model_matrix)
    //             .expect("Model matrix uniform missing") = model_matrix;

    //         let mesh_ref = render_pass.node.mesh.unwrap();
    //         let mesh = gpu.assets.get_mesh(mesh_ref.id).expect("Mesh asset not found");

    //         for primitive in mesh.primitives.iter() {
    //             let gpu_primitive = self.gpu_assets.get_primitive(primitive.id).expect("Gpu primitive not found");
    //             let material = gpu.assets.get_material(primitive.material).expect("Material not found");
    //             let material_ref = material.get_ref(&gpu.uniforms, &self.gpu_assets).expect("Unable to create MaterialRef from uniforms");

    //             let program = self.gpu_assets.get_program(gpu.globals.gouraud).expect("Gouraud program not found");

    //             let vertex_buffer = &gpu_primitive.vertex_buffer;

    //             let draw_parameters = glium::DrawParameters {
    //                 depth: glium::Depth {
    //                     test: glium::draw_parameters::DepthTest::IfLess,
    //                     write: true,
    //                     .. Default::default()
    //                 },
    //                 blend: glium::draw_parameters::Blend::alpha_blending(),
    //                 .. Default::default()
    //             };

    //             renderables.push(
    //                 Renderable {
    //                     vertex_buffer,
    //                     index_buffer: gpu_primitive.index_buffer.as_ref(),
    //                     program: &program.gl,
    //                     material_ref,
    //                     draw_parameters,
    //                 }
    //             );
    //         }
    //     }

    //     renderables
    // }

    pub fn render(&mut self, framebuffer: &mut SimpleFrameBuffer, gpu: &mut GraphicsChip) {
        let clear_color = gpu.render_state.clear_color;
        framebuffer.clear_color_and_depth(
            (
                clear_color.x,
                clear_color.y,
                clear_color.z,
                clear_color.w
            ),
            1.0
        );

        let target_dimensions = framebuffer.get_dimensions();

        // perspective matrix
        let perspective_matrix = Camera::perspective_matrix(
            target_dimensions.0, 
            target_dimensions.1
        );

        // ortho matrix
        let ortho_matrix = Camera::orthographic_matrix(
            0.0,
            target_dimensions.0 as f32, 
            target_dimensions.1 as f32,
            0.0,
            -10.0,
            10.0,
        );

        *gpu.database.borrow_mut().uniforms
            .get_vec2_mut(gpu.globals.global_uniforms.resolution)
            .expect("Resolution uniform missing") = Vec2::new(
                target_dimensions.0 as f32, 
                target_dimensions.1 as f32
            );

        for pass in gpu.render_graph.borrow().get_passes().iter() {
            for cmd in pass.get_cmds() {
                // model matrix
                *gpu.database.borrow_mut().uniforms
                    .get_mat4_mut(gpu.globals.global_uniforms.model_matrix)
                    .expect("Model matrix uniform missing") = cmd.transform.to_matrix();
                
                // view matrix
                *gpu.database.borrow_mut().uniforms
                    .get_mat4_mut(gpu.globals.global_uniforms.view_matrix)
                    .expect("View matrix uniform missing") = pass.render_state.view;

                // projection matrix
                *gpu.database.borrow_mut().uniforms
                    .get_mat4_mut(gpu.globals.global_uniforms.projection_matrix)
                    .expect("Perspective matrix uniform missing") = if cmd.perspective { perspective_matrix } else { ortho_matrix };

                *gpu.database.borrow_mut().uniforms
                    .get_boolean_mut(gpu.globals.global_uniforms.enable_lighting)
                    .expect("Enable lighting uniform missing") = pass.render_state.enable_lighting;

                *gpu.database.borrow_mut().uniforms
                    .get_boolean_mut(gpu.globals.global_uniforms.enable_fog)
                    .expect("Enable fog uniform missing") = pass.render_state.enable_fog;

                *gpu.database.borrow_mut().uniforms
                    .get_float_mut(gpu.globals.global_uniforms.fog_start)
                    .expect("Fog start uniform missing") = pass.render_state.fog_start;

                *gpu.database.borrow_mut().uniforms
                    .get_float_mut(gpu.globals.global_uniforms.fog_end)
                    .expect("Fog end uniform missing") = pass.render_state.fog_end;

                let db = gpu.database.borrow();
                let mesh = db.assets
                    .get_mesh(cmd.mesh)
                    .expect("Mesh asset not found");

                let gpu_mesh = self.gpu_assets
                    .get_mesh(cmd.mesh)
                    .expect("Gpu mesh not found");

                let material = db.assets
                    .get_material(mesh.material)
                    .expect("Material not found");

                let uniform_values = material
                    .get_uniform_values(&db.uniforms, &self.gpu_assets, pass)
                    .expect("Unable to generate uniform values");

                let program = self.gpu_assets
                    .get_program(material.program)
                    .expect("Program not found");

                let draw_params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    blend: glium::draw_parameters::Blend::alpha_blending(),
                    .. Default::default()
                };

                if let Some(gl_index_buffer) = &gpu_mesh.index_buffer {
                    framebuffer.draw(
                        &gpu_mesh.vertex_buffer,
                        gl_index_buffer,
                        &program.gl, 
                        &uniform_values,
                        &draw_params
                    ).unwrap();
                }
                else {
                    framebuffer.draw(
                        &gpu_mesh.vertex_buffer,
                        glium::index::NoIndices(
                            glium::index::PrimitiveType::from(
                                mesh.primitive_type
                            )
                        ),
                        &program.gl, 
                        &uniform_values,
                        &draw_params
                    ).unwrap();
                }
            }
        }
    }

    pub fn blit_buffers_to_frame(&self, framebuffer: &SimpleFrameBuffer, frame: &mut Frame) {
        let framebuffer_dimensions = framebuffer.get_dimensions();
        let scale = frame.get_dimensions().1 as f32 / framebuffer_dimensions.1 as f32;
        let new_width = framebuffer_dimensions.0 as f32 * scale;
        let new_x_pos = (frame.get_dimensions().0 as f32 - new_width) as f32 / 2.0;

        frame.blit_buffers_from_simple_framebuffer(
            &framebuffer,
            &Rect {
                left: 0, 
                bottom: 0, 
                width: framebuffer_dimensions.0, 
                height: framebuffer_dimensions.1
            }, 
            &BlitTarget {
                left: new_x_pos as u32, 
                bottom: 0, 
                width: new_width as i32, 
                height: frame.get_dimensions().1 as i32
            }, 
            uniforms::MagnifySamplerFilter::Nearest, 
            BlitMask::color_and_depth()
        );
    }

    pub fn post_render(&self, gpu: &mut GraphicsChip) {
        *gpu.database.borrow_mut().uniforms
            .get_mat4_mut(gpu.globals.global_uniforms.view_matrix)
            .expect("View matrix uniform missing") = Mat4::IDENTITY;
    }
}