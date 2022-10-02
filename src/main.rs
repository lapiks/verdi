use verdi::app::prelude::App;
use verdi::graphics::prelude::GraphicsChip;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GPU: Mutex<GraphicsChip> = Mutex::new(GraphicsChip::new().expect("GraphicsChip initialisation failed"));
}

fn main() {
    App::run(&GPU).expect("Unexpected error");
}
