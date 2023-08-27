use std::{cell::RefCell, rc::Rc};

use crate::{
    vertex::Vertex, 
    render_pass::RenderPass, 
    image::{Image, ImageHandle, ImageId}, 
    model::ModelId, 
    gltf_loader::{GltfError, GltfLoader}, 
    material::{Material, MaterialId}, 
    globals::Globals, 
    mesh::{MeshId, Mesh, PrimitiveType, MeshHandle}, 
    render_state::RenderState, 
    pass::PassHandle, 
    render_graph::RenderGraph, 
    camera::{Camera, CameraHandle}, 
    sprite::{SpriteId, Sprite}, 
    uniform::{Uniform, UniformType, UniformId}, 
    gpu_assets::{GpuAssets, PrepareAsset}, 
    program::Program,
};

use glium::Display;
use image::ImageError;
use verdi_database::Assets;
use verdi_math::prelude::*;

/// High level access to rendering features.
pub struct GraphicsChip {
    pub render_graph: Rc<RefCell<RenderGraph>>,
    pub render_passes: Vec<RenderPass>,
    pub stream_buffer: StreamBufferState,
    pub assets: Assets,
    pub gpu_assets: Rc<RefCell<GpuAssets>>,
    pub globals: Rc<Globals>,
    pub render_state: RenderState,
    math: Rc<RefCell<Math>>, 
}

// Public API
impl GraphicsChip {
    pub fn new(math: Rc<RefCell<Math>>) -> Result<Self, std::io::Error> {
        let mut assets = Assets::new();

        let globals = Rc::new(
            Globals::new(&mut assets).expect("Globals creation failed")
        );

        let mat_2d = assets
            .add(
                Box::new(
                    Material::new(globals.global_shaders.gouraud, &globals.global_uniforms)
                        .clone()
                )
        );

        let streaming_mesh_id = assets
            .add(
                Box::new(
                    Mesh::new(
                        vec![Vertex::default(); 1024 * 1024],
                        None,
                        //PrimitiveType::Triangles,
                        PrimitiveType::Lines,
                        mat_2d,
                    )
            )
        );

        let streaming_mesh = MeshHandle::new(assets.clone(), streaming_mesh_id);

        let stream_buffer = StreamBufferState {
            mesh: streaming_mesh,
            vertex_count: 0,
            current_offset: 0,
        };

        Ok(Self { 
            render_graph: Rc::new(RefCell::new(RenderGraph::new())),
            render_passes: Vec::new(),
            stream_buffer,
            assets,
            gpu_assets: Rc::new(RefCell::new(GpuAssets::new())),
            globals,
            render_state: RenderState::new(),
            math,
        })
    }

    pub fn on_game_start(&mut self) {

    }

    pub fn on_game_shutdown(&mut self) {
        self.assets.clear();
        self.gpu_assets.borrow_mut().clear();
        self.render_passes.clear();
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

    pub fn prepare_gpu_assets(&mut self, display: &Display) {
        // à rendre générique

        let asset_datas = self.assets.get_datas();

        for pass in self.render_graph.borrow().get_passes().iter() {
            for cmd in pass.get_cmds() {
                let mesh = asset_datas
                    .get::<Mesh>(cmd.mesh.get_id())
                    .expect("Missing primitive resource");

                // construct gpu primitive
                match mesh.prepare_rendering(display, &self.assets) {
                    Ok(gpu_mesh) => self.gpu_assets.borrow_mut().add(cmd.mesh.get_id(), gpu_mesh),
                    Err(_) => todo!(),
                }

                // construct gpu objects needed by the material
                if let Some(material) = self.assets.get_datas().get::<Material>(mesh.material) {
                    for texture_id in material.get_textures() {
                        if let Some(texture) = asset_datas.get::<Image>(*texture_id) {
                            match texture.prepare_rendering(display, &self.assets)  {
                                Ok(gpu_texture) => self.gpu_assets.borrow_mut().add(*texture_id, gpu_texture),
                                Err(_) => todo!(),
                            }
                        }
                    }
                }     
            }     
        }
        
        // construct gpu programs
        // Pas fou !
        if self.gpu_assets.borrow().get::<Program>(self.globals.global_shaders.gouraud).is_none() {
            if let Some(program) = self.assets.get_datas().get::<Program>(self.globals.global_shaders.gouraud) {
                match program.prepare_rendering(display, &self.assets)  {
                    Ok(gpu_program) => self.gpu_assets.borrow_mut().add(self.globals.global_shaders.gouraud, gpu_program),
                    Err(_) => todo!(),
                }
            }   
        }

        if self.gpu_assets.borrow().get::<Program>(self.globals.global_shaders.gouraud_textured).is_none() {
            if let Some(program) = self.assets.get_datas().get::<Program>(self.globals.global_shaders.gouraud_textured) {
                match program.prepare_rendering(display, &self.assets)  {
                    Ok(gpu_program) => self.gpu_assets.borrow_mut().add(self.globals.global_shaders.gouraud_textured, gpu_program),
                    Err(_) => todo!(),
                }
            }   
        }

        if self.gpu_assets.borrow().get::<Program>(self.globals.global_shaders.std_2d).is_none() {
            if let Some(program) = self.assets.get_datas().get::<Program>(self.globals.global_shaders.std_2d) {
                match program.prepare_rendering(display, &self.assets)  {
                    Ok(gpu_program) => self.gpu_assets.borrow_mut().add(self.globals.global_shaders.std_2d, gpu_program),
                    Err(_) => todo!(),
                }
            }   
        }
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

        Ok(self.assets.add(Box::new(image)))
    }

    pub fn bind_texture(&mut self, image: ImageHandle) {
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_texture = Some(image);
        //     },
        //     None => return
        // };
    }

    // pub fn draw_model(&mut self, model_id: ModelId) {
    //     let db = self.assets.borrow();
    //     let model = db.get::<Model>(model_id).unwrap();
    //     for node in model.nodes.iter() {
    //         // nothing to draw
    //         if node.mesh.is_none() {
    //             continue;
    //         }

    //         if let Some(mesh) = db.get::<Mesh>(node.mesh.unwrap().get_id()) {
    //             self.render_passes.push(
    //                 RenderPass { 
    //                     mesh_id: mesh.id,
    //                     transform: node.transform,
    //                 }
    //             );
    //         }
    //     }
    // }

    // pub fn draw_mesh(&mut self, mesh_id: MeshId, transform_id: TransformId) {
    //     if let Some(mesh) = self.assets.borrow().get::<Mesh>(mesh_id) {
    //         self.render_passes.push(
    //             RenderPass { 
    //                 mesh_id: mesh.id,
    //                 transform: transform_id,
    //             }
    //         );
    //     }
    // }

    // pub fn draw_node(&mut self, node: &Node) {
    //     // nothing to draw
    //     if node.mesh.is_none() {
    //         return;
    //     }

    //     if let Some(mesh) = self.assets.borrow().get::<Mesh>(node.mesh.unwrap().get_id()) {
    //         self.render_passes.push(
    //             RenderPass { 
    //                 mesh_id: node.mesh.unwrap(),
    //                 transform: node.transform.clone(),
    //             }
    //         );
    //     }
    // }

    pub fn new_model(&mut self, path: &String) -> Result<ModelId, GltfError> {
        let model = GltfLoader::load(path, &mut self.assets, self.math.clone(), &self.globals)?;

        Ok(self.assets.add(Box::new(model)))
    }
    
    pub fn new_mesh(&mut self) -> Result<MeshId, GltfError> {
        let vertex_buffer:Vec<Vertex> = Vec::new();
        let index_buffer = None;

        let material_id = self.new_gouraud_material();

        Ok(self.assets.add(
            Box::new(
                Mesh::new(
                    vertex_buffer,
                    index_buffer,
                    PrimitiveType::Triangles,
                    material_id
                )
            )
        ))
    }

    // pub fn new_sprite(&mut self, image: ImageHandle) -> SpriteId {
    //     let mut dimensions = (100, 100);
    //     if let Some(image_ref) = self.assets.borrow().get::<Image>(image.id) {
    //         dimensions = image_ref.get_dimensions();
    //     }
        
    //     let vertex_buffer:Vec<Vertex> = vec![
    //         Vertex {
    //             position: [0.0, 0.0, 0.0],
    //             normal: [0.0, 0.0, 0.0],
    //             uv: [0.0, 0.0],
    //             color: [0.0, 0.0, 0.0, 0.0],
    //         },
    //         Vertex {
    //             position: [0.0, dimensions.1 as f32, 0.0],
    //             normal: [0.0, 0.0, 0.0],
    //             uv: [0.0, 1.0],
    //             color: [0.0, 0.0, 0.0, 0.0],
    //         },
    //         Vertex {
    //             position: [dimensions.0 as f32, 0.0, 0.0],
    //             normal: [0.0, 0.0, 0.0],
    //             uv: [1.0, 0.0],
    //             color: [0.0, 0.0, 0.0, 0.0],
    //         },
    //         Vertex {
    //             position: [dimensions.0 as f32, dimensions.1 as f32, 0.0],
    //             normal: [0.0, 0.0, 0.0],
    //             uv: [1.0, 1.0],
    //             color: [0.0, 0.0, 0.0, 0.0],
    //         },
    //     ];
    //     let index_buffer = vec![0, 1, 2, 2, 1, 3];

    //     let mut material = Material::new(
    //         self.globals.global_shaders.std_2d, 
    //         &self.globals.global_uniforms
    //     );

    //     let tex_uniform_id = self.assets.borrow_mut().add(
    //         Box::new(
    //             Uniform::new(image)
    //         )
    //     );
    //     material.add_uniform("u_texture", tex_uniform_id);

    //     let material_id = self.assets.borrow_mut().add(
    //         Box::new(material)
    //     );

    //     // TODO: reuse same mesh
    //     let quad = self.assets.borrow_mut().add(
    //         Box::new(
    //             Mesh::new(
    //                 vertex_buffer,
    //                 Some(index_buffer),
    //                 PrimitiveType::Triangles,
    //                 material_id
    //             )
    //         )
    //     );
    //     let sprite = Sprite::new(image.id, quad);

    //     self.assets.borrow_mut().add(Box::new(sprite))
    // }

    pub fn new_gouraud_material(&mut self) -> MaterialId {
        let mut material = Material::new(
            self.globals.global_shaders.gouraud, 
            &self.globals.global_uniforms
        );
        material.add_uniform("u_fog_start".to_string(), self.globals.global_uniforms.fog_start.clone());
        material.add_uniform("u_fog_end".to_string(), self.globals.global_uniforms.fog_end.clone());
        material.add_uniform("u_enable_lighting".to_string(), self.globals.global_uniforms.enable_lighting.clone());

        self.assets.add(
            Box::new(material)
        )
    }

    pub fn new_2d_material(&mut self) -> MaterialId {
        let material = Material::new(
            self.globals.global_shaders.std_2d, 
            &self.globals.global_uniforms
        );
        self.assets.add(
            Box::new(material)
        )
    }

    pub fn new_uniform<T: UniformType>(&mut self, value: T) -> UniformId {
        self.assets.add(
            Box::new(
                Uniform::new(value)
            )
        )
    }

    pub fn new_camera(&mut self, transform: TransformHandle) -> CameraHandle {
        let camera_id = self.assets.add(
            Box::new(
                Camera::new(transform)
            )
        );
        CameraHandle::new(
            self.assets.clone(),
            camera_id
        )
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

    // pub fn translate(&mut self, v: &Vec3) {
    //     *self.assets.borrow_mut().uniforms
    //         .get_mat4_mut(
    //             self.globals.global_uniforms.view_matrix
    //         ).unwrap() 
    //             *= Mat4::from_translation(*v);
    // }

    // pub fn rotate(&mut self, angle: f32, axis: &Vec3) {
    //     *self.assets.borrow_mut().uniforms
    //         .get_mat4_mut(
    //             self.globals.global_uniforms.view_matrix
    //         ).unwrap() 
    //             *= Mat4::from_axis_angle(*axis, angle);
    // }

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
    pub mesh: MeshHandle, 
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
        let mut asset_datas = self.assets.get_datas_mut();
        let mesh = asset_datas
            .get_mut::<Mesh>(self.stream_buffer.mesh.get_id())
            .expect("Primitive not found");

        if cmd.primitive_type != mesh.primitive_type {
            //self.flush_stream_buffer();
            self.render_passes.push(
                RenderPass {
                    mesh: self.stream_buffer.mesh.clone(),
                    transform_matrix: self.globals.global_uniforms.identity_mat.clone(),
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
                mesh: self.stream_buffer.mesh.clone(),
                transform_matrix: self.globals.global_uniforms.identity_mat.clone(),
            }
        )
    }
}