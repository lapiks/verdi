struct Name(String);
struct Age(u16);
struct Weight(u16);
struct Size(u16);

pub fn start() {
    let mut world = World::new();

    let vertices: &[Vertex] = &[
        Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
        Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
        Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
        Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
        Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
    ];

    let indices: &[u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ];
    let mesh = Mesh::new(vertices.to_vec(), indices.to_vec());

    let entity = world.spawn()
        .add(Name("Lapiks".to_string()))
        .add(Age(29)).id();

    let entity_ref = world.entity(entity).unwrap()
        .add(Weight(68))
        .add(Size(175))
        .add(mesh)
        .remove::<Name>()
        .remove::<Age>();

    world.despawn(entity);
}

pub fn update() {

}

pub fn draw() {
    
}