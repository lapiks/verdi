use std::{path::Path, rc::Rc, cell::RefCell};

use gltf::buffer::Data;
use image::ImageError;

use thiserror::Error;
use verdi_database::Assets;
use verdi_math::{Mat4, prelude::Math};

use crate::{
    mesh::{Mesh, PrimitiveType, MeshHandle}, 
    image::{Image, ImageId}, 
    material::{Material, MaterialId}, 
    node::Node,
    vertex::Vertex, 
    model::Model, 
    globals::Globals, 
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

pub struct GltfLoader;

impl GltfLoader {
    pub fn load<P: AsRef<Path>>(path: P, assets: &mut Assets, math: Rc<RefCell<Math>>, globals: &Globals) -> Result<Model, GltfError> {
        let mut model = Model::new();

        let gltf = gltf::Gltf::open(path.as_ref())?;

        let (_, buffers, _) = gltf::import(path.as_ref())?;

        let folder = path.as_ref().parent().unwrap();

        let mut textures = vec![];
        for gltf_texture in gltf.textures() {
            let image_id = assets.add(
                Box::new(
                    GltfLoader::load_texture(
                        gltf_texture, 
                        &buffers,
                        folder
                    )?
                )
            );

            textures.push(image_id);
        }

        let mut materials = vec![];
        for gltf_material in gltf.materials() {
            materials.push(
                assets.add(
                    Box::new(
                        GltfLoader::load_material(
                            gltf_material,
                            &textures, 
                            globals
                        )
                    )
                )
            )
        }

        let mut meshes = vec![];
        for gltf_mesh in gltf.meshes() {
            for gltf_primitive in gltf_mesh.primitives() {
                // creates a mesh per gltf primitive
                meshes.push(
                    assets.add(
                        Box::new(
                            GltfLoader::load_primitive(
                                gltf_primitive, 
                                &buffers, 
                                &materials
                            )?
                        )
                    )
                );
            }
        }

        

        for gltf_node in gltf.nodes() {
            let mesh_id = gltf_node
            .mesh()
            .map(|mesh| mesh.index())
            .and_then(|i| meshes.get(i).cloned())
            .unwrap();

            let transform = math
                .borrow_mut()
                .new_transform_from_matrix(
                    Mat4::from_cols_array_2d(
                        &gltf_node.transform().matrix()
                    )
                );

            model.nodes.push( 
                Node {
                    mesh: Some(MeshHandle::new(assets.clone(), mesh_id)),
                    transform: transform,
                    children: vec![],
                }
            );
        }

        Ok(model)
    }

    fn load_primitive(gltf_primitive: gltf::Primitive, buffers: &Vec<Data>, materials: &Vec<MaterialId>) -> Result<Mesh, GltfError> {
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

        let material_id = gltf_primitive.material().index()
            .and_then(|i| materials.get(i).cloned()).unwrap(); // unwrap ??

        Ok(
            Mesh::new(
                vertex_buffer,
                index_buffer,
                PrimitiveType::Triangles,
                material_id
            )
        )
    }

    fn load_texture(gltf_texture: gltf::Texture, buffers: &Vec<Data>, folder_path: &Path) -> Result<Image, ImageError> {
        gltf_texture.sampler();
        let source = match gltf_texture.source().source() {
            gltf::image::Source::View { view, mime_type: _ } => {
                let start = view.offset() as usize;
                let end = (view.offset() + view.length()) as usize;
                let buffer = &buffers[view.buffer().index()][start..end];
                Image::from_buffer(buffer)?
            }
            gltf::image::Source::Uri { uri, mime_type: _ } => {
                let image_path = folder_path.join(uri);
                Image::new(image_path)?
            }
        };

        Ok(source)
    }

    fn load_material(gltf_material: gltf::Material, textures: &Vec<ImageId>, globals: &Globals) -> Material {
        let texture_id = gltf_material
            .pbr_metallic_roughness()
            .base_color_texture()
            .map(|info| info.texture().index())
            .and_then(|i| textures.get(i).cloned());

        let mut material = Material::new(globals.global_shaders.gouraud_textured, &globals.global_uniforms);
        material.add_uniform("u_enable_fog", globals.global_uniforms.enable_fog.get_id());
        material.add_uniform("u_fog_start", globals.global_uniforms.fog_start.get_id());
        material.add_uniform("u_fog_end", globals.global_uniforms.fog_end.get_id());
        material.add_uniform("u_enable_lighting", globals.global_uniforms.enable_lighting.get_id());

        if let Some(id) = texture_id {
            material.add_uniform("u_texture", id);
        }

        material.to_owned()
    }
}