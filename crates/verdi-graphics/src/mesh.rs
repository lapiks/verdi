use std::sync::{Mutex, Arc};

use glium::Display;
use rlua::{UserData, UserDataMethods, Table};
use slotmap::{new_key_type, Key};

use crate::{
    graphics_chip::{GraphicsChip, PrimitiveType},
    vertex::Vertex, 
    material::MaterialId, 
    gpu_assets::GpuAssets, 
    gpu_mesh::GpuMesh, 
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Reading gltf file failed")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
}

new_key_type! {
    pub struct MeshId;
}

type VertexBuffer = Vec<Vertex>;
type IndexBuffer = Vec<u32>;

pub struct Mesh {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: Option<IndexBuffer>,
    pub primitive_type: PrimitiveType,
    pub material: MaterialId,
    pub id: MeshId,
}

impl Mesh {
    pub fn new(vertex_buffer: VertexBuffer,
        index_buffer: Option<IndexBuffer>,
        primitive_type: PrimitiveType,
        material: MaterialId
    ) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
            primitive_type,
            material,
            id: MeshId::null(),
        }
    }

    pub fn prepare_rendering(&self, display: &Display, gpu_assets: &mut GpuAssets) {
        if gpu_assets.get_mesh(self.id).is_none() {
            let vertex_buffer = glium::VertexBuffer::new(display, &self.vertex_buffer).unwrap();

            if let Some(index_buffer) = &self.index_buffer {
                let indices = glium::IndexBuffer::new(
                    display, 
                    glium::index::PrimitiveType::from(self.primitive_type),
                    index_buffer
                ).unwrap();

                let gpu_mesh = GpuMesh::new(vertex_buffer, Some(indices));
                gpu_assets.add_mesh(self.id, gpu_mesh);
            }
            else {
                // let indices = glium::index::NoIndices(glium::index::PrimitiveType::from(render_pass.current_primitive));

                let gpu_mesh = GpuMesh::new(vertex_buffer, None);
                gpu_assets.add_mesh(self.id, gpu_mesh);
            }
        }
    }
}

#[derive(Clone)]
pub struct MeshRef {
    pub gpu: Arc<Mutex<GraphicsChip>>,
    pub id: MeshId,
}

impl MeshRef {
    pub fn new(gpu: Arc<Mutex<GraphicsChip>>, id: MeshId) -> Self{
        Self { 
            gpu,
            id,
         }
    }

    pub fn set_vertices(&mut self, vertices: Table) {
        let mut gpu = self.gpu.lock().unwrap();
        let mesh = gpu.assets.get_mesh_mut(self.id).unwrap();
        
        if let Ok(v_length) = vertices.len() {
            mesh.vertex_buffer.resize(v_length as usize, Vertex::default());
            // fill mesh
            for (vertex_index, vertex) in vertices.sequence_values::<Table>().enumerate() {
                if let Ok(vertex) = vertex {
                    for (comp_index, comp) in vertex.sequence_values::<f32>().enumerate() {
                        if let Ok(comp) = comp {
                            mesh.vertex_buffer[vertex_index].position[comp_index] = comp;
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        self.gpu.lock().unwrap().draw_mesh(self.id);
    }
}

impl UserData for MeshRef {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("setVertices", |_, mesh, vertices: Table| {
            Ok(mesh.set_vertices(vertices))
        });

        methods.add_method("draw", |_, mesh, ()| {
            Ok(mesh.draw())
        });
    }
}