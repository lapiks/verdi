use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    image::{Image, ImageRef}, 
    mesh::{Mesh, MeshRef}, 
    material::Material, 
    shader::Shader, 
    program::Program
};

pub type AssetId = Uuid;

#[derive(PartialEq)]
pub enum AssetState {
    Created,
    Loaded,
}

pub struct Assets {
    textures: HashMap<AssetId, Image>,
    meshes: HashMap<AssetId, Mesh>,
    materials: HashMap<AssetId, Material>,
    shaders: HashMap<AssetId, Shader>,
    programs: HashMap<AssetId, Program>,
}

impl Assets {
    pub fn new() -> Self {
        Self { 
            textures: HashMap::default(),
            meshes: HashMap::default(),
            materials: HashMap::default(),
            shaders: HashMap::default(),
            programs: HashMap::default(),
        }
    }

    pub fn add_texture(&mut self, image: Image) -> ImageRef {
        let id = Uuid::new_v4();
        self.textures.insert(id, image);

        ImageRef::new(id)
    }

    pub fn get_texture(&self, id: AssetId) -> Option<&Image> {
        self.textures.get(&id)
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshRef {
        let id = Uuid::new_v4();
        self.meshes.insert(id, mesh);

        MeshRef::new(id)
    }

    pub fn get_mesh(&self, id: AssetId) -> Option<&Mesh> {
        self.meshes.get(&id)
    }

    pub fn add_material(&mut self, material: Material) -> AssetId {
        let id = Uuid::new_v4();
        self.materials.insert(id, material);

        id
        //MeshRef::new(id)
    }

    pub fn get_material(&self, id: AssetId) -> Option<&Material> {
        self.materials.get(&id)
    }

    pub fn add_shader(&mut self, shader: Shader) -> AssetId {
        let id = Uuid::new_v4();
        self.shaders.insert(id, shader);

        id
    }

    pub fn get_shader(&self, id: AssetId) -> Option<&Shader> {
        self.shaders.get(&id)
    }

    pub fn add_program(&mut self, program: Program) -> AssetId {
        let id = Uuid::new_v4();
        self.programs.insert(id, program);

        id
    }

    pub fn get_program(&self, id: AssetId) -> Option<&Program> {
        self.programs.get(&id)
    }
}