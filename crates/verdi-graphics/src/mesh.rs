use std::ops::{Deref, DerefMut};

use glium::Display;
use mlua::{UserData, UserDataMethods, Table};
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::{
    vertex::Vertex, 
    material::MaterialId, 
    gpu_mesh::GpuMesh, 
    gpu_assets::{GpuAsset, GpuAssetError, PrepareAsset}, 
};

use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Reading gltf file failed")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
}

pub type MeshId = ResourceId;

type VertexBuffer = Vec<Vertex>;
type IndexBuffer = Vec<u32>;

#[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: Option<IndexBuffer>,
    pub primitive_type: PrimitiveType,
    pub material: MaterialId, // toutes les instances d'un même mesh devront utiliser un même matériau
    pub id: MeshId,
}

impl Resource for Mesh {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
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
}

impl PrepareAsset for Mesh {
    fn prepare_rendering(&self, display: &Display, assets: &Assets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
        let vertex_buffer = glium::VertexBuffer::new(display, &self.vertex_buffer).unwrap();

        if let Some(index_buffer) = &self.index_buffer {
            let indices = glium::IndexBuffer::new(
                display, 
                glium::index::PrimitiveType::from(self.primitive_type),
                index_buffer
            ).unwrap();

            return Ok(
                Box::new(
                    GpuMesh::new(vertex_buffer, Some(indices))
                )
            );
        }

        Ok(
            Box::new(
                GpuMesh::new(vertex_buffer, None)
            )
        )
    }
}

#[derive(Clone)]
pub struct MeshHandle(Handle<Mesh>);

impl Deref for MeshHandle {
    type Target = Handle<Mesh>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MeshHandle {
      fn deref_mut(&mut self) -> &mut Handle<Mesh> {
        &mut self.0
    }
}

impl MeshHandle {
    pub fn new(assets: Assets, id: MeshId) -> Self {
        MeshHandle(assets.new_handle(id))
    }

    pub fn set_vertices(&mut self, vertices: Table) {
        let mesh_id = self.get_id();
        if let Some(mesh) = self.get_datas_mut().get_mut::<Mesh>(mesh_id)
        {
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
    }

    pub fn set_primitive_type(&mut self, primitive_type: PrimitiveType) {
        let mesh_id = self.get_id();
        if let Some(mesh) = self.get_datas_mut().get_mut::<Mesh>(mesh_id)
        {
            mesh.primitive_type = primitive_type;
        }
    }
}

impl UserData for MeshHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("setVertices", |_, mesh, vertices: Table| {
            Ok(mesh.set_vertices(vertices))
        });

        methods.add_method_mut("setPrimitiveType", |_, mesh, primitive_string: String| {
            Ok(mesh.set_primitive_type(PrimitiveType::from(primitive_string)))
        });
    }
}