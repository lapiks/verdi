use wgpu::util::DeviceExt;

pub struct RenderDevice {
    pub device: wgpu::Device,
}

impl From<wgpu::Device> for RenderDevice {
    fn from(device: wgpu::Device) -> Self {
        Self { device }
    }
}

impl RenderDevice  {
    pub fn create_shader_module(&self, ) -> wgpu::ShaderModule {
        self.device.create_shader_module(wgpu::include_wgsl!("shader/shader.wgsl"))
    }

    pub fn create_pipeline_layout(&self) -> wgpu::PipelineLayout {
        self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        })
    }

    pub fn create_render_pipeline(&self, descriptor: &wgpu::RenderPipelineDescriptor) -> wgpu::RenderPipeline {
        self.device.create_render_pipeline(descriptor)
    }

    pub fn create_buffer(&self, descriptor: &wgpu::util::BufferInitDescriptor) -> wgpu::Buffer {
        self.device.create_buffer_init(descriptor)
    }
}