use glium::uniform;

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        graphics_chip::GraphicsChip,
        renderer::Renderer,
    };
}

mod graphics_chip;
mod bind_graphics_chip;
mod vertex;
mod render_pass;
mod renderer;

// pub fn run() {
//     use glium::{glutin, Surface};

//     let event_loop = glutin::event_loop::EventLoop::new();
//     let wb = glutin::window::WindowBuilder::new();
//     let cb = glutin::ContextBuilder::new();
//     let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    
//     let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
//     let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
//     let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
//                                           &teapot::INDICES).unwrap();

//     let vertex_shader_src =  r#"
//         #version 150

//         in vec3 position;
//         in vec3 normal;
        
//         out vec3 v_normal;
        
//         uniform mat4 matrix;
        
//         void main() {
//             v_normal = transpose(inverse(mat3(matrix))) * normal;
//             gl_Position = matrix * vec4(position, 1.0);
//         }
//     "#;
//     //let vertex_shader = shader::Shader::new(vertex_shader_src.to_string());

//     let fragment_shader_src = r#"
//         #version 140

//         in vec3 v_normal;
//         out vec4 color;
//         uniform vec3 u_light;
        
//         void main() {
//             float brightness = dot(normalize(v_normal), normalize(u_light));
//             vec3 dark_color = vec3(0.6, 0.0, 0.0);
//             vec3 regular_color = vec3(1.0, 0.0, 0.0);
//             color = vec4(mix(dark_color, regular_color, brightness), 1.0);
//         }
//     "#;
//     //let fragment_shader = shader::Shader::new(fragment_shader_src.to_string());

//     //let program = program::Program::new(&display, &vertex_shader, &fragment_shader);
    
//     let program = match glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None) {
//         Ok(program) => program,
//         Err(e) => {
//             println!("{}", e.to_string());
//             return;
//         }
//     };

//     // uniforms
//     let matrix = [
//         [0.01, 0.0, 0.0, 0.0],
//         [0.0, 0.01, 0.0, 0.0],
//         [0.0, 0.0, 0.01, 0.0],
//         [0.0, 0.0, 0.0, 1.0f32]
//     ];

//     // the direction of the light
//     let light = [-1.0, 0.4, 0.9f32];

//     event_loop.run(move |ev, _, control_flow| {

//         let mut target = display.draw();
//         target.clear_color(0.0, 0.0, 1.0, 1.0);

//         target.draw(
//             (&positions, &normals), 
//             &indices, 
//             &program, 
//             &uniform! { matrix: matrix, u_light: light },
//             &Default::default()
//         ).unwrap();

//         target.finish().unwrap();

//         let next_frame_time = std::time::Instant::now() +
//             std::time::Duration::from_nanos(16_666_667);

//         *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
//         match ev {
//             glutin::event::Event::WindowEvent { event, .. } => match event {
//                 glutin::event::WindowEvent::CloseRequested => {
//                     *control_flow = glutin::event_loop::ControlFlow::Exit;
//                     return;
//                 },
//                 _ => return,
//             },
//             _ => (),
//         }
//     });
// }