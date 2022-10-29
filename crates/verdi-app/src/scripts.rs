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

    pub fn load_file<P: AsRef<Path>>(&mut self, file_path: P) -> std::io::Result<()>  {
        self.scripts.insert(
            PathBuf::from(file_path.as_ref()),
            Script::new(file_path)?,
        );

        Ok(())
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