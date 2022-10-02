use gltf::buffer::Data;
use ::image::ImageError;
use rlua::UserData;
use verdi_math::Mat4;

use thiserror::Error;

use crate::{
    mesh::{Mesh, Primitive}, 
    vertex::Vertex, 
    node::Node, 
    transform::Transform, 
    image::{Image, ImageRef}, 
    prelude::GraphicsChip
};

#[derive(Error, Debug)]
pub enum GltfError {
    #[error("Reading gltf file failed")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error")]
    GltfError(#[from] gltf::Error),
    #[error("Image loading eror")]
    ImageError(#[from] ImageError),
}

#[derive(Clone)]
pub struct Scene {
    pub nodes: Vec<Node>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &String, gpu: &mut GraphicsChip) -> Result<(), GltfError> {
        let gltf = gltf::Gltf::open(path)?;

        let (_, buffers, _) = gltf::import(path)?;

        let mut textures = vec![];
        for gltf_texture in gltf.textures() {
            textures.push(
                gpu.assets.add_texture(
                    Scene::load_texture(gltf_texture, &buffers)?
                )
            );
            // todo read sampler infos
        }

        let mut meshes = vec![];
        for gltf_mesh in gltf.meshes() {
            meshes.push(
                gpu.assets.add_mesh(
                    Scene::load_mesh(gltf_mesh, &buffers, &textures, gpu)?
                )
            );
        }

        for gltf_node in gltf.nodes() {
            // if let Some(camera) = gltf_node.camera()
            // {
            //     self.nodes.push(
            //         Node {

            //         }
            //     );
            // }

            self.nodes.push( 
                Node {
                    mesh: gltf_node
                        .mesh()
                        .map(|mesh| mesh.index())
                        .and_then(|i| meshes.get(i).cloned()),
                    transform: Transform::from_matrix(
                        Mat4::from_cols_array_2d(
                            &gltf_node.transform().matrix()
                        )
                    ),
                    children: vec![],
                }
            );
        }

        Ok(())
    }

    fn load_mesh(gltf_mesh: gltf::Mesh, buffers: &Vec<Data>, textures: &Vec<ImageRef>, gpu: &GraphicsChip) -> Result<Mesh, GltfError> {
        let mut primitives = Vec::new();
        for gltf_primitive in gltf_mesh.primitives() {
            let reader = gltf_primitive.reader(|buffer| Some(&buffers[buffer.index()]));

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

            let mut index_buffer = None;
            if let Some(indices) = reader.read_indices() {
                index_buffer = Some(indices.into_u32().collect());
            };

            let gltf_material = gltf_primitive.material();
            let tex_ref = gltf_material
                .pbr_metallic_roughness()
                .base_color_texture()
                .map(|info| info.texture().index())
                .and_then(|i| textures.get(i).cloned());

            let primitive = Primitive {
                vertex_buffer,
                index_buffer,
                material: gpu.globals.standard_material,
            };
            
            primitives.push(primitive);
        }

        Ok(Mesh::new(primitives))
    }

    fn load_texture(gltf_texture: gltf::Texture, buffers: &Vec<Data>) -> Result<Image, ImageError> {
        let source = match gltf_texture.source().source() {
            gltf::image::Source::View { view, mime_type } => {
                let start = view.offset() as usize;
                let end = (view.offset() + view.length()) as usize;
                let buffer = &buffers[view.buffer().index()][start..end];
                Image::from_buffer(buffer)?
            }
            gltf::image::Source::Uri { uri, mime_type } => {
                Image::new(&uri.to_string())?
            }
        };

        Ok(source)
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