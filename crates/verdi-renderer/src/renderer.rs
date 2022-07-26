use verdi_window::Window;

pub async fn initialise(window: &Window) {
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
}