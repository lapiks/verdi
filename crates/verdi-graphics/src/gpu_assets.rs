use std::any::Any;

use slotmap::SecondaryMap;
use verdi_database::{ResourceId, Resource, Assets};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GpuAssetError {
    #[error("Gpu asset creation failed")]
    PreparationFailed,
    #[error("Miniquad shader compilation error")]
    ShaderError(#[from] miniquad::ShaderError),
}

pub trait GpuAsset: Resource {}

pub(crate) trait PrepareAsset {
    fn prepare_rendering(&self, ctx: &mut dyn miniquad::RenderingBackend, assets: &Assets, gpu_assets: &GpuAssets) -> Result<Box<dyn GpuAsset>, GpuAssetError>;
}

pub struct GpuAssets(SecondaryMap<ResourceId, Box<dyn GpuAsset>>);

impl GpuAssets {
    pub fn new() -> Self {
        Self(SecondaryMap::default())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn add(&mut self, id: ResourceId, gpu_asset: Box<dyn GpuAsset>) {
        self.0.insert(id, gpu_asset);
    }

    pub fn get<A: Any>(&self, id: ResourceId) -> Option<&A> {
        match self.0.get(id) {
            Some(value) => {
                return value.as_any().downcast_ref();
            },
            None => return None,
        };
    }

    pub fn get_mut<A: Any>(&mut self, id: ResourceId) -> Option<&mut A> {
        match self.0.get_mut(id) {
            Some(value) => {
                return value.as_any_mut().downcast_mut();
            },
            None => return None,
        };
    }
}

impl Resource for GpuAssets {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}