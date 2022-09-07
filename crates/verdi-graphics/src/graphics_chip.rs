use crate::{vertex::Vertex, render_pass::RenderPass};
use verdi_math::prelude::*;
use std::{cell::RefCell, sync::{Arc, Mutex}};
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

pub struct GraphicsChip {
    pub render_passes: Vec<RenderPass>
}

pub enum GraphicsChipError {
    ProgramCreation,
    ShaderParsing,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PrimitiveType {
    triangles,
    points,
    lines,
}

impl From<PrimitiveType> for glium::index::PrimitiveType {
    fn from(p: PrimitiveType) -> Self {
        if p == PrimitiveType::triangles { return glium::index::PrimitiveType::TrianglesList; }
        else if p == PrimitiveType::lines { return glium::index::PrimitiveType::LinesList; }
        else { return glium::index::PrimitiveType::Points; }
    }
}

impl GraphicsChip {
    pub fn new() -> Self {
        let render_passes = (Vec::new());
        Self { render_passes }
    }

    pub fn begin(&mut self, primitive_type: PrimitiveType) {
        let render_pass = RenderPass::new(
            Vec::new(), 
            Vertex::default(), 
            primitive_type
        );

        self.render_passes.push(render_pass);
    }

    pub fn end(&mut self) {
        match self.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_vertex_state = Vertex::default();
                //render_pass.vertex_buffer.clear();
            },
            None => return
        };
    }

    pub fn vertex(&mut self, coords: Vec3) {
        match self.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_vertex_state.position = coords.to_array();
                render_pass.vertex_buffer.push(render_pass.current_vertex_state);
            },
            None => return
        };
    }

    pub fn normal(&mut self, coords: Vec3) {
        match self.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_vertex_state.normal = coords.to_array();
            },
            None => return
        };
    }

    pub fn tex_coord(&mut self, coords: Vec2) {
        match self.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_vertex_state.uv = coords.to_array();
            },
            None => return
        };
    }

    pub fn color(&mut self, coords: Vec4) {
        match self.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_vertex_state.color = coords.to_array();
            },
            None => return
        };
    }
}

// impl UserData for GraphicsChip {
//     fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
//         methods.add_method_mut("endObject", |_, this, _: ()| {
//             Ok(this.end())
//         });
//     }
// }