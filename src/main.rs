use verdi::app::prelude::{App, Inputs};
use verdi::graphics::prelude::GraphicsChip;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GPU: Mutex<GraphicsChip> = Mutex::new(GraphicsChip::new().expect("GraphicsChip initialisation failed"));
    static ref INPUTS: Mutex<Inputs> = Mutex::new(Inputs::new());
}

fn main() {
    App::run(&GPU, &INPUTS).expect("Unexpected error");
}
