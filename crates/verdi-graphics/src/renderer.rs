use glium::{Surface, uniform};

use crate::{vertex::Vertex, render_pass::RenderPass, prelude::GraphicsChip};

pub struct Renderer {
    display: glium::Display,
    program: glium::Program
}

impl Renderer {
    pub fn new(display: glium::Display) -> Result<Self, std::io::Error> {
        // TODO gérer erreurs avec GraphicsChipError
        let gouraud_vs = match std::fs::read_to_string( "./crates/verdi-graphics/shaders/gouraud.vs") {
            Ok(gouraud_vs)  => gouraud_vs,
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        };

        let gouraud_fs = match std::fs::read_to_string("./crates/verdi-graphics/shaders/gouraud.fs") {
            Ok(gouraud_fs)  => gouraud_fs,
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        };
        
        let program = glium::Program::from_source(
            &display, 
            gouraud_vs.as_str(), 
            gouraud_fs.as_str(), 
            None
        ).unwrap();

        Ok(Self { display ,program })
    }

    pub fn render(&mut self, gpu: &GraphicsChip) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // uniforms (à bouger)
        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        // the direction of the light
        let light = [-1.0, 0.4, 0.9f32];

        let vertex1 = Vertex { position: [-0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], ..Vertex::default() };
        let vertex2 = Vertex { position: [ 0.0,  0.5, 0.0], normal: [0.0, 0.0, 1.0], ..Vertex::default()};
        let vertex3 = Vertex { position: [ 0.5, -0.25, 0.0], normal: [0.0, 0.0, 1.0], ..Vertex::default()};
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        for render_pass in gpu.render_passes.lock().unwrap().iter() {
            target.draw(
                //&render_pass.vertex_buffer,
                &vertex_buffer,
                &indices, 
                &self.program, 
                &uniform! { matrix: matrix, u_light: light },
                &Default::default()
            ).unwrap();
        }        

        target.finish().unwrap();
    }
}