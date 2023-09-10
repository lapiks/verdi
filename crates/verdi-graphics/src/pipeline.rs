use std::ops::{Deref, DerefMut};

use verdi_database::{Resource, ResourceId, Assets, Handle};

use crate::{
    gpu_assets::{PrepareAsset, GpuAsset, GpuAssetError, GpuAssets}, 
    gpu_pipeline::GpuPipeline, 
    gpu_program::GpuProgram, program::ProgramId
};

pub type PipelineId = ResourceId;

pub struct Pipeline {
    program: ProgramId,
}

impl Pipeline {
    pub fn new(program: ProgramId) -> Self {
        Self {
            program,
        }
    }
}

impl Resource for Pipeline {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl PrepareAsset for Pipeline {
    fn prepare_rendering(&self, ctx: &mut dyn miniquad::RenderingBackend, assets: &Assets, gpu_assets: &GpuAssets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
        if let Some(gpu_program) = gpu_assets.get::<GpuProgram>(self.program) {
            let params = miniquad::PipelineParams {
                color_blend: Some(miniquad::BlendState::new(
                    miniquad::Equation::Add,
                    miniquad::BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
                    miniquad::BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
                )),
                depth_test: miniquad::Comparison::LessOrEqual,
                depth_write: true,
                ..Default::default()
            };

            let pipeline = ctx.new_pipeline_with_params(
                &[miniquad::BufferLayout::default()],
                &[
                    miniquad::VertexAttribute::new("position", miniquad::VertexFormat::Float3),
                    miniquad::VertexAttribute::new("normal", miniquad::VertexFormat::Float3),
                    miniquad::VertexAttribute::new("color", miniquad::VertexFormat::Float4),
                    miniquad::VertexAttribute::new("uv", miniquad::VertexFormat::Float2),
                ],
                gpu_program.get_shader(),
                params,
            );

            return Ok(Box::new(
                GpuPipeline::new(pipeline)
            ));
        }

        Err(GpuAssetError::PreparationFailed)
    }
}

#[derive(Clone)]
pub struct PipelineHandle(Handle);

impl Deref for PipelineHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PipelineHandle {
      fn deref_mut(&mut self) -> &mut Handle {
        &mut self.0
    }
}

impl PipelineHandle {
    pub fn new(assets: Assets, id: PipelineId) -> Self {
        PipelineHandle(assets.new_handle(id))
    }
}