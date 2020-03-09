#![allow(dead_code)]

pub(crate) mod ffmpeg_api;
pub(crate) mod ingest;
pub(crate) mod util;

use std::path::Path;

use failure::Error;
use structopt::StructOpt;

use crate::util::media_time::MediaTime;

#[derive(StructOpt, Debug)]
#[structopt(author, about)]
struct Options {
    input: String,
    output: String,
    #[structopt(long = "frame-interval", default_value = "2")]
    frame_interval: i64,
    #[structopt(long = "num-horizontal", default_value = "5")]
    num_horizontal: u32,
    #[structopt(long = "num-vertical", default_value = "5")]
    num_vertical: u32,
    #[structopt(long = "max-size", default_value = "160")]
    max_size: u32,
}

fn main() -> Result<(), Error> {
    let options = Options::from_args();

    ingest::extract::extract(
        options.max_size,
        options.num_horizontal,
        options.num_vertical,
        MediaTime::from_seconds(options.frame_interval),
        Path::new(&options.input),
        Path::new(&options.output),
    )?;

    Ok(())
}
