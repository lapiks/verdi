#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        audio::{AudioHandle, Audio},
        bind_audio::BindAudio,
    };
}

mod audio;
mod bind_audio;
mod source;