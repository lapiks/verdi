use std::ops::{Deref, DerefMut};

use verdi_database::{Resource, Assets, ResourceId, Handle};

use crate::{image::ImageHandle, depth_buffer::DepthBufferHandle};



pub struct Framebuffer {
    color_target: ImageHandle,
    depth_target: DepthBufferHandle,
}

impl Framebuffer {
    pub fn new(color_target: ImageHandle, depth_target: DepthBufferHandle) -> Self {
        Self {
            color_target,
            depth_target,
        }
    }

    pub fn get_color_target(&self) -> ImageHandle {
        self.color_target.clone()
    }

    pub fn get_depth_target(&self) -> DepthBufferHandle {
        self.depth_target.clone()
    }
}

impl Resource for Framebuffer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// impl PrepareAsset for Framebuffer {
//     fn prepare_rendering(&self, ctx: &Display, assets: &Assets, gpu_assets: &GpuAssets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
//         let gpu_color = gpu_assets.get::<GpuImage>(self.color_target.get_id()).unwrap();
//         let gpu_depth =  gpu_assets.get::<GpuDepthBuffer>(self.depth_target.get_id()).unwrap();

//         // create a framebuffer to draw into 
//         let mut gl_framebuffer = SimpleFrameBuffer::with_depth_buffer(
//             ctx, 
//             gpu_color.get_gl_texture(), 
//             gpu_depth.get_gl_depth_buffer()
//         ).unwrap();

//         Ok(
//             Box::new(
//                 GpuFramebuffer::new(gl_framebuffer)
//             )
//         )
//     }
// }

#[derive(Clone)]
pub struct FramebufferHandle(Handle);

impl Deref for FramebufferHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FramebufferHandle {
      fn deref_mut(&mut self) -> &mut Handle {
        &mut self.0
    }
}

impl FramebufferHandle {
    pub fn new(assets: Assets, id: ResourceId) -> Self {
        FramebufferHandle(assets.new_handle(id))
    }
}