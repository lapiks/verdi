pub struct Shader {
    src: String,
}

impl Shader {
    pub fn new(src: String) -> Self {
        Self { src }
    }

    pub fn get_source(&self) -> &str {
        self.src.as_str()
    }
}