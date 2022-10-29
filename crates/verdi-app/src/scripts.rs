use std::{collections::HashMap, path::{PathBuf, Path}, fs::File, io::Read};

pub struct Scripts {
    scripts: HashMap<PathBuf, Script>
}

impl Scripts {
    pub fn new() -> Self {
        Self {
            scripts: HashMap::default(),
        }
    }

    pub fn load_dir<P: AsRef<Path>>(&mut self, dir_path: P) -> std::io::Result<()>  {
        let paths = std::fs::read_dir(dir_path).unwrap();

        for path in paths {
            let path = path.unwrap().path();
    
            match path.extension() {
                Some(p) if p == "lua" => {
                    println!("Loading script {:?}", path.file_name().unwrap());
                    self.scripts.insert(
                        path.clone(),
                        Script::new(path)?,
                    );
                },
                _ => (),
            }
        }

        Ok(())
    }

    pub fn add_script(&mut self, path: PathBuf, code: String) {
        let script = Script {
            code
        };
        
        self.scripts.insert(path, script);
    }

    fn load_code<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
        let mut f = File::open(path)?;
        let mut content: String = String::new();
        f.read_to_string(&mut content)?;
        
        Ok(content)
    }

    pub fn get_scripts(&self) -> &HashMap<PathBuf, Script> {
        &self.scripts
    }
}

pub struct Script {
    pub code: String,
}

impl Script {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self>  {
        let mut f = File::open(path)?;
        let mut code: String = String::new();
        f.read_to_string(&mut code)?;
        
        Ok(
            Self {
                code,
            }
        )
    }
}