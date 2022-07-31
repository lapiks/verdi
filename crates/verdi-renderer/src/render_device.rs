
pub struct RenderDevice {
    pub device: wgpu::Device,
}

impl From<wgpu::Device> for RenderDevice {
    fn from(device: wgpu::Device) -> Self {
        Self { device }
    }
}