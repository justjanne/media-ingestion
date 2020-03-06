#![allow(dead_code)]

pub(crate) mod ffmpeg_api;
pub(crate) mod ingest;
pub(crate) mod util;

use crate::util::media_time::MediaTime;
use std::path::Path;

fn main() -> Result<(), failure::Error> {
    ingest::extract::extract(
        160, 5, 5,
        MediaTime::from_seconds(2),
        Path::new("/home/kuschku/Workspace/projects/mediaflix/data/movie.mp4"),
        Path::new("/home/kuschku/Workspace/projects/mediaflix/data/output")
    )?;

    Ok(())
}
