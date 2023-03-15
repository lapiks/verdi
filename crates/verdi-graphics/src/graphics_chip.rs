use std::{cell::RefCell, rc::Rc};

use crate::{
    vertex::Vertex, 
    render_pass::RenderPass, 
    image::{Image, ImageHandle, ImageId}, 
    model::ModelId, 
    uniforms::UniformId, 
    gltf_loader::{GltfError, GltfLoader}, 
    node::Node, 
    material::{Material, MaterialId}, 
    globals::Globals, 
    mesh::{MeshId, Mesh, PrimitiveType}, 
    prelude::Database, 
    render_state::RenderState, 
    pass::PassHandle, 
    render_graph::RenderGraph, 
    camera::{CameraId, Camera},
};

use image::ImageError;
use verdi_math::prelude::*;

/// High level access to rendering features.
pub struct GraphicsChip {
    pub render_graph: Rc<RefCell<RenderGraph>>,
    pub render_passes: Vec<RenderPass>,
    pub stream_buffer: StreamBufferState,
    pub database: Rc<RefCell<Database>>,
    pub globals: Rc<Globals>,
    pub render_state: RenderState,
    pub camera: CameraId,
}

// Public API
impl GraphicsChip {
    pub fn new(database: Rc<RefCell<Database>>, globals: Rc<Globals>) -> Result<Self, std::io::Error> {
        let mat_2d = database.borrow_mut().assets.add_material(
            Material::new(globals.global_shaders.gouraud, &globals.global_uniforms)
                .clone()
        );

        let streaming_mesh = database.borrow_mut().assets.add_mesh(
            Mesh::new(
                vec![Vertex::default(); 1024 * 1024],
                None,
                //PrimitiveType::Triangles,
                PrimitiveType::Lines,
                mat_2d,
            )
        );

        let stream_buffer = StreamBufferState {
            mesh_id: streaming_mesh,
            vertex_count: 0,
            current_offset: 0,
        };

        // default camera
        let camera = Camera::new();
        let camera_id = database.borrow_mut().assets.add_camera(camera);

        Ok(Self { 
            render_graph: Rc::new(RefCell::new(RenderGraph::new())),
            render_passes: Vec::new(),
            stream_buffer,
            database,
            globals,
            render_state: RenderState::new(),
            camera: camera_id,
        })
    }

    pub fn on_game_start(&mut self) {

    }

    pub fn on_game_shutdown(&mut self) {
        self.database.borrow_mut().assets.clear();
        self.render_passes.clear();
        self.database.borrow_mut().uniforms.clear();
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
        self.render_graph.borrow_mut().clear();
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

        Ok(self.database.borrow_mut().assets.add_texture(image))
    }

    pub fn bind_texture(&mut self, image: ImageHandle) {
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_texture = Some(image);
        //     },
        //     None => return
        // };
    }

    pub fn draw_model(&mut self, model_id: ModelId) {
        let db = self.database.borrow();
        let model = db.assets.get_model(model_id).unwrap();
        for node in model.nodes.iter() {
            // nothing to draw
            if node.mesh.is_none() {
                continue;
            }

            if let Some(mesh) = db.assets.get_mesh(node.mesh.unwrap()) {
                self.render_passes.push(
                    RenderPass { 
                        mesh_id: mesh.id,
                        transform: node.transform.clone(),
                    }
                );
            }
        }
    }

    pub fn draw_mesh(&mut self, mesh_id: MeshId) {
        if let Some(mesh) = self.database.borrow().assets.get_mesh(mesh_id) {
            self.render_passes.push(
                RenderPass { 
                    mesh_id: mesh.id,
                    transform: Transform::IDENTITY,
                }
            );
        }
    }

    pub fn draw_node(&mut self, node: &Node) {
        // nothing to draw
        if node.mesh.is_none() {
            return;
        }

        if let Some(mesh) = self.database.borrow().assets.get_mesh(node.mesh.unwrap()) {
            self.render_passes.push(
                RenderPass { 
                    mesh_id: mesh.id,
                    transform: node.transform.clone(),
                }
            );
        }
    }

    pub fn new_model(&mut self, path: &String) -> Result<ModelId, GltfError> {
        let model = GltfLoader::load(path, &mut self.database.borrow_mut(), &self.globals)?;

        Ok(self.database.borrow_mut().assets.add_model(model))
    }
    
    pub fn new_mesh(&mut self) -> Result<MeshId, GltfError> {
        let vertex_buffer:Vec<Vertex> = Vec::new();
        let index_buffer = None;

        let material_id = self.new_material();

        Ok(self.database.borrow_mut().assets.add_mesh(
            Mesh::new(
                vertex_buffer,
                index_buffer,
                PrimitiveType::Triangles,
                material_id
            )
        ))
    }

    pub fn new_material(&mut self) -> MaterialId {
        let mut material = Material::new(self.globals.global_shaders.gouraud, &self.globals.global_uniforms);
        material.add_uniform("u_fog_start", self.globals.global_uniforms.fog_start);
        material.add_uniform("u_fog_end", self.globals.global_uniforms.fog_end);
        material.add_uniform("u_enable_lighting", self.globals.global_uniforms.enable_lighting);

        self.database.borrow_mut().assets.add_material(
            material
        )
    }

    pub fn new_uniform_float(&mut self, value: f32) -> UniformId {
        self.database.borrow_mut().uniforms.add_float(value)
    }

    pub fn new_uniform_vec2(&mut self, value: Vec2) -> UniformId {
        self.database.borrow_mut().uniforms.add_vec2(value)
    }

    pub fn new_pass(&mut self) -> PassHandle {
        PassHandle {
            graph: self.render_graph.clone(),
            id: self.render_graph.borrow_mut().create_pass(),
        }
    }

    pub fn set_clear_color(&mut self, color: &Vec4) {
        self.render_state.clear_color = *color;
    }

    pub fn translate(&mut self, v: &Vec3) {
        *self.database.borrow_mut().uniforms
            .get_mat4_mut(
                self.globals.global_uniforms.view_matrix
            ).unwrap() 
                *= Mat4::from_translation(*v);
    }

    pub fn rotate(&mut self, angle: f32, axis: &Vec3) {
        *self.database.borrow_mut().uniforms
            .get_mat4_mut(
                self.globals.global_uniforms.view_matrix
            ).unwrap() 
                *= Mat4::from_axis_angle(*axis, angle);
    }

    pub fn draw_line(&mut self, p1: &Vec2, p2: &Vec2) {
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

        let mut vertices = [v1, v2];
        let cmd = DrawCommand {
            primitive_type: PrimitiveType::Lines,
            vertex_count: 2,
            data: &mut vertices,
        };

        self.request_flush(&cmd);
    }
}

pub struct StreamBufferState {
    pub mesh_id: MeshId, 
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

pub struct DrawCommand<'a> {
    pub primitive_type: PrimitiveType,
    pub vertex_count: u32,
    pub data: &'a mut [Vertex],
    // material_id
}

// Private impl
impl GraphicsChip {
    fn request_flush(&mut self, cmd: &DrawCommand) {
        let mut db = self.database.borrow_mut();
        let mesh = db.assets
            .get_mesh_mut(self.stream_buffer.mesh_id)
            .expect("Primitive not found");

        if cmd.primitive_type != mesh.primitive_type {
            //self.flush_stream_buffer();
            self.render_passes.push(
                RenderPass {
                    mesh_id: self.stream_buffer.mesh_id,
                    transform: Transform::IDENTITY,
                }
            );

            mesh.primitive_type = cmd.primitive_type;
            self.stream_buffer.vertex_count = cmd.vertex_count;
            self.stream_buffer.current_offset = 0;
        }
        else {
            self.stream_buffer.current_offset = self.stream_buffer.vertex_count as usize;
            self.stream_buffer.vertex_count += cmd.vertex_count;
        }

        let new_offset = self.stream_buffer.current_offset + cmd.vertex_count as usize;
        let stream_buffer = StreamBuffer {
            data: &mut mesh.vertex_buffer[
                self.stream_buffer.current_offset
                ..
                new_offset
            ],
        };

        self.stream_buffer.current_offset = new_offset;

        stream_buffer.data.clone_from_slice(&cmd.data);
    }

    pub fn flush_stream_buffer(&mut self) {
        self.render_passes.push(
            RenderPass {
                mesh_id: self.stream_buffer.mesh_id,
                transform: Transform::IDENTITY,
            }
        )
    }
}