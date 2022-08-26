use glium::{glutin, Surface, uniform};
use crate::vertex::Vertex;

pub struct GraphicsChip {
    display: glium::Display,
    gouraud_program: glium::Program,
    vertex_buffer: Vec<Vertex>,
}

pub enum GraphicsChipError {
    ProgramCreation,
    ShaderParsing,
}

impl GraphicsChip {
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
        
        let gouraud_program = glium::Program::from_source(
            &display, 
            gouraud_vs.as_str(), 
            gouraud_fs.as_str(), 
            None
        ).unwrap();

        let vertex_buffer = Vec::new();

        Ok(Self {
            display,
            gouraud_program,
            vertex_buffer,
        })
    }

    pub fn init() {
        
    }

    pub fn render(&mut self) {
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

        let vertex1 = Vertex { position: [-0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0] };
        let vertex2 = Vertex { position: [ 0.0,  0.5, 0.0], normal: [0.0, 0.0, 1.0] };
        let vertex3 = Vertex { position: [ 0.5, -0.25, 0.0], normal: [0.0, 0.0, 1.0] };
        self.vertex_buffer = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &self.vertex_buffer).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        target.draw(
            &vertex_buffer, 
            &indices, 
            &self.gouraud_program, 
            &uniform! { matrix: matrix, u_light: light },
            &Default::default()
        ).unwrap();

        target.finish().unwrap();
    }
}