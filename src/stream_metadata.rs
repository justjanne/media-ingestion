use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use serde::{Deserialize, Serialize};

use media_time::MediaTime;

#[derive(Serialize, Deserialize)]
pub struct StreamMetadata {
    content_type: String,
    duration: i64,
    bitrate: i64,
    aspect_ratio: f32,
    width: i32,
    height: i32,
}

impl StreamMetadata {
    pub fn new(content_type: impl AsRef<str>, duration: MediaTime, bitrate: i64) -> StreamMetadata {
        StreamMetadata {
            content_type: String::from(content_type.as_ref()),
            duration: duration.seconds(),
            bitrate,
            aspect_ratio: 0.0,
            width: 0,
            height: 0,
        }
    }

    pub fn set_frame_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.aspect_ratio = (width as f64 / height as f64) as f32;
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        serde_json::to_writer(BufWriter::new(File::create(path)?), self)?;
        Ok(())
    }
}
