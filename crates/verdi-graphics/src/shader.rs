pub struct Shader {
    pub src: String,
}

impl Shader {
    pub fn new(src: String) -> Self {
        Self { src }
    }
}