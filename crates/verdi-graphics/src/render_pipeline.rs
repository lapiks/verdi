use verdi_math::{Mat4, Vec2, Vec4};

use crate::{
    uniforms::{UniformId, Uniforms}, 
    render_pass::RenderPass
};

pub struct RenderPipeline {
    pub model_matrix: UniformId,
    pub view_matrix: UniformId,
    pub perspective_matrix: UniformId,
    pub resolution: UniformId,
    pub render_passes: Vec<RenderPass>,
    pub clear_color: Vec4,
}

impl RenderPipeline {
    pub fn new(uniforms: &mut Uniforms) -> Self{
        Self {
            model_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            view_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            perspective_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            resolution: uniforms.add_vec2(Vec2::ZERO),
            render_passes: Vec::new(),
            clear_color: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
        
    }
}