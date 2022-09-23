use crate::{vertex::Vertex, render_pass::RenderPass, image::{Image, ImageRef}, assets::Assets, scene::Scene};
use image::ImageError;
use verdi_math::prelude::*;

pub struct GraphicsChip {
    pub render_passes: Vec<RenderPass>,
    pub assets: Assets,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PrimitiveType {
    Triangles,
    Points,
    Lines,
}

impl From<PrimitiveType> for glium::index::PrimitiveType {
    fn from(p: PrimitiveType) -> Self {
        if p == PrimitiveType::Triangles { return glium::index::PrimitiveType::TrianglesList; }
        else if p == PrimitiveType::Lines { return glium::index::PrimitiveType::LinesList; }
        else { return glium::index::PrimitiveType::Points; }
    }
}

impl GraphicsChip {
    pub fn new() -> Self {
        Self { 
            render_passes: Vec::new(),
            assets: Assets::new(),
        }
    }

    pub fn begin(&mut self, primitive_type: PrimitiveType) {
        // let render_pass = RenderPass::new(
        //     None,
        //     primitive_type
        // );

        // self.render_passes.push(render_pass);
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
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_vertex_state.position = coords.to_array();
        //         render_pass.vertex_buffer.push(render_pass.current_vertex_state);
        //     },
        //     None => return
        // };
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

    pub fn new_image(&mut self, path: &String) -> Result<ImageRef, ImageError> {
        let image = Image::new(path)?;

        Ok(self.assets.add_texture(image))
    }

    pub fn bind_texture(&mut self, image: ImageRef) {
        match self.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_texture = Some(image);
            },
            None => return
        };
    }

    pub fn new_scene(&mut self, path: &String) -> Result<Scene, gltf::Error> {
        let mut scene = Scene::new();
        scene.load(path, &mut self.assets)?;

        Ok(scene)
    }

    pub fn draw(&mut self, scene: &Scene) {
        for mesh_ref in scene.meshes.iter() {
            let render_pass = RenderPass::new(
                *mesh_ref,
                PrimitiveType::Triangles
            );

            self.render_passes.push(render_pass);
        }
    }
}