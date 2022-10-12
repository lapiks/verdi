use slotmap::SlotMap;

use crate::{
    image::{Image, ImageId}, 
    mesh::{Mesh, MeshId}, 
    material::{Material, MaterialId}, 
    shader::{Shader, ShaderId}, 
    program::{Program, ProgramId}, 
    scene::{Scene, SceneId}, 
    primitive::{Primitive, PrimitiveId}
};

#[derive(PartialEq)]
pub enum AssetState {
    Created,
    Loaded,
}

pub struct Assets {
    textures: SlotMap<ImageId, Image>,
    meshes: SlotMap<MeshId, Mesh>,
    primitives: SlotMap<PrimitiveId, Primitive>,
    materials: SlotMap<MaterialId, Material>,
    shaders: SlotMap<ShaderId, Shader>,
    programs: SlotMap<ProgramId, Program>,
    scenes: SlotMap<SceneId, Scene>,
}

impl Assets {
    pub fn new() -> Self {
        Self { 
            textures: SlotMap::default(),
            meshes: SlotMap::default(),
            primitives: SlotMap::default(),
            materials: SlotMap::default(),
            shaders: SlotMap::default(),
            programs: SlotMap::default(),
            scenes: SlotMap::default(),
        }
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

    pub fn add_primitive(&mut self, primitive: Primitive) -> PrimitiveId {
        let id = self.primitives.insert(primitive);
        unsafe {
            self.primitives.get_unchecked_mut(id).id = id;
        }
        id
    }

    pub fn get_primitive(&self, id: PrimitiveId) -> Option<&Primitive> {
        self.primitives.get(id)
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

    pub fn add_scene(&mut self, scene: Scene) -> SceneId {
        let id = self.scenes.insert(scene);
        unsafe {
            self.scenes.get_unchecked_mut(id).id = id;
        }
        id
    }

    pub fn get_scene(&self, id: SceneId) -> Option<&Scene> {
        self.scenes.get(id)
    }
}