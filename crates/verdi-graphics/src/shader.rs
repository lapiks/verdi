use slotmap::{new_key_type, Key};

new_key_type! {
    pub struct ShaderId;
}

pub struct Shader {
    src: String,
    pub id: ShaderId,
}

impl Shader {
    pub fn new(src: String) -> Self {
        Self { 
            src,
            id: ShaderId::null(),
        }
    }

    pub fn get_source(&self) -> &str {
        self.src.as_str()
    }
}

#[derive(Clone, Copy)]
pub struct ShaderRef {
    pub id: ShaderId,
}

impl ShaderRef {
    pub fn new(id: ShaderId) -> Self{
        Self { id }
    }
}