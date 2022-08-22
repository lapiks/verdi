use glium::uniform;

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        
    };
}

mod vertex;
mod vertex_buffer;
mod index_buffer;
mod program;
mod shader;
mod render_pass;
mod teapot;

extern crate glium;

pub fn run() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();

    let vertex_shader_src =  r#"
        #version 140

        in vec3 position;
        in vec3 normal;
        
        uniform mat4 matrix;
        
        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;
    //let vertex_shader = shader::Shader::new(vertex_shader_src.to_string());

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    //let fragment_shader = shader::Shader::new(fragment_shader_src.to_string());

    //let program = program::Program::new(&display, &vertex_shader, &fragment_shader);
    
    let program = match glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None) {
        Ok(program) => program,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    
    let matrix = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(
            (&positions, &normals), 
            &indices, 
            &program, 
            &uniform! { matrix: matrix },
            &Default::default()
        ).unwrap();

        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}