use glium::{
    Surface, 
    framebuffer::SimpleFrameBuffer, 
    Frame, 
    Rect, 
    BlitTarget, 
    BlitMask, 
    uniforms
};

use verdi_math::{Mat4, Vec2, prelude::Transform};

use crate::{
    camera::Camera,
    prelude::GraphicsChip,
    mesh::Mesh, 
    material::Material, 
    uniform::Uniform,
    gpu_mesh::GpuMesh, 
    gpu_program::GpuProgram, 
};

// Le renderer pourrait être plus bas niveau. 
// Une fonction render() pourrait prendre en paramètre un Renderable définissant toutes les infos nécessaire pour rendre un mesh.

/// Low level interface to GPU. 
/// Given some renderable GPU resources, the Renderer is able to draw them using the render function. 
pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {        
        Self {}
    }

    pub fn render(&mut self, framebuffer: &mut SimpleFrameBuffer, gpu: &mut GraphicsChip) {
        let global_uniforms = &gpu.globals.global_uniforms;
        let mut assets = gpu.assets.borrow_mut();
        let gpu_assets = gpu.gpu_assets.borrow_mut();

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

        assets.get_mut::<Uniform<Vec2>>(global_uniforms.resolution)
            .expect("Resolution uniform missing")
            .value = Vec2::new(
                target_dimensions.0 as f32, 
                target_dimensions.1 as f32
            );

        for pass in gpu.render_graph.borrow().get_passes().iter() {
            for cmd in pass.get_cmds() {
                // get transform
                let transform = assets
                    .get::<Transform>(cmd.transform.get_id())
                    .expect("Transform missing");

                // model matrix
                assets.get_mut::<Uniform<Mat4>>(global_uniforms.model_matrix)
                    .expect("Model matrix uniform missing")
                    .value = transform.to_matrix();
                
                // view matrix
                assets.get_mut::<Uniform<Mat4>>(global_uniforms.view_matrix)
                    .expect("View matrix uniform missing")
                    .value = pass.render_state.view;

                // projection matrix
                assets.get_mut::<Uniform<Mat4>>(global_uniforms.projection_matrix)
                    .expect("Perspective matrix uniform missing")
                    .value = if cmd.perspective { perspective_matrix } else { ortho_matrix };

                assets.get_mut::<Uniform<bool>>(global_uniforms.enable_lighting)
                    .expect("Enable lighting uniform missing")
                    .value = pass.render_state.enable_lighting;

                assets.get_mut::<Uniform<bool>>(global_uniforms.enable_fog)
                    .expect("Enable fog uniform missing")
                    .value = pass.render_state.enable_fog;

                assets.get_mut::<Uniform<f32>>(global_uniforms.fog_start)
                    .expect("Fog start uniform missing")
                    .value = pass.render_state.fog_start;

                assets.get_mut::<Uniform<f32>>(global_uniforms.fog_end)
                    .expect("Fog end uniform missing")
                    .value = pass.render_state.fog_end;

                let mesh = assets
                    .get::<Mesh>(cmd.mesh.get_id())
                    .expect("Mesh resource not found");

                let gpu_mesh = gpu_assets
                    .get::<GpuMesh>(cmd.mesh.get_id())
                    .expect("Gpu mesh not found");

                let material = assets
                    .get::<Material>(mesh.material)
                    .expect("Material not found");

                let uniform_values = material
                    .get_uniform_values(&assets, pass)
                    .expect("Unable to generate uniform values");

                let program = gpu_assets
                    .get::<GpuProgram>(material.program)
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
        gpu.assets.borrow_mut()
            .get_mut::<Uniform<Mat4>>(gpu.globals.global_uniforms.view_matrix)
            .expect("View matrix uniform missing")
            .value = Mat4::IDENTITY;
    }
}