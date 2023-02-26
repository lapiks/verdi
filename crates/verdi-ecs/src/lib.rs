#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        world::{World, WorldHandle},
        bind_world::BindWorld, 
    };
}

mod world;
mod entity;
mod component;
mod system;
mod bind_world;