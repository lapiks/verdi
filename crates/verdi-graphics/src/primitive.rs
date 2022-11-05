use std::sync::{Arc, Mutex};

use glium::Display;
use rlua::{UserData, UserDataMethods};
use slotmap::{new_key_type, Key};

use crate::{
    graphics_chip::{PrimitiveType, GraphicsChip}, 
    vertex::Vertex, 
    material::MaterialId, 
    gpu_primitive::GpuPrimitive, 
    gpu_assets::GpuAssets,
    
};

new_key_type! {
    pub struct PrimitiveId;
}

type VertexBuffer = Vec<Vertex>;
type IndexBuffer = Vec<u32>;

pub struct Primitive {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: Option<IndexBuffer>,
    pub primitive_type: PrimitiveType,
    pub material: MaterialId,
    pub id: PrimitiveId,
}

impl Primitive {
    pub fn new(vertex_buffer: VertexBuffer,
        index_buffer: Option<IndexBuffer>,
        primitive_type: PrimitiveType,
        material: MaterialId) -> Self {
            Self {
                vertex_buffer,
                index_buffer,
                primitive_type,
                material,
                id: PrimitiveId::null(),
            }
    }

    pub fn prepare_rendering(&self, display: &Display, gpu_assets: &mut GpuAssets) {
        if gpu_assets.get_primitive(self.id).is_none() {
            let vertex_buffer = glium::VertexBuffer::new(display, &self.vertex_buffer).unwrap();

            if let Some(index_buffer) = &self.index_buffer {
                let indices = glium::IndexBuffer::new(
                    display, 
                    glium::index::PrimitiveType::from(self.primitive_type),
                    index_buffer
                ).unwrap();

                let gpu_primitive = GpuPrimitive::new(vertex_buffer, Some(indices));
                gpu_assets.add_primitive(self.id, gpu_primitive);
            }
            else {
                // let indices = glium::index::NoIndices(glium::index::PrimitiveType::from(render_pass.current_primitive));

                let gpu_primitive = GpuPrimitive::new(vertex_buffer, None);
                gpu_assets.add_primitive(self.id, gpu_primitive);
            }
        }
    }
}

pub struct PrimitiveHandle {
    pub id: PrimitiveId,
    pub gpu: Arc<Mutex<GraphicsChip>>,
}

impl UserData for PrimitiveHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("vertex", |_, handle, ()| {
            Ok(())
        });
    }
}