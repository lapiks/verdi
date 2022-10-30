pub mod prelude {
    pub use crate::{
        inputs::{Inputs, MouseButton, Key}, 
        bind_inputs::BindInputs,
    };
}

mod inputs;
mod bind_inputs;