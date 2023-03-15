use slotmap::SlotMap;

use crate::{
    image::{Image, ImageId}, 
    mesh::{Mesh, MeshId}, 
    material::{Material, MaterialId}, 
    shader::{Shader, ShaderId}, 
    program::{Program, ProgramId}, 
    model::{Model, ModelId}, 
    camera::{Camera, CameraId},
};

#[derive(PartialEq)]
pub enum AssetState {
    Created,
    Loaded,
}

pub struct Assets {
    textures: SlotMap<ImageId, Image>,
    meshes: SlotMap<MeshId, Mesh>,
    materials: SlotMap<MaterialId, Material>,
    shaders: SlotMap<ShaderId, Shader>,
    programs: SlotMap<ProgramId, Program>,
    models: SlotMap<ModelId, Model>,
    cameras: SlotMap<CameraId, Camera>,
}

impl Assets {
    pub fn new() -> Self {
        Self { 
            textures: SlotMap::default(),
            meshes: SlotMap::default(),
            materials: SlotMap::default(),
            shaders: SlotMap::default(),
            programs: SlotMap::default(),
            models: SlotMap::default(),
            cameras: SlotMap::default(),
        }
    }

    pub fn clear(&mut self) {
        self.textures.clear();
        self.meshes.clear();
        self.materials.clear();
        self.shaders.clear();
        self.programs.clear();
        self.models.clear();
        self.cameras.clear();
    }

    pub fn add_texture(&mut self, image: Image) -> ImageId {
        let id = self.textures.insert(image);
        unsafe {
            self.textures.get_unchecked_mut(id).id = id;
        }
        id
    }

    pub fn get_texture(&self, id: ImageId) -> Option<&Image> {
        self.textures.get(id)
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshId {
        let id = self.meshes.insert(mesh);
        unsafe {
            self.meshes.get_unchecked_mut(id).id = id;
        }
        id
    }

    pub fn get_mesh(&self, id: MeshId) -> Option<&Mesh> {
        self.meshes.get(id)
    }

    pub fn get_mesh_mut(&mut self, id: MeshId) -> Option<&mut Mesh> {
        self.meshes.get_mut(id)
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        let id = self.materials.insert(material);
        unsafe {
            self.materials.get_unchecked_mut(id).id = id;
        }
        id
    }

    pub fn get_material(&self, id: MaterialId) -> Option<&Material> {
        self.materials.get(id)
    }

    pub fn get_material_mut(&mut self, id: MaterialId) -> Option<&mut Material> {
        self.materials.get_mut(id)
    }

    pub fn add_shader(&mut self, shader: Shader) -> ShaderId {
        let id = self.shaders.insert(shader);
        unsafe {
            self.shaders.get_unchecked_mut(id).id = id;
        }
        id
    }

    pub fn get_shader(&self, id: ShaderId) -> Option<&Shader> {
        self.shaders.get(id)
    }

    pub fn add_program(&mut self, program: Program) -> ProgramId {
        let id = self.programs.insert(program);
        unsafe {
            self.programs.get_unchecked_mut(id).id = id;
        }
        id
    }

    pub fn get_program(&self, id: ProgramId) -> Option<&Program> {
        self.programs.get(id)

    }

    pub fn add_model(&mut self, model: Model) -> ModelId {
        let id = self.models.insert(model);
        unsafe {
            self.models.get_unchecked_mut(id).id = id;
        }
        id
    }

    pub fn get_model(&self, id: ModelId) -> Option<&Model> {
        self.models.get(id)
    }

    pub fn add_camera(&mut self, camera: Camera) -> CameraId {
        let id = self.cameras.insert(camera);
        unsafe {
            self.cameras.get_unchecked_mut(id).id = id;
        }
        id
    }

    pub fn get_camera(&self, id: CameraId) -> Option<&Camera> {
        self.cameras.get(id)
    }

    pub fn get_camera_mut(&mut self, id: CameraId) -> Option<&mut Camera> {
        self.cameras.get_mut(id)
    }
}