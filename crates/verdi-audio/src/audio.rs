use std::{rc::Rc, cell::RefCell, path::Path};

use rodio::{OutputStream, OutputStreamHandle, Sink};
use slotmap::SlotMap;

use crate::audio_clip::{AudioClipHandle, AudioClipId, AudioClip};

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

    pub fn new_clip<P: AsRef<Path>>(&self, path: P) -> AudioClipHandle {
        self.inner.borrow_mut().new_clip(path)
    }

    pub fn play_clip(&self, clip: AudioClipHandle) {
        self.inner.borrow_mut().play_clip(clip);
    }
}

pub struct Audio {
    clips: SlotMap<AudioClipId, AudioClip>,
    stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sinks: Vec<Sink>,
}

impl Audio {
    pub fn new() -> Self {
        if let Ok((stream, stream_handle)) = OutputStream::try_default() {
            Self {
                clips: SlotMap::default(),
                stream: Some(stream),
                stream_handle: Some(stream_handle),
                sinks: Vec::default(),
            }
        }
        else {
            println!("No audio device found.");
            Self {
                clips: SlotMap::default(),
                stream: None,
                stream_handle: None,
                sinks: Vec::default(),
            }
        }
    }

    pub fn add_sink(&mut self, sink: Sink) {
        self.sinks.push(sink);
    }

    pub fn new_clip<P: AsRef<Path>>(&mut self, path: P) -> AudioClipHandle {
        let id = self.clips.insert(
            AudioClip::new(path)
        );

        AudioClipHandle::new(id)
    }

    pub fn play_clip(&mut self, clip_handle: AudioClipHandle) {
        if let Some(clip) = self.clips.get(clip_handle.id) {
            if let Some(sink) = self.stream_handle
                .as_ref()
                .and_then(|stream_handle, | match Sink::try_new(stream_handle) {
                    Ok(sink) => {
                        let source = clip.get_source();
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