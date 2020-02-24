#![allow(dead_code)]

pub(crate) mod ffmpeg_api;
pub(crate) mod thumbnail;
pub(crate) mod util;

use crate::util::media_time::MediaTime;

fn main() -> Result<(), failure::Error> {
    thumbnail::extract::extract(
        160, 5, 5,
        MediaTime::from_seconds(2),
        "/home/janne/Workspace/justflix/data/video.mp4",
        "/home/janne/Workspace/justflix/data/spritesheets"
    )?;

    Ok(())
}
