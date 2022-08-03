use verdi::common::*;

struct Name(String);
struct Age(u16);

fn main() {
    App::run();
    World::new()
        .spawn()
        .add(Name)
        .add(Age);
}
