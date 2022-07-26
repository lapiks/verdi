pub struct RenderDevice {
    device: wgpu::Device,
    queue: wgpu::Queue
}

impl RenderDevice {
    pub async fn new(adapter: &wgpu::Adapter) -> RenderDevice {
        let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        Self { device: device, queue: queue }
    }
}