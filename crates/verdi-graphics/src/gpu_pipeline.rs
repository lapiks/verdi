use verdi_database::Resource;

use crate::gpu_assets::GpuAsset;


pub struct GpuPipeline {
    pipeline: miniquad::Pipeline,
}

impl GpuPipeline {
    pub fn new(quad_pipeline: miniquad::Pipeline) -> Self {
        Self {
            pipeline: quad_pipeline,
        }
    }

    pub fn get_quad_pipeline(&self) -> &miniquad::Pipeline {
        &self.pipeline
    } 
}

impl Resource for GpuPipeline {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl GpuAsset for GpuPipeline {}