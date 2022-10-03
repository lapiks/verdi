use glium::Display;
use rlua::UserData;
use uuid::Uuid;

use crate::{
    assets::{AssetId, Assets}, 
    vertex::Vertex, 
    gpu_assets::GpuAssets, 
    graphics_chip::PrimitiveType, 
    gpu_primitive::GpuPrimitive,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Reading gltf file failed")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
}

type VertexBuffer = Vec<Vertex>;
type IndexBuffer = Vec<u32>;

pub struct Primitive {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: Option<IndexBuffer>,
    pub primitive_type: PrimitiveType,
    pub material: AssetId,
    pub id: AssetId,
}

impl Primitive {
    pub fn prepare_rendering(&self, display: &Display, assets: &Assets, gpu_assets: &mut GpuAssets) {
        let vertex_buffer = glium::VertexBuffer::new(display, &self.vertex_buffer).unwrap();

        if let Some(index_buffer) = &self.index_buffer {
            let indices = glium::IndexBuffer::new(
                display, 
                glium::index::PrimitiveType::from(self.primitive_type),
                index_buffer
            ).unwrap();

            let gpu_mesh = GpuPrimitive::new(vertex_buffer, Some(indices));
            gpu_assets.add_primitive(self.id, gpu_mesh);
        }
        else {
            // let indices = glium::index::NoIndices(glium::index::PrimitiveType::from(render_pass.current_primitive));

            let gpu_mesh = GpuPrimitive::new(vertex_buffer, None);
            gpu_assets.add_primitive(self.id, gpu_mesh);
        }
    }
}

pub struct Mesh {
    pub primitives: Vec<Primitive>,
    pub id: AssetId,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            id: Uuid::nil(),
        }
    }

    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}

#[derive(Clone, Copy)]
pub struct MeshRef {
    pub id: AssetId,
}

impl MeshRef {
    pub fn new(id: AssetId) -> Self{
        Self { id }
    }
}

impl UserData for MeshRef {}