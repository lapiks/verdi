use std::{cell::RefCell, rc::Rc};

use crate::{
    vertex::Vertex, 
    render_pass::RenderPass, 
    image::{Image, ImageHandle}, 
    model::ModelHandle, 
    gltf_loader::{GltfError, GltfLoader}, 
    material::{Material, MaterialHandle}, 
    globals::Globals, 
    mesh::{Mesh, PrimitiveType, MeshHandle}, 
    render_state::RenderState, 
    pass::PassHandle, 
    render_graph::RenderGraph, 
    camera::{Camera, CameraHandle},
    gpu_assets::{GpuAssets, PrepareAsset}, 
    framebuffer::{FramebufferHandle, Framebuffer}, 
    depth_buffer::{DepthBufferHandle, DepthBuffer, GpuDepthBuffer}, 
    gpu_program::GpuProgram, 
    pipeline::Pipeline, 
    program::Program, 
    gpu_image::GpuImage, gpu_mesh::GpuMesh, uniform::{Uniform, UniformValue},
};

use glium::Display;
use image::ImageError;
use verdi_database::Assets;
use verdi_math::prelude::*;

/// High level access to rendering features.
pub struct GraphicsChip {
    pub render_graph: Rc<RefCell<RenderGraph>>,
    pub render_passes: Vec<RenderPass>,
    framebuffer: Option<FramebufferHandle>,
    pub stream_buffer: StreamBufferState,
    pub assets: Assets,
    pub gpu_assets: GpuAssets,
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
                    Material::new(globals.global_programs.gouraud.clone(), &globals.global_uniforms)
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
            framebuffer: None,
            stream_buffer,
            assets,
            gpu_assets: GpuAssets::new(),
            globals,
            render_state: RenderState::new(),
            math,
        })
    }

    pub fn get_framebuffer(&self) -> Option<FramebufferHandle> {
        self.framebuffer.clone()
    }

    pub fn on_game_start(&mut self) {
        let color_target = self.new_empty_image(320, 240);
        let depth_target  = self.new_depth_buffer(320, 240);
        let framebuffer = self.new_framebuffer(color_target, depth_target);

        self.framebuffer = Some(framebuffer.clone());
    }

    pub fn on_game_shutdown(&mut self) {
        self.assets.clear();
        self.gpu_assets.clear();
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

    pub fn prepare_gpu_assets(&mut self, ctx: &Display) {
        // fonction à revoir commplètement. Le gros point noir du moteur pour l'instant.
        let asset_datas = self.assets.get_datas();

        for pass in self.render_graph.borrow().get_passes().iter() {
            for cmd in pass.get_cmds() {
                if self.gpu_assets.get::<GpuMesh>(cmd.mesh.get_id()).is_none() {
                    let mesh = asset_datas
                        .get::<Mesh>(cmd.mesh.get_id())
                        .expect("Missing primitive resource");

                    // construct gpu primitive
                    match mesh.prepare_rendering(ctx, &self.assets, &self.gpu_assets) {
                        Ok(gpu_mesh) => self.gpu_assets.add(cmd.mesh.get_id(), gpu_mesh),
                        Err(_) => todo!(),
                    }

                    // construct gpu objects needed by the material
                    if let Some(material) = self.assets.get_datas().get::<Material>(mesh.material) {
                        for uniform_handle in material.get_uniforms() {
                            if let Some(uniform_handle) = uniform_handle {
                                if let Some(uniform) = uniform_handle.1.get_datas().get::<Uniform>(uniform_handle.1.get_id()) {
                                    match uniform.get_value() {
                                        UniformValue::Texture(id) => {
                                            if let Some(texture) = asset_datas.get::<Image>(*id) {                      
                                                match texture.prepare_rendering(ctx, &self.assets, &self.gpu_assets) {
                                                    Ok(gpu_image) => self.gpu_assets.add(*id, gpu_image),
                                                    Err(_) => todo!(),
                                                }
                                            }
                                        },
                                        _ => {
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }     
                }                
            }     
        }

        if let Some(pipeline) = self.assets.get_datas().get::<Pipeline>(self.globals.global_pipelines.default_pipeline.get_id()) {
            if self.gpu_assets.get::<GpuProgram>(pipeline.get_program().get_id()).is_none() {
                if let Some(program) = self.assets.get_datas().get::<Program>(pipeline.get_program().get_id()) {
                    match program.prepare_rendering(ctx, &self.assets, &self.gpu_assets)  {
                        Ok(gpu_program) => self.gpu_assets.add(pipeline.get_program().get_id(), gpu_program),
                        Err(_) => todo!(),
                    }
                }
            }
        }

        if let Some(framebuffer_handle) = &self.framebuffer {
            if let Some(framebuffer) = framebuffer_handle.get_datas().get::<Framebuffer>(framebuffer_handle.get_id()) {
                let color_handle = framebuffer.get_color_target();
                let depth_handle = framebuffer.get_depth_target();
                let color_id = color_handle.get_id();
                let depth_id = depth_handle.get_id();
                if self.gpu_assets.get::<GpuImage>(color_id).is_none() {
                    if let Some(image) = self.assets.get_datas().get::<Image>(color_id) {
                        match image.prepare_rendering(ctx, &self.assets, &self.gpu_assets)  {
                            Ok(gpu_image) => self.gpu_assets.add(color_id, gpu_image),
                            Err(_) => todo!(),
                        }
                    }
                }
                if self.gpu_assets.get::<GpuDepthBuffer>(depth_id).is_none() {
                    if let Some(depth) = self.assets.get_datas().get::<DepthBuffer>(depth_id) {
                        match depth.prepare_rendering(ctx, &self.assets, &self.gpu_assets)  {
                            Ok(gpu_depth) => self.gpu_assets.add(depth_id, gpu_depth),
                            Err(_) => todo!(),
                        }
                    }
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

    pub fn new_image(&mut self, path: &String) -> Result<ImageHandle, ImageError> {
        let image = Image::from_path(path)?;
        Ok(
            ImageHandle::new(
                self.assets.clone(),
                self.assets.add(Box::new(image))
            )
        )
    }

    pub fn new_empty_image(&mut self, width: u32, height: u32) -> ImageHandle {
        let image = Image::new(width, height);
        ImageHandle::new(
            self.assets.clone(),
            self.assets.add(Box::new(image))
        )
    }

    pub fn new_depth_buffer(&mut self, width: u32, height: u32) -> DepthBufferHandle {
        let depth_buffer = DepthBuffer::new(width, height);
        DepthBufferHandle::new(
            self.assets.clone(),
            self.assets.add(Box::new(depth_buffer))
        )
    }

    pub fn new_framebuffer(&mut self, color_target: ImageHandle, depth_target: DepthBufferHandle) -> FramebufferHandle {
        let framebuffer = Framebuffer::new(color_target, depth_target);
        FramebufferHandle::new(
            self.assets.clone(),
            self.assets.add(Box::new(framebuffer))
        )
    }

    pub fn bind_texture(&mut self, image: ImageHandle) {
        // match self.render_passes.last_mut() {
        //     Some(render_pass) => {
        //         render_pass.current_texture = Some(image);
        //     },
        //     None => return
        // };
    }

    pub fn new_model(&mut self, path: &String) -> Result<ModelHandle, GltfError> {
        let model = GltfLoader::load(path, &mut self.assets, self.math.clone(), &self.globals)?;

        Ok(
            ModelHandle::new(
                self.assets.clone(), 
                self.assets.add(Box::new(model))
            )
        )
    }
    
    pub fn new_mesh(&mut self) -> Result<MeshHandle, GltfError> {
        let vertex_buffer:Vec<Vertex> = Vec::new();
        let material_id = self.new_gouraud_material();

        Ok(
            MeshHandle::new(
                self.assets.clone(),
                self.assets.add(
                    Box::new(
                        Mesh::new(
                            vertex_buffer,
                            None,
                            PrimitiveType::Triangles,
                            material_id.get_id()
                        )
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
    //         self.globals.global_materials.std_2d, 
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

    pub fn new_gouraud_material(&mut self) -> MaterialHandle {
        let mut material = Material::new(
            self.globals.global_programs.gouraud_textured.clone(), 
            &self.globals.global_uniforms
        );
        material.add_uniform("u_enable_fog", self.globals.global_uniforms.enable_fog.clone());
        material.add_uniform("u_fog_start", self.globals.global_uniforms.fog_start.clone());
        material.add_uniform("u_fog_end", self.globals.global_uniforms.fog_end.clone());
        material.add_uniform("u_enable_lighting", self.globals.global_uniforms.enable_lighting.clone());

        MaterialHandle::new(
            self.assets.clone(),
            self.assets.add(
             Box::new(material)
            )
        )
    }

    pub fn new_2d_material(&mut self) -> MaterialHandle {
        let material = Material::new(
            self.globals.global_programs.std_2d.clone(), 
            &self.globals.global_uniforms
        );
        MaterialHandle::new(
            self.assets.clone(),
            self.assets.add(
             Box::new(material)
            )
        )
    }

    // pub fn new_uniform<T: UniformType>(&mut self, value: T) -> UniformId {
    //     self.assets.add(
    //         Box::new(
    //             Uniform::new(value)
    //         )
    //     )
    // }

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
        let color_target = self.new_empty_image(800, 600);
        let depth_target = self.new_depth_buffer(800, 600);

        let framebuffer = FramebufferHandle::new(
            self.assets.clone(), 
            self.assets.add(Box::new(Framebuffer::new(color_target, depth_target)))
        );

        PassHandle {
            graph: self.render_graph.clone(),
            id: self.render_graph.borrow_mut().create_pass(framebuffer),
        }
    }

    pub fn set_clear_color(&mut self, color: &Vec4) {
        self.render_state.clear_color = *color;
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
            data: &mut mesh.vertices[
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