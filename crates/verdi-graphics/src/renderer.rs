use verdi_math::{Mat4, Vec2, prelude::Transform};

use crate::{
    camera::Camera,
    prelude::GraphicsChip,
    mesh::Mesh, 
    material::Material, 
    uniform::{Uniform, UniformValue},
    gpu_mesh::GpuMesh, 
    render_target::RenderTarget, 
    gpu_pipeline::GpuPipeline, image::Image, gpu_image::GpuImage, 
};

#[repr(C)]
struct TestUniforms {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    resolution: Vec2,
    enable_fog: bool,
    fog_start: f32,
    fog_end: f32,
    enable_lighting: bool,
}

// Le renderer pourrait être plus bas niveau. 
// Une fonction render() pourrait prendre en paramètre un Renderable définissant toutes les infos nécessaire pour rendre un mesh.

/// Low level interface to GPU. 
/// Given some renderable GPU resources, the Renderer is able to draw them using the render function. 
pub struct Renderer {
    uniforms: TestUniforms,
}

impl Renderer {
    pub fn new() -> Self {        
        Self {
            uniforms : TestUniforms {
                model: Mat4::IDENTITY,
                view: Mat4::IDENTITY,
                projection: Mat4::IDENTITY,
                resolution: Vec2::ZERO,
                enable_fog: true,
                fog_start: 0.0,
                fog_end: 0.0,
                enable_lighting: true,
            }
        }
    }

    pub fn render(&mut self, ctx: &mut dyn miniquad::RenderingBackend, render_target: &RenderTarget, gpu: &mut GraphicsChip) {
        let global_uniforms = &gpu.globals.global_uniforms;
        let gpu_assets = &gpu.gpu_assets;

        let render_pass = ctx.new_render_pass(
            render_target.get_color_target(),
            Some(render_target.get_depth_target())
        );

        let target_dimensions = render_target.get_dimensions();

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

        ctx.begin_pass(
            Some(render_pass), 
            miniquad::PassAction::clear_color(1.0, 1.0, 1.0, 1.0)
        );

        for pass in gpu.render_graph.borrow().get_passes().iter() {
            for cmd in pass.get_cmds() {
                // get transform
                let transform_datas = cmd.transform.get_datas();
                let transform = transform_datas
                    .get::<Transform>(cmd.transform.get_id())
                    .expect("Transform missing");

                self.uniforms.model = transform.to_matrix();
                self.uniforms.view = pass.render_state.view;
                self.uniforms.projection = perspective_matrix;
                self.uniforms.resolution = Vec2::new(
                    target_dimensions.0 as f32, 
                    target_dimensions.1 as f32
                );
                self.uniforms.fog_start  = pass.render_state.fog_start;
                self.uniforms.fog_end= pass.render_state.fog_end;
                self.uniforms.enable_lighting= pass.render_state.enable_lighting;

                {
                    let mut asset_datas = gpu.assets.get_datas_mut();
                    asset_datas
                        .get_mut::<Uniform>(global_uniforms.resolution.get_id())
                        .expect("Resolution uniform missing")
                        .value = UniformValue::Vec2(
                            Vec2::new(
                                target_dimensions.0 as f32, 
                                target_dimensions.1 as f32
                            )
                        );
    
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
                        .value = UniformValue::Mat4(
                            if cmd.perspective { 
                                perspective_matrix 
                            } 
                            else { 
                                ortho_matrix
                            }
                        );
    
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
                }
                

                let asset_datas = gpu.assets.get_datas();
                let mesh = asset_datas
                    .get::<Mesh>(cmd.mesh.get_id())
                    .expect("Mesh resource not found");

                let gpu_mesh = gpu_assets
                    .get::<GpuMesh>(cmd.mesh.get_id())
                    .expect("Gpu mesh not found");

                let material = asset_datas
                    .get::<Material>(mesh.material)
                    .expect("Material not found");

                let uniform_values = material
                    .get_uniform_values()
                    .expect("Unable to generate uniform values");

                
                let mut quad_textures: Vec<miniquad::TextureId> = vec![];
                for texture_id in material.get_textures() {
                    if let Some(gpu_texture) = gpu_assets.get::<GpuImage>(*texture_id) {
                        quad_textures.push(gpu_texture.get_quad_texture());
                    }
                }

                let pipeline = gpu_assets
                    .get::<GpuPipeline>(gpu.globals.global_pipelines.default_pipeline.get_id())
                    .expect("Pipeline not found");

                let bindings = miniquad::Bindings {
                    vertex_buffers: vec![gpu_mesh.vertex_buffer],
                    index_buffer: gpu_mesh.index_buffer,
                    images: quad_textures,
                };

                ctx.apply_pipeline(pipeline.get_quad_pipeline());
                ctx.apply_bindings(&bindings);
                //ctx.apply_uniforms_from_bytes(uniform_values.as_ptr(), uniform_values.len());
                ctx.apply_uniforms(miniquad::UniformsSource::table(&self.uniforms));

                ctx.draw(0, mesh.indices.len() as i32, 1);

                ctx.end_render_pass();

                ctx.commit_frame();
            }
        }
    }

    // pub fn blit_buffers_to_frame(&self, framebuffer: &SimpleFrameBuffer, frame: &mut Frame) {
    //     let framebuffer_dimensions = framebuffer.get_dimensions();
    //     let scale = frame.get_dimensions().1 as f32 / framebuffer_dimensions.1 as f32;
    //     let new_width = framebuffer_dimensions.0 as f32 * scale;
    //     let new_x_pos = (frame.get_dimensions().0 as f32 - new_width) as f32 / 2.0;

    //     frame.blit_buffers_from_simple_framebuffer(
    //         &framebuffer,
    //         &Rect {
    //             left: 0, 
    //             bottom: 0, 
    //             width: framebuffer_dimensions.0, 
    //             height: framebuffer_dimensions.1
    //         }, 
    //         &BlitTarget {
    //             left: new_x_pos as u32, 
    //             bottom: 0, 
    //             width: new_width as i32, 
    //             height: frame.get_dimensions().1 as i32
    //         }, 
    //         uniforms::MagnifySamplerFilter::Nearest, 
    //         BlitMask::color_and_depth()
    //     );
    // }

    pub fn post_render(&self, gpu: &mut GraphicsChip) {
        gpu.assets
            .get_datas_mut()
            .get_mut::<Uniform>(gpu.globals.global_uniforms.view_matrix.get_id())
            .expect("View matrix uniform missing")
            .value = UniformValue::Mat4(Mat4::IDENTITY);
    }
}