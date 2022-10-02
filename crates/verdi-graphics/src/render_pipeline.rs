use verdi_math::Mat4;

use crate::{
    uniforms::{UniformId, Uniforms}, 
    render_pass::RenderPass
};

pub struct RenderPipeline {
    pub model_matrix: UniformId,
    pub view_matrix: UniformId,
    pub perspective_matrix: UniformId,
    pub render_passes: Vec<RenderPass>,
}

impl RenderPipeline {
    pub fn new(uniforms: &mut Uniforms) -> Self{
        Self {
            model_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            view_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            perspective_matrix: uniforms.add_mat4(Mat4::IDENTITY),
            render_passes: Vec::new(),
        }
        
    }
}