use crate::{
    vertex::Vertex, 
    render_pass::RenderPass, 
    image::{Image, ImageRef, ImageId}, 
    assets::Assets, 
    scene::SceneId, 
    uniforms::Uniforms, 
    gltf_loader::{GltfError, GltfLoader}, 
    node::Node, 
    primitive::{Primitive, PrimitiveId}, 
    material::Material, 
    globals::Globals, transform::Transform
};

use image::ImageError;
use verdi_math::prelude::*;

pub struct GraphicsChip {
    pub render_passes: Vec<RenderPass>,
    pub stream_buffer: StreamBufferState,
    pub assets: Assets,
    pub uniforms: Uniforms,
    pub globals: Globals,
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

// Public API
impl GraphicsChip {
    pub fn new() -> Result<Self, std::io::Error> {
        let mut assets = Assets::new();
        let mut uniforms = Uniforms::default();

        // create globals
        let globals = Globals::new(
            &mut assets, 
            &mut uniforms,
        )?;

        let mat_2d = assets.add_material(
            *Material::new(globals.global_shaders.gouraud)
                .add_uniform("u_model", globals.global_uniforms.model_matrix)
                .add_uniform("u_view", globals.global_uniforms.view_matrix)
                .add_uniform("u_projection", globals.global_uniforms.perspective_matrix)
                .add_uniform("u_resolution", globals.global_uniforms.resolution)
        );

        let streaming_primitive = assets.add_primitive(
            Primitive::new(
                vec![Vertex::default(); 1024 * 1024],
                None,
                //PrimitiveType::Triangles,
                PrimitiveType::Lines,
                mat_2d,
            )
        );

        let stream_buffer = StreamBufferState {
            primitive_id: streaming_primitive,
            vertex_count: 0,
            current_offset: 0,
        };

        Ok(Self { 
            render_passes: Vec::new(),
            stream_buffer,
            assets,
            uniforms,
            globals,
        })
    }

    pub fn on_game_start(&mut self) {

    }

    pub fn on_game_shutdown(&mut self) {
        self.assets.clear();
        self.render_passes.clear();
        self.uniforms.clear();
    }

    pub fn new_frame(&mut self) {
        // if self.buffer_state.vertex_count > 0 {
        //     let render_pass = RenderPass {
        //         primitive_id: self.buffer_state.primitive_id,
        //         transform: Transform::default(),
        //     };
    
        //     self.render_passes.push(render_pass);
        // }   
    }

    pub fn frame_ends(&mut self) {
        self.render_passes.clear();   
        //self.buffer_state.next_frame();
    }

    pub fn begin(&mut self, primitive_type: PrimitiveType) {
        // let handle = PrimitiveHandle {
        //     id: self.buffer_state.primitive_id,
        //     gpu: Arc::new(Mutex::new(self)),
        // };

        // let cmd = DrawCommand {
        //     primitive_type: primitive_type,
        //     vertex_count: 0,
        // };

        // self.request_flush(&cmd);

        // let render_pass = RenderPass {
        //     None,
        //     primitive_type
        // };

        // self.render_passes.push(render_pass);
    }

    pub fn end(&mut self) {
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_vertex_state = Vertex::default();
        //         //render_pass.vertex_buffer.clear();
        //     },
        //     None => return
        // };
    }

    pub fn vertex(&mut self, coords: &Vec3) {
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_vertex_state.position = coords.to_array();
        //         render_pass.vertex_buffer.push(render_pass.current_vertex_state);
        //     },
        //     None => return
        // };

        // let cmd = DrawCommand {
        //     primitive_type: PrimitiveType::Lines,
        //     vertex_count: 2,
        // };

        // let vertex = Vertex {
        //     position: coords.to_array(),
        //     uv: [0.0, 0.0],
        //     normal: [0.0, 0.0, 0.0],
        //     color: [1.0, 0.0, 0.0, 1.0],
        // };

        // let stream_buffer = self.request_flush(&cmd);
        // stream_buffer.data.clone_from_slice(&vertices)
    }

    pub fn normal(&mut self, coords: &Vec3) {
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_vertex_state.normal = coords.to_array();
        //     },
        //     None => return
        // };
    }

    pub fn tex_coord(&mut self, coords: &Vec2) {
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_vertex_state.uv = coords.to_array();
        //     },
        //     None => return
        // };
    }

    pub fn color(&mut self, color: &Vec4) {
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_vertex_state.color = color.to_array();
        //     },
        //     None => return
        // };
    }

    pub fn new_image(&mut self, path: &String) -> Result<ImageId, ImageError> {
        let image = Image::new(path)?;

        Ok(self.assets.add_texture(image))
    }

    pub fn bind_texture(&mut self, image: ImageRef) {
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_texture = Some(image);
        //     },
        //     None => return
        // };
    }

    pub fn draw_scene(&mut self, scene_id: SceneId) {
        let scene = self.assets.get_scene(scene_id).unwrap();
        for node in scene.nodes.iter() {
            // nothing to draw
            if node.mesh.is_none() {
                continue;
            }

            if let Some(mesh) = self.assets.get_mesh(node.mesh.unwrap()) {
                for primitive_id in mesh.primitives.iter() {
                    self.render_passes.push(
                        RenderPass { 
                            primitive_id: *primitive_id,
                            transform: node.transform.clone(),
                        }
                    );
                }
            }
        }
    }

    pub fn draw_node(&mut self, node: &Node) {
        // nothing to draw
        if node.mesh.is_none() {
            return;
        }

        if let Some(mesh) = self.assets.get_mesh(node.mesh.unwrap()) {
            for primitive_id in mesh.primitives.iter() {
                self.render_passes.push(
                    RenderPass { 
                        primitive_id: *primitive_id,
                        transform: node.transform.clone(),
                    }
                );
            }
        }
    }

    pub fn new_scene(&mut self, path: &String) -> Result<SceneId, GltfError>{
        let scene = GltfLoader::load(path, self)?;

        Ok(self.assets.add_scene(scene))
    }

    pub fn set_clear_color(&mut self, color: &Vec4) {
        self.globals.clear_color = *color;
    }

    pub fn translate(&mut self, v: &Vec3) {
        *self.uniforms
            .get_mat4_mut(
                self.globals.global_uniforms.view_matrix
            ).unwrap() 
                *= Mat4::from_translation(*v);
    }

    pub fn rotate(&mut self, angle: f32, axis: &Vec3) {
        *self.uniforms
            .get_mat4_mut(
                self.globals.global_uniforms.view_matrix
            ).unwrap() 
                *= Mat4::from_axis_angle(*axis, angle);
    }

    pub fn set_fog_start(&mut self, distance: f32) {
        *self.uniforms
            .get_float_mut(
                self.globals.global_uniforms.fog_start
            ).unwrap() 
                = distance;
    }

    pub fn set_fog_end(&mut self, distance: f32) {
        *self.uniforms
            .get_float_mut(
                self.globals.global_uniforms.fog_end
            ).unwrap() 
                = distance;
    }

    pub fn draw_line(&mut self, p1: &Vec2, p2: &Vec2) {
        let cmd = DrawCommand {
            primitive_type: PrimitiveType::Lines,
            vertex_count: 2,
        };

        let v1 = Vertex {
            position: [p1.x, p1.y, 0.0],
            uv: [0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        };

        let v2 = Vertex {
            position: [p2.x, p2.y, 0.0],
            uv: [0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        };

        let vertices = [v1, v2];

        let stream_buffer = self.request_flush(&cmd);
        stream_buffer.data.clone_from_slice(&vertices)
    }
}

pub struct StreamBufferState {
    pub primitive_id: PrimitiveId, 
    pub vertex_count: u32,
    pub current_offset: usize,
}

impl StreamBufferState {
    pub fn next_frame(&mut self) {
        self.current_offset = 0;
        self.vertex_count = 0;
    }
}

pub struct StreamBuffer<'a> {
    pub data: &'a mut [Vertex],
}

pub struct DrawCommand {
    pub primitive_type: PrimitiveType,
    pub vertex_count: u32,
    // material_id
}

// Private impl
impl GraphicsChip {
    fn request_flush(&mut self, cmd: &DrawCommand) -> StreamBuffer {
        let primitive = self.assets
            .get_primitive_mut(self.stream_buffer.primitive_id)
            .expect("Primitive not found");

        if cmd.primitive_type != primitive.primitive_type {
            //self.flush_stream_buffer();
            self.render_passes.push(
                RenderPass {
                    primitive_id: self.stream_buffer.primitive_id,
                    transform: Transform::identity(),
                }
            );

            primitive.primitive_type = cmd.primitive_type;
            self.stream_buffer.vertex_count = cmd.vertex_count;
            self.stream_buffer.current_offset = 0;
        }
        else {
            self.stream_buffer.current_offset = self.stream_buffer.vertex_count as usize;
            self.stream_buffer.vertex_count += cmd.vertex_count;
        }

        let new_offset = self.stream_buffer.current_offset + cmd.vertex_count as usize;
        let stream_buffer = StreamBuffer {
            data: &mut primitive.vertex_buffer[
                self.stream_buffer.current_offset
                ..
                new_offset
            ],
        };

        self.stream_buffer.current_offset = new_offset;

        stream_buffer
    }

    pub fn flush_stream_buffer(&mut self) {
        self.render_passes.push(
            RenderPass {
                primitive_id: self.stream_buffer.primitive_id,
                transform: Transform::identity(),
            }
        )
    }
}