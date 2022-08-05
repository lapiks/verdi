use verdi::common::*;

struct Name(String);
struct Age(u16);
struct Weight(u16);

fn main() {
    //App::run();

    let mut world = World::new();

    let entity = world.spawn()
        .add(Name("Lapiks".to_string()))
        .add(Age(29)).id();

    let entity_ref = world.entity(entity).unwrap()
        .add(Weight(68));
}
