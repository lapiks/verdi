use std::ops::Deref;
use glium::{Display, framebuffer::DepthRenderBuffer};
use mlua::UserData;
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::gpu_assets::{GpuAsset, GpuAssetError, PrepareAsset, GpuAssets, 
};


#[derive(Clone)]
pub struct DepthBuffer {
    width: u32,
    height: u32,
    pub id: ResourceId,
}

impl Resource for DepthBuffer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl DepthBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { 
            width: width, 
            height: height,
            id: ResourceId::null(),
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        return (self.width, self.height)
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl PrepareAsset for DepthBuffer {
    fn prepare_rendering(&self, ctx: &Display, assets: &Assets, gpu_assets: &GpuAssets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
        if let Ok(gl_depth) = DepthRenderBuffer::new(
            ctx,
            glium::texture::DepthFormat::I24,
            self.width,
            self.height
        ) {
            return Ok(
                Box::new(
                    GpuDepthBuffer::new(gl_depth)
                )
            )
        }

        Err(GpuAssetError::PreparationFailed)
    }
}

#[derive(Clone)]
pub struct DepthBufferHandle(Handle);

impl Deref for DepthBufferHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DepthBufferHandle {
    pub fn new(assets: Assets, id: ResourceId) -> Self {
        DepthBufferHandle(assets.new_handle(id))
    }
}

impl UserData for DepthBufferHandle {}

pub struct GpuDepthBuffer { 
    gl: DepthRenderBuffer,
}

impl GpuDepthBuffer {
    pub fn new(gl: DepthRenderBuffer) -> Self {    
        Self { gl }
    }

    pub fn get_gl_depth_buffer(&self) -> &DepthRenderBuffer {
        &self.gl
    }
}

impl Resource for GpuDepthBuffer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl GpuAsset for GpuDepthBuffer {}