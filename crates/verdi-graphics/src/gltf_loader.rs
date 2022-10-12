use gltf::buffer::Data;
use image::ImageError;

use thiserror::Error;
use verdi_math::Mat4;

use crate::{
    mesh::Mesh, 
    graphics_chip::PrimitiveType, 
    image::Image, 
    uniforms::{UniformId, TextureUniform}, 
    prelude::GraphicsChip, 
    material::{Material, MaterialId}, 
    node::Node, 
    transform::Transform, 
    vertex::Vertex, 
    scene::Scene, 
    primitive::Primitive
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
    pub fn load(path: &String, gpu: &mut GraphicsChip) -> Result<Scene, GltfError> {
        let mut scene = Scene::new();

        let gltf = gltf::Gltf::open(path)?;

        let (_, buffers, _) = gltf::import(path)?;

        let mut texture_uniforms = vec![];
        for gltf_texture in gltf.textures() {
            let image_id = gpu.assets.add_texture(
                GltfLoader::load_texture(
                    gltf_texture, 
                    &buffers
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
            let mesh = GltfLoader::load_mesh(
                gltf_mesh, 
                &buffers, 
                &materials,
                gpu
            )?;
            
            meshes.push(
                gpu.assets.add_mesh(
                    mesh 
                )
            );
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

    fn load_mesh(gltf_mesh: gltf::Mesh, buffers: &Vec<Data>, materials: &Vec<MaterialId>, gpu: &mut GraphicsChip) -> Result<Mesh, GltfError> {
        let mut mesh = Mesh::new();
        for gltf_primitive in gltf_mesh.primitives() {
            mesh.add_primitive(
                gpu.assets.add_primitive(
                    GltfLoader::load_primitive(
                        gltf_primitive, 
                        buffers, 
                        materials
                    )?
                )
            );
        }

        Ok(mesh)
    }

    fn load_primitive(gltf_primitive: gltf::Primitive, buffers: &Vec<Data>, materials: &Vec<MaterialId>) -> Result<Primitive, GltfError> {
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
            Primitive::new(
                vertex_buffer,
                index_buffer,
                PrimitiveType::Triangles,
                material_id
            )
        )
    }

    fn load_texture(gltf_texture: gltf::Texture, buffers: &Vec<Data>) -> Result<Image, ImageError> {
        gltf_texture.sampler();
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

    fn load_material(gltf_material: gltf::Material, textures: &Vec<UniformId>, gpu: &GraphicsChip) -> Material {
        let uniform_id = gltf_material
            .pbr_metallic_roughness()
            .base_color_texture()
            .map(|info| info.texture().index())
            .and_then(|i| textures.get(i).cloned());

        let mut material = Material::new(gpu.globals.gouraud);
        material.add_uniform("u_model", gpu.pipeline.model_matrix);
        material.add_uniform("u_view", gpu.pipeline.view_matrix);
        material.add_uniform("u_projection", gpu.pipeline.perspective_matrix);
        if let Some(id) = uniform_id {
            material.add_uniform("u_texture", id)
        }

        material
    }
}