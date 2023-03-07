use std::{path::{Path, PathBuf}, io::BufReader, fs::File};

use mlua::UserData;
use rodio::Decoder;
use slotmap::new_key_type;

new_key_type! {
    pub struct SourceId;
}

pub struct Source {
    //source: Decoder<BufReader<File>>,
    path: PathBuf,
}

impl Source {
    pub fn new<P: AsRef<Path>>(path: P) -> Source {
        // Load a sound from a file, using a path relative to Cargo.toml
        //let file = BufReader::new(File::open("examples/music.ogg").unwrap());
        // Decode that sound file into a source
        //let source = Decoder::new(file).unwrap();

        Source { 
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn get_source(&self) -> Decoder<BufReader<File>> {
        //&self.source
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(self.path.clone()).unwrap());
        // Decode that sound file into a source
        Decoder::new(file).unwrap()
    }
}

#[derive(Clone)]
pub struct SourceHandle {
    pub id: SourceId,
}

impl SourceHandle {
    pub fn new(id: SourceId) -> Self {
        Self {
            id,
        }
    }
}

impl UserData for SourceHandle {}