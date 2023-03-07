use std::{rc::Rc, cell::RefCell, path::Path};

use rodio::{OutputStream, OutputStreamHandle, Sink};
use slotmap::SlotMap;

use crate::source::{SourceHandle, SourceId, Source};

#[derive(Clone)]
pub struct AudioHandle {
    inner: Rc<RefCell<Audio>>,
}

impl AudioHandle {
    pub fn new(audio: Rc<RefCell<Audio>>) -> Self{
        Self {
            inner: audio,
        }
    }

    pub fn new_source<P: AsRef<Path>>(&self, path: P) -> SourceHandle {
        self.inner.borrow_mut().new_source(path)
    }

    pub fn play_source(&self, source: SourceHandle) {
        self.inner.borrow_mut().play_source(source);
    }
}

pub struct Audio {
    sources: SlotMap<SourceId, Source>,
    stream: Option<OutputStream>, // must not be dropped
    stream_handle: Option<OutputStreamHandle>,
    sinks: Vec<Sink>,
}

impl Audio {
    pub fn new() -> Self {
        if let Ok((stream, stream_handle)) = OutputStream::try_default() {
            Self {
                sources: SlotMap::default(),
                stream: Some(stream),
                stream_handle: Some(stream_handle),
                sinks: Vec::default(),
            }
        }
        else {
            println!("No audio device found.");
            Self {
                sources: SlotMap::default(),
                stream: None,
                stream_handle: None,
                sinks: Vec::default(),
            }
        }
    }

    pub fn add_sink(&mut self, sink: Sink) {
        self.sinks.push(sink);
    }

    pub fn new_source<P: AsRef<Path>>(&mut self, path: P) -> SourceHandle {
        let id = self.sources.insert(
            Source::new(path)
        );

        SourceHandle::new(id)
    }

    pub fn play_source(&mut self, source_handle: SourceHandle) {
        if let Some(source) = self.sources.get(source_handle.id) {
            if let Some(sink) = self.stream_handle
                .as_ref()
                .and_then(|stream_handle, | match Sink::try_new(stream_handle) {
                    Ok(sink) => {
                        let source = source.get_source();
                        sink.append(source);
                        Some(sink)
                    }
                    Err(err) => {
                        println!("Error playing sound: {err:?}");
                        None
                    }
                })
                {
                    self.sinks.push(sink);
                }

        }
        else {
            println!("Error playing sound: Audio source not found."); 
        }
    } 
}