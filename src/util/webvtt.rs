use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use crate::util::media_time::MediaTime;

pub struct WebVTTFile {
    cues: Vec<WebVTTCue>
}

pub struct WebVTTCue {
    start: MediaTime,
    end: MediaTime,
    payload: std::string::String,
}

impl WebVTTFile {
    pub fn new() -> WebVTTFile {
        WebVTTFile {
            cues: Vec::new()
        }
    }

    pub fn add(&mut self, cue: WebVTTCue) {
        self.cues.push(cue);
    }

    pub fn save(&self, path: std::string::String) -> Result<(), std::io::Error> {
        let file = File::create(path)?;
        let mut file = LineWriter::new(file);
        file.write_all(b"WEBVTT\n\n")?;
        for cue in &self.cues {
            cue.save(&mut file)?;
        }
        file.flush()?;
        Ok(())
    }
}

impl WebVTTCue {
    pub fn new(start: MediaTime, end: MediaTime, payload: std::string::String) -> WebVTTCue {
        WebVTTCue { start, end, payload }
    }

    fn save(&self, writer: &mut LineWriter<File>) -> Result<(), std::io::Error>{
        writer.write_all(format!("{} --> {}\n", self.start, self.end).as_bytes())?;
        writer.write_all(self.payload.as_bytes())?;
        writer.write_all(b"\n\n")?;

        Ok(())
    }
}
