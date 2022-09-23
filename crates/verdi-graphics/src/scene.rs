use rlua::{UserData, UserDataMethods};

use crate::{mesh::{MeshRef, Mesh, Primitive}, vertex::Vertex, assets::Assets};

#[derive(Clone)]
pub struct Scene {
    pub meshes: Vec<MeshRef>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &String, assets: &mut Assets) -> Result<(), gltf::Error> {
        let gltf = gltf::Gltf::open(path)?;

        let (_, buffers, _) = gltf::import(path)?;

        for gltf_mesh in gltf.meshes() {
            let mut primitives = Vec::new();
            for gltf_primitive in gltf_mesh.primitives() {
                let reader = gltf_primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let mut primitive = Primitive::new();

                let vertex_count = reader.read_positions().unwrap().size_hint();
                let mut vertex_buffer:Vec<Vertex> = Vec::new();
                vertex_buffer.resize(vertex_count.0, Vertex::default());

                let mut i = 0;
                if let Some(positions) = reader.read_positions() {
                    for pos in positions {
                        vertex_buffer[i].position = pos;
                        i += 1;
                    }
                }

                let mut i = 0;
                if let Some(normals) = reader.read_normals() {
                    for normal in normals {
                        vertex_buffer[i].normal = normal;
                        i += 1;
                    }
                }

                let mut i = 0;
                if let Some(colors) = reader.read_colors(0) {
                    for color in colors.into_rgba_f32() {
                        vertex_buffer[i].color = color;
                        i += 1;
                    }
                }

                let mut i = 0;
                if let Some(uvs) = reader.read_tex_coords(0) {
                    for uv in uvs.into_f32() {
                        vertex_buffer[i].uv = uv;
                        i += 1;
                    }
                }

                // copy ici !!!
                primitive.vertex_buffer = vertex_buffer;

                if let Some(indices) = reader.read_indices() {
                    primitive.index_buffer = Some(indices.into_u32().collect());
                };

                primitives.push(primitive);
            }
            
            self.meshes.push(assets.add_mesh(Mesh::new(primitives)));
        }
    
        Ok(())
    }

    pub fn draw(&self) {
        
    }
}

impl UserData for Scene {
    // fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    //     methods.add_method("draw", |_, scene, ()| {
    //         //draw
    //         // ajouter une render pass par mesh à rendre dans le graphics chip
    //         // scene aurait une ref vers GraphicsChip ?
    //         Ok(())
    //     });
    // }
}