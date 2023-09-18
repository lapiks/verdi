use std::ops::{Deref, DerefMut};

use glium::Display;
use mlua::{UserData, UserDataMethods, Table};
use slotmap::Key;
use verdi_database::{ResourceId, Resource, Assets, Handle};

use crate::{
    vertex::Vertex, 
    material::MaterialId, 
    gpu_mesh::GpuMesh, 
    gpu_assets::{GpuAsset, GpuAssetError, PrepareAsset, GpuAssets}, 
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
        match string.as_str() {
            "triangles" => return PrimitiveType::Triangles,
            "points" => return PrimitiveType::Points,
            "lines" => return PrimitiveType::Lines,
            _ => PrimitiveType::Triangles
        }
    }
}

impl From<PrimitiveType> for glium::index::PrimitiveType {
    fn from(p: PrimitiveType) -> Self {
        match p {
            PrimitiveType::Triangles => glium::index::PrimitiveType::TrianglesList,
            PrimitiveType::Points => glium::index::PrimitiveType::Points,
            PrimitiveType::Lines => glium::index::PrimitiveType::LinesList,
        }
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

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
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
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Option<Vec<u32>>,
        primitive_type: PrimitiveType,
        material: MaterialId
    ) -> Self {
        Self {
            vertices,
            indices,
            primitive_type,
            material,
            id: MeshId::null(),
        }
    }
}

impl PrepareAsset for Mesh {
    fn prepare_rendering(&self, ctx: &Display, assets: &Assets, gpu_assets: &GpuAssets) -> Result<Box<dyn GpuAsset>, GpuAssetError> {
        let vertex_buffer = glium::VertexBuffer::new(
            ctx, 
            &self.vertices
        ).unwrap();

        if let Some(indices) = &self.indices {
            let index_buffer = glium::IndexBuffer::new(
                ctx, 
                glium::index::PrimitiveType::from(self.primitive_type),
                indices
            ).unwrap();

            return Ok(
                Box::new(
                    GpuMesh::new(vertex_buffer, Some(index_buffer))
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
pub struct MeshHandle(Handle);

impl Deref for MeshHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MeshHandle {
      fn deref_mut(&mut self) -> &mut Handle {
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
                mesh.vertices.resize(v_length as usize, Vertex::default());
                // fill vertex buffer
                for (vertex_index, vertex) in vertices.sequence_values::<Table>().enumerate() {
                    if let Ok(vertex) = vertex {
                        for (comp_index, comp) in vertex.sequence_values::<f32>().enumerate() {
                            if let Ok(comp) = comp {
                                mesh.vertices[vertex_index].position[comp_index] = comp;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn set_indices(&mut self, indices_table: Table) {
        let mesh_id = self.get_id();
        if let Some(mesh) = self.get_datas_mut().get_mut::<Mesh>(mesh_id)
        {
            // TODO: if no indices yet?
            if let Ok(v_length) = indices_table.len() {
                if let Some(indices) = &mut mesh.indices {
                    indices.resize(v_length as usize, 0);
                    // fill index buffer
                    for (table_index, value) in indices_table.sequence_values::<u32>().enumerate() {
                        if let Ok(value) = value {
                            indices[table_index] = value;
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

        methods.add_method_mut("setIndices", |_, mesh, indices: Table| {
            Ok(mesh.set_indices(indices))
        });

        methods.add_method_mut("setPrimitiveType", |_, mesh, primitive_string: String| {
            Ok(mesh.set_primitive_type(PrimitiveType::from(primitive_string)))
        });
    }
}