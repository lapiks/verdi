use std::path::Path;

use gltf::buffer::Data;
use image::ImageError;

use thiserror::Error;
use verdi_math::{Mat4, prelude::Transform};

use crate::{
    mesh::{Mesh, PrimitiveType}, 
    image::Image, 
    uniforms::{UniformId, TextureUniform}, 
    prelude::GraphicsChip, 
    material::{Material, MaterialId}, 
    node::Node,
    vertex::Vertex, 
    scene::Scene, 
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
    pub fn load<P: AsRef<Path>>(path: P, gpu: &mut GraphicsChip) -> Result<Scene, GltfError> {
        let mut scene = Scene::new();

        let gltf = gltf::Gltf::open(path.as_ref())?;

        let (_, buffers, _) = gltf::import(path.as_ref())?;

        let folder = path.as_ref().parent().unwrap();

        let mut texture_uniforms = vec![];
        for gltf_texture in gltf.textures() {
            let image_id = gpu.assets.add_texture(
                GltfLoader::load_texture(
                    gltf_texture, 
                    &buffers,
                    folder
                )?
            );

            texture_uniforms.push(
                gpu.uniforms.add_texture(
                    TextureUniform::new(image_id)
                )
            );
        }

        let mut materials = vec![];
        for gltf_material in gltf.materials() {
            materials.push(
                gpu.assets.add_material(
                    GltfLoader::load_material(
                        gltf_material,
                        &texture_uniforms, 
                        gpu
                    )
                )
            )
        }

        let mut meshes = vec![];
        for gltf_mesh in gltf.meshes() {
            for gltf_primitive in gltf_mesh.primitives() {
                // creates a mesh per gltf primitive
                meshes.push(
                    gpu.assets.add_mesh(
                        GltfLoader::load_primitive(
                            gltf_primitive, 
                            &buffers, 
                            &materials
                        )?
                    )
                );
            }
        }

        for gltf_node in gltf.nodes() {
            scene.nodes.push( 
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

        Ok(scene)
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

    fn load_material(gltf_material: gltf::Material, textures: &Vec<UniformId>, gpu: &GraphicsChip) -> Material {
        let uniform_id = gltf_material
            .pbr_metallic_roughness()
            .base_color_texture()
            .map(|info| info.texture().index())
            .and_then(|i| textures.get(i).cloned());
        
        let globals = &gpu.globals;

        let mut material = Material::new(globals.global_shaders.gouraud);
        material.add_uniform("u_model", globals.global_uniforms.model_matrix);
        material.add_uniform("u_view", globals.global_uniforms.view_matrix);
        material.add_uniform("u_projection", globals.global_uniforms.perspective_matrix);
        material.add_uniform("u_resolution", globals.global_uniforms.resolution);
        material.add_uniform("u_enable_fog", globals.global_uniforms.enable_fog);
        material.add_uniform("u_fog_start", globals.global_uniforms.fog_start);
        material.add_uniform("u_fog_end", globals.global_uniforms.fog_end);
        material.add_uniform("u_enable_lighting", globals.global_uniforms.enable_lighting);
        if let Some(id) = uniform_id {
            material.add_uniform("u_texture", id);
        }

        material
    }
}