use crate::{
    vertex::Vertex, 
    render_pass::RenderPass, 
    image::{Image, ImageRef, ImageId}, 
    assets::Assets, 
    scene::SceneId, 
    prelude::GlobalShaders, 
    render_pipeline::RenderPipeline, 
    uniforms::Uniforms, 
    gltf_loader::{GltfError, GltfLoader}, 
    node::Node
};

use image::ImageError;
use verdi_math::prelude::*;

pub struct GraphicsChip {
    pub pipeline: RenderPipeline,
    pub assets: Assets,
    pub globals: GlobalShaders,
    pub uniforms: Uniforms
}

#[derive(Copy, Clone, PartialEq)]
pub enum PrimitiveType {
    Triangles,
    Points,
    Lines,
}

impl From<String> for PrimitiveType {
    fn from(string: String) -> Self {
        if string == "triangles" { PrimitiveType::Triangles }
        else if string == "points" { PrimitiveType::Points }
        else if string == "lines" { PrimitiveType::Lines }
        else { PrimitiveType::Triangles }
    }
}

impl From<PrimitiveType> for glium::index::PrimitiveType {
    fn from(p: PrimitiveType) -> Self {
        if p == PrimitiveType::Triangles { return glium::index::PrimitiveType::TrianglesList; }
        else if p == PrimitiveType::Lines { return glium::index::PrimitiveType::LinesList; }
        else { return glium::index::PrimitiveType::Points; }
    }
}

impl GraphicsChip {
    pub fn new() -> Result<Self, std::io::Error> {
        let mut assets = Assets::new();
        let mut uniforms = Uniforms::default();
        let pipeline = RenderPipeline::new(&mut uniforms);
        let globals = GlobalShaders::new(&mut assets, &pipeline)?;

        Ok(Self { 
            pipeline,
            assets,
            globals,
            uniforms,
        })
    }

    pub fn begin(&mut self, primitive_type: PrimitiveType) {
        // let render_pass = RenderPass::new(
        //     None,
        //     primitive_type
        // );

        // self.render_passes.push(render_pass);
    }

    pub fn end(&mut self) {
        match self.pipeline.render_passes.last_mut() {
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
        match self.pipeline.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_vertex_state.normal = coords.to_array();
            },
            None => return
        };
    }

    pub fn tex_coord(&mut self, coords: Vec2) {
        match self.pipeline.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_vertex_state.uv = coords.to_array();
            },
            None => return
        };
    }

    pub fn color(&mut self, coords: Vec4) {
        match self.pipeline.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_vertex_state.color = coords.to_array();
            },
            None => return
        };
    }

    pub fn new_image(&mut self, path: &String) -> Result<ImageId, ImageError> {
        let image = Image::new(path)?;

        Ok(self.assets.add_texture(image))
    }

    pub fn bind_texture(&mut self, image: ImageRef) {
        match self.pipeline.render_passes.last_mut() {
            Some(render_pass) => {
                render_pass.current_texture = Some(image);
            },
            None => return
        };
    }

    pub fn draw(&mut self, scene_id: SceneId) {
        let scene = self.assets.get_scene(scene_id).unwrap();
        for node in scene.nodes.iter() {
            if node.mesh.is_none() {
                continue;
            }
    
            let render_pass = RenderPass::new(
                node.clone(),
                PrimitiveType::Triangles
            );
    
            self.pipeline.render_passes.push(render_pass);
        }
    }

    pub fn draw_node(&mut self, node: &Node) {
        if node.mesh.is_none() {
            return;
        }

        let render_pass = RenderPass::new(
            node.clone(),
            PrimitiveType::Triangles
        );

        self.pipeline.render_passes.push(render_pass);
    }

    pub fn new_scene(&mut self, path: &String) -> Result<SceneId, GltfError>{
        let scene = GltfLoader::load(path, self)?;

        Ok(self.assets.add_scene(scene))
    }

    pub fn set_clear_color(&mut self, color: Vec4) {
        self.pipeline.clear_color = color;
    }

    pub fn translate(&mut self, v: Vec3) {
        *self.uniforms
            .get_mat4_mut(
                self.pipeline.view_matrix
            ).unwrap() 
                *= Mat4::from_translation(v);
    }

    pub fn rotate(&mut self, angle: f32, axis: Vec3) {
        *self.uniforms
            .get_mat4_mut(
                self.pipeline.view_matrix
            ).unwrap() 
                *= Mat4::from_axis_angle(axis, angle);
    }

    pub fn set_fog_start(&mut self, distance: f32) {
        *self.uniforms
            .get_float_mut(
                self.pipeline.fog_start
            ).unwrap() 
                = distance;
    }

    pub fn set_fog_end(&mut self, distance: f32) {
        *self.uniforms
            .get_float_mut(
                self.pipeline.fog_end
            ).unwrap() 
                = distance;
    }
}