use crate::{vertex::Vertex, assets::Assets, mesh::Mesh};

pub fn load(path: &String, assets: &mut Assets) -> Result<(), gltf::Error> {
    let gltf = gltf::Gltf::open(path)?;

    let (_, buffers, _) = gltf::import(path)?;

    for gltf_mesh in gltf.meshes() {
        let mut primitives = Vec::new();
        for primitive in gltf_mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let vertex_count = reader.read_positions().unwrap().size_hint();
            let mut vertex_buffer:Vec<Vertex> = Vec::new();
            vertex_buffer.resize(vertex_count.0, Vertex::default());

            let i = 0;
            if let Some(positions) = reader.read_positions() {
                for pos in positions {
                    vertex_buffer[i].position = pos;
                }
            }
            if let Some(normals) = reader.read_normals() {
                for normal in normals {
                    vertex_buffer[i].normal = normal;
                }
            }
            if let Some(colors) = reader.read_colors(0) {
                for color in colors.into_rgba_f32() {
                    vertex_buffer[i].color = color;
                }
            }
            if let Some(uvs) = reader.read_tex_coords(0) {
                for uv in uvs.into_f32() {
                    vertex_buffer[i].uv = uv;
                }
            }

            primitives.push(vertex_buffer);
        }
        
        assets.add_mesh(Mesh::new(primitives));
    }

    Ok(())
}