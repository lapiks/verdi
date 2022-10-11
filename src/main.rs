use verdi::app::prelude::{App, Inputs};
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INPUTS: Mutex<Inputs> = Mutex::new(Inputs::new());
}

fn main() {
    App::run(&INPUTS).expect("Unexpected error");
}
