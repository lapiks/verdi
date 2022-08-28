use crate::{vertex::Vertex, render_pass::RenderPass};
use verdi_math::prelude::*;
use std::{cell::RefCell, sync::{Arc, Mutex}};

type RenderPasses = Mutex<Vec<RenderPass>>;

pub struct GraphicsChip {
    pub render_passes: Arc<RenderPasses>
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
    pub fn new(display: &glium::Display) -> Self {
        let render_passes = Arc::new(Mutex::new(Vec::new()));
        Self { render_passes }
    }

    pub fn begin(&self, primitive_type: PrimitiveType) {
        let render_pass = RenderPass::new(
            Vec::new(), 
            Vertex::default(), 
            primitive_type
        );

        self.render_passes.lock().unwrap().push(render_pass);
    }

    pub fn end(&self) {
        match self.render_passes.lock().unwrap().last_mut() {
            Some(render_pass) => {
                render_pass.current_vertex_state = Vertex::default();
                render_pass.vertex_buffer.clear();
            },
            None => return
        };
    }

    // pub fn vertex(&self, coords: Vec3) {
    //     match self.render_passes.borrow_mut().last_mut() {
    //         Some(render_pass) => {
    //             render_pass.current_vertex_state.position = coords.to_array();
    //             render_pass.vertex_buffer.push(render_pass.current_vertex_state);
    //         },
    //         None => return
    //     };
    // }

    // pub fn normal(&self, coords: Vec3) {
    //     match self.render_passes.borrow_mut().last_mut() {
    //         Some(render_pass) => {
    //             render_pass.current_vertex_state.normal = coords.to_array();
    //         },
    //         None => return
    //     };
    // }

    // pub fn tex_coord(&self, coords: Vec2) {
    //     match self.render_passes.borrow_mut().last_mut() {
    //         Some(render_pass) => {
    //             render_pass.current_vertex_state.uv = coords.to_array();
    //         },
    //         None => return
    //     };
    // }

    // pub fn color(&self, coords: Vec4) {
    //     match self.render_passes.borrow_mut().last_mut() {
    //         Some(render_pass) => {
    //             render_pass.current_vertex_state.color = coords.to_array();
    //         },
    //         None => return
    //     };
    // }
}