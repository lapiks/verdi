use std::{collections::HashMap, path::{PathBuf, Path}, fs::OpenOptions, io::Write};

use verdi_utils::read_at_path;
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
            let file_path = path?.path();
            self.load_file(file_path)?;
        }

        Ok(())
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, file_path: P) -> std::io::Result<()>  {
        match &file_path.as_ref().extension() {
            Some(p) if *p == "lua" => {
                println!("Loading script {:?}", file_path.as_ref().file_name().unwrap());
                self.scripts.insert(
                    file_path.as_ref().to_path_buf(),
                    Script::new(file_path)?,
                );
            },
            _ => (),
        }

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
        Ok(
            Self {
                code: read_at_path(path)?
            }
        )
    }

    pub fn save_at<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        f.write_all(&self.code.as_bytes())
    }

    pub fn reload_from<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        self.code = read_at_path(path)?;

        Ok(())
    }
}