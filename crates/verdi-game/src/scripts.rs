use std::{collections::HashMap, path::{PathBuf, Path}, fs::File, io::{Read, Write}};

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

    pub fn save_script(&mut self, file_path: &PathBuf) -> std::io::Result<()> {
        if let Some(script) = self.get_script_mut(file_path) {
            script.save_at(file_path)?;
        }

        Ok(())
    }

    pub fn get_scripts(&self) -> &HashMap<PathBuf, Script> {
        &self.scripts
    }

    pub fn get_scripts_mut(&mut self) -> &mut HashMap<PathBuf, Script> {
        &mut self.scripts
    }

    pub fn get_script(&self, path: &PathBuf) -> Option<&Script> {
        self.scripts.get(path)
    }

    pub fn get_script_mut(&mut self, path: &PathBuf) -> Option<&mut Script> {
        self.scripts.get_mut(path)
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

    pub fn save_at<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut f = File::open(path)?;
        f.write_all(&self.code.as_bytes())
    }
}