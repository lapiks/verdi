use std::{path::{Path, PathBuf}, io::BufReader, fs::File};

use mlua::UserData;
use rodio::Decoder;
use slotmap::new_key_type;

new_key_type! {
    pub struct AudioClipId;
}

pub struct AudioClip {
    //source: Decoder<BufReader<File>>,
    path: PathBuf,
}

impl AudioClip {
    pub fn new<P: AsRef<Path>>(path: P) -> AudioClip {
        // Load a sound from a file, using a path relative to Cargo.toml
        //let file = BufReader::new(File::open("examples/music.ogg").unwrap());
        // Decode that sound file into a source
        //let source = Decoder::new(file).unwrap();

        AudioClip { 
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
pub struct AudioClipHandle {
    pub id: AudioClipId,
}

impl AudioClipHandle {
    pub fn new(id: AudioClipId) -> Self {
        Self {
            id,
        }
    }
}

impl UserData for AudioClipHandle {}