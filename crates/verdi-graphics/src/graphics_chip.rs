use glium::{glutin, Surface, uniform};
use crate::{vertex::Vertex, program};
use verdi_math::prelude::*;
pub struct GraphicsChip {
    display: glium::Display,
    gouraud_program: glium::Program,
    vertex_buffer: Vec<Vertex>,
    current_vertex_state: Vertex,
    current_primitive: PrimitiveType
}

pub enum GraphicsChipError {
    ProgramCreation,
    ShaderParsing,
}

pub enum PrimitiveType {
    triangles,
    points,
    lines,
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

        let current_vertex_state = Vertex::default();

        let current_primitive = PrimitiveType::triangles;

        Ok(Self {
            display,
            gouraud_program,
            vertex_buffer,
            current_vertex_state,
            current_primitive
        })
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

        let vertex1 = Vertex { position: [-0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], ..Vertex::default() };
        let vertex2 = Vertex { position: [ 0.0,  0.5, 0.0], normal: [0.0, 0.0, 1.0], ..Vertex::default()};
        let vertex3 = Vertex { position: [ 0.5, -0.25, 0.0], normal: [0.0, 0.0, 1.0], ..Vertex::default()};
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

    pub fn begin(&mut self, primitive_type: PrimitiveType) {
        self.current_primitive = primitive_type;
    }

    pub fn end(&mut self) {
        self.render();
        self.current_vertex_state = Vertex::default();
        self.vertex_buffer.clear();
    }

    pub fn vertex(&mut self, coords: Vec3) {
        self.current_vertex_state.position = coords.to_array();
        self.vertex_buffer.push(self.current_vertex_state);
    }

    pub fn normal(&mut self, coords: Vec3) {
        self.current_vertex_state.normal = coords.to_array();
    }

    pub fn tex_coord(&mut self, coords: Vec2) {
        self.current_vertex_state.uv = coords.to_array();
    }

    pub fn color(&mut self, coords: Vec4) {
        self.current_vertex_state.color = coords.to_array();
    }
}