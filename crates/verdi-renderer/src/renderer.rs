use verdi_window::Window;

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration
}

impl Renderer
{
    pub async fn new(window: &Window) -> Self {
        // The instance is the first thing you create when using wgpu. Its main purpose is to create Adapters and Surfaces
        let instance = wgpu::Instance::new(wgpu::Backends::all());
    
        // The surface is the part of the window that we draw to
        let surface = unsafe { instance.create_surface(&window.internal_window) };
    
        // The adapter is a handle to our actual graphics card
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
    
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
    
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: window.width,
            height: window.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);
    
        Self {
            surface,
            device,
            queue,
            config
        }
    }

    pub fn on_window_resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.config.width = new_width;
            self.config.height = new_height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
