use glium::{uniforms::SamplerBehavior, texture::SrgbTexture2d};
use verdi_database::Resource;

use crate::gpu_assets::GpuAsset;

pub struct GpuImage { 
    gl: SrgbTexture2d,
    sampler: SamplerBehavior,
}

impl GpuImage {
    pub fn new(gl: glium::texture::SrgbTexture2d, sampler: SamplerBehavior) -> Self {    
        Self {
            gl,
            sampler,
        }
    }

    pub fn get_gl_texture(&self) -> &SrgbTexture2d {
        &self.gl
    }

    pub fn get_gl_sampler(&self) -> &SamplerBehavior {
        &self.sampler
    }
}

impl Resource for GpuImage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl GpuAsset for GpuImage {}