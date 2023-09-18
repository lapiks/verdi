use glium::{
    framebuffer::SimpleFrameBuffer, uniforms, BlitMask, BlitTarget, Display, Frame, Rect, Surface,
};
use verdi_math::{prelude::Transform, Mat4, Vec2};

use crate::{
    camera::Camera,
    depth_buffer::GpuDepthBuffer,
    framebuffer::Framebuffer,
    gpu_image::GpuImage,
    gpu_mesh::GpuMesh,
    gpu_program::GpuProgram,
    material::{GlUniformValues, Material},
    mesh::Mesh,
    pipeline::Pipeline,
    prelude::GraphicsChip,
    uniform::{Uniform, UniformValue}, image::Image,
};

// Le renderer pourrait être plus bas niveau.
// Une fonction render() pourrait prendre en paramètre un Renderable définissant toutes les infos nécessaire pour rendre un mesh.

/// Low level interface to GPU.
/// Given some renderable GPU resources, the Renderer is able to draw them using the render function.
pub struct Renderer {}

impl Renderer {
    pub fn render(&mut self, ctx: &Display, gpu: &mut GraphicsChip, frame: &mut Frame) {
        let global_uniforms = &gpu.globals.global_uniforms;
        let gpu_assets = &gpu.gpu_assets;

        for pass in gpu.render_graph.borrow().get_passes().iter() {
            let mut asset_datas = gpu.assets.get_datas_mut();
            let framebuffer = asset_datas
                .get::<Framebuffer>(pass.get_framebuffer().get_id())
                .expect("Framebuffer missing");

            let gpu_color = gpu_assets
                .get::<GpuImage>(framebuffer.get_color_target().get_id())
                .unwrap();
            let gpu_depth = gpu_assets
                .get::<GpuDepthBuffer>(framebuffer.get_depth_target().get_id())
                .unwrap();

            let color = asset_datas
                .get::<Image>(framebuffer.get_color_target().get_id())
                .unwrap();

            // create a framebuffer to draw into
            let mut gl_framebuffer = SimpleFrameBuffer::with_depth_buffer(
                ctx,
                gpu_color.get_gl_texture(),
                gpu_depth.get_gl_depth_buffer(),
            )
            .unwrap();

            let clear_color = gpu.render_state.clear_color;
            gl_framebuffer.clear_color_and_depth(
                (clear_color.x, clear_color.y, clear_color.z, clear_color.w),
                1.0,
            );

            let target_dimensions = color.get_dimensions();

            // perspective matrix
            let perspective_matrix =
            Camera::perspective_matrix(target_dimensions.0, target_dimensions.1);

            // ortho matrix
            let ortho_matrix = Camera::orthographic_matrix(
                0.0,
                target_dimensions.0 as f32,
                target_dimensions.1 as f32,
                0.0,
                -10.0,
                10.0,
            );

            for cmd in pass.get_cmds() {
                // get transform
                let transform_datas = cmd.transform.get_datas();
                let transform = transform_datas
                    .get::<Transform>(cmd.transform.get_id())
                    .expect("Transform missing");

                //let mut asset_datas = gpu.assets.get_datas_mut();
                asset_datas
                    .get_mut::<Uniform>(global_uniforms.resolution.get_id())
                    .expect("Resolution uniform missing")
                    .value = UniformValue::Vec2(Vec2::new(
                    target_dimensions.0 as f32,
                    target_dimensions.1 as f32,
                ));

                // model matrix
                asset_datas
                    .get_mut::<Uniform>(global_uniforms.model_matrix.get_id())
                    .expect("Model matrix uniform missing")
                    .value = UniformValue::Mat4(transform.to_matrix());

                // view matrix
                asset_datas
                    .get_mut::<Uniform>(global_uniforms.view_matrix.get_id())
                    .expect("View matrix uniform missing")
                    .value = UniformValue::Mat4(pass.render_state.view);

                // projection matrix
                asset_datas
                    .get_mut::<Uniform>(global_uniforms.projection_matrix.get_id())
                    .expect("Perspective matrix uniform missing")
                    .value = UniformValue::Mat4(if cmd.perspective {
                    perspective_matrix
                } else {
                    ortho_matrix
                });

                asset_datas
                    .get_mut::<Uniform>(global_uniforms.enable_lighting.get_id())
                    .expect("Enable lighting uniform missing")
                    .value = UniformValue::Bool(pass.render_state.enable_lighting);

                asset_datas
                    .get_mut::<Uniform>(global_uniforms.enable_fog.get_id())
                    .expect("Enable fog uniform missing")
                    .value = UniformValue::Bool(pass.render_state.enable_fog);

                asset_datas
                    .get_mut::<Uniform>(global_uniforms.fog_start.get_id())
                    .expect("Fog start uniform missing")
                    .value = UniformValue::Float(pass.render_state.fog_start);

                asset_datas
                    .get_mut::<Uniform>(global_uniforms.fog_end.get_id())
                    .expect("Fog end uniform missing")
                    .value = UniformValue::Float(pass.render_state.fog_end);

                //let asset_datas = gpu.assets.get_datas();
                let mesh = asset_datas
                    .get::<Mesh>(cmd.mesh.get_id())
                    .expect("Mesh resource not found");

                let gpu_mesh = gpu_assets
                    .get::<GpuMesh>(cmd.mesh.get_id())
                    .expect("Gpu mesh not found");

                let material = asset_datas
                    .get::<Material>(mesh.material)
                    .expect("Material not found");

                let mut uniform_values = [None; 64];

                for (uniform_value, uniform_handle) in
                    uniform_values.iter_mut().zip(material.get_uniforms())
                {
                    if let Some((name, handle)) = uniform_handle.clone() {
                        if let Some(uniform) = asset_datas.get::<Uniform>(handle.get_id()) {
                            *uniform_value = Some((name, uniform.get_gl_value(gpu_assets)));
                        }
                    } else {
                        break;
                    }
                }

                let gl_uniform_values = GlUniformValues { uniform_values };

                let pipeline = asset_datas
                    .get::<Pipeline>(gpu.globals.global_pipelines.default_pipeline.get_id())
                    .expect("Pipeline not found");

                let gpu_program = gpu_assets
                    .get::<GpuProgram>(pipeline.get_program().get_id())
                    .expect("GPU Program not found");

                let draw_params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    blend: glium::draw_parameters::Blend::alpha_blending(),
                    ..Default::default()
                };

                if let Some(gl_index_buffer) = &gpu_mesh.get_index_buffer() {
                    gl_framebuffer
                        .draw(
                            gpu_mesh.get_vertex_buffer(),
                            gl_index_buffer,
                            gpu_program.get_gl_program(),
                            &gl_uniform_values,
                            &draw_params,
                        )
                        .unwrap();
                } else {
                    gl_framebuffer
                        .draw(
                            gpu_mesh.get_vertex_buffer(),
                            glium::index::NoIndices(glium::index::PrimitiveType::from(
                                mesh.primitive_type,
                            )),
                            gpu_program.get_gl_program(),
                            &gl_uniform_values,
                            &draw_params,
                        )
                        .unwrap();
                }
            }

            let scale = frame.get_dimensions().1 as f32 / target_dimensions.1 as f32;
            let new_width = target_dimensions.0 as f32 * scale;
            let new_x_pos = (frame.get_dimensions().0 as f32 - new_width) as f32 / 2.0;

            frame.blit_buffers_from_simple_framebuffer(
                &gl_framebuffer,
                &Rect {
                    left: 0,
                    bottom: 0,
                    width: target_dimensions.0,
                    height: target_dimensions.1,
                },
                &BlitTarget {
                    left: new_x_pos as u32,
                    bottom: 0,
                    width: new_width as i32,
                    height: frame.get_dimensions().1 as i32,
                },
                uniforms::MagnifySamplerFilter::Nearest,
                BlitMask::color_and_depth(),
            );
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
                height: framebuffer_dimensions.1,
            },
            &BlitTarget {
                left: new_x_pos as u32,
                bottom: 0,
                width: new_width as i32,
                height: frame.get_dimensions().1 as i32,
            },
            uniforms::MagnifySamplerFilter::Nearest,
            BlitMask::color_and_depth(),
        );
    }

    pub fn post_render(&self, gpu: &mut GraphicsChip) {
        gpu.assets
            .get_datas_mut()
            .get_mut::<Uniform>(gpu.globals.global_uniforms.view_matrix.get_id())
            .expect("View matrix uniform missing")
            .value = UniformValue::Mat4(Mat4::IDENTITY);
    }
}
