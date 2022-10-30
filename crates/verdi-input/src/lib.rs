pub mod prelude {
    pub use crate::{
        inputs::Inputs, 
        inputs::MouseButton,
        inputs::Key,
        bind_inputs::BindInputs,
    };
}

mod inputs;
mod bind_inputs;