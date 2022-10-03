use uuid::Uuid;

use crate::assets::AssetId;

pub struct Shader {
    src: String,
    pub id: AssetId,
}

impl Shader {
    pub fn new(src: String) -> Self {
        Self { 
            src,
            id: Uuid::nil(),
        }
    }

    pub fn get_source(&self) -> &str {
        self.src.as_str()
    }
}