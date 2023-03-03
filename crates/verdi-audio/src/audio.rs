use std::{rc::Rc, cell::RefCell, path::Path};

use rodio::{OutputStream, OutputStreamHandle, Source};
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
        self.inner.borrow_mut().play_clip(clip)
    }
}

pub struct Audio {
    clips: SlotMap<AudioClipId, AudioClip>,
    stream_handle: OutputStreamHandle,
}

impl Audio {
    pub fn new() -> Self {
        let (_, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            clips: SlotMap::default(),
            stream_handle,
        }
    }

    pub fn new_clip<P: AsRef<Path>>(&mut self, path: P) -> AudioClipHandle {
        let id = self.clips.insert(
            AudioClip::new(path)
        );

        AudioClipHandle::new(id)
    }

    pub fn play_clip(&self, clip: AudioClipHandle) {
        if let Some(clip) = self.clips.get(clip.id) {
            let source = clip.get_source();
            self.stream_handle.play_raw(source.convert_samples());
        }
    } 
}