#![allow(dead_code)]

pub(crate) mod ffmpeg_api;
pub(crate) mod ingest;
pub(crate) mod util;

use std::path::Path;

use failure::Error;
use structopt::StructOpt;

use crate::ffmpeg_api::enums::{SwsFlags, SwsScaler};
use crate::util::media_time::MediaTime;
use image::ImageOutputFormat;

fn parse_scaler(src: &str) -> Result<SwsScaler, String> {
    match src {
        "fast_bilinear" => Ok(SwsScaler::FastBilinear),
        "bilinear" => Ok(SwsScaler::Bilinear),
        "bicubic" => Ok(SwsScaler::Bicubic),
        "x" => Ok(SwsScaler::X),
        "point" => Ok(SwsScaler::Point),
        "area" => Ok(SwsScaler::Area),
        "bicublin" => Ok(SwsScaler::Bicublin),
        "gauss" => Ok(SwsScaler::Gauss),
        "sinc" => Ok(SwsScaler::Sinc),
        "lanczos" => Ok(SwsScaler::Lanczos),
        "spline" => Ok(SwsScaler::Spline),
        _ => Err(format!("Invalid scaler: {}", src)),
    }
}

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
    #[structopt(long = "max-size", default_value = "240")]
    max_size: u32,
    #[structopt(long = "format", default_value = "jpg")]
    format: String,
    #[structopt(long = "quality", default_value = "90")]
    quality: u8,
    #[structopt(long = "scaler", default_value = "area", parse(try_from_str = parse_scaler))]
    scaler: SwsScaler,
    #[structopt(long = "fast-chroma")]
    fast_chroma: bool,
    #[structopt(long = "fast-rounding")]
    fast_rounding: bool,
    #[structopt(long = "fast-scaling")]
    fast_scaling: bool,
}

fn main() -> Result<(), Error> {
    let options = Options::from_args();

    let mut flags = SwsFlags::empty();
    if !options.fast_chroma {
        flags |= SwsFlags::FULL_CHROMA_INTERPOLATION | SwsFlags::FULL_CHROMA_INPUT;
    }
    if !options.fast_rounding {
        flags |= SwsFlags::ACCURATE_ROUNDING;
    }
    if !options.fast_scaling {
        flags |= SwsFlags::BIT_EXACT_SCALING;
    }

    ingest::extract::extract(
        options.max_size,
        options.num_horizontal,
        options.num_vertical,
        MediaTime::from_seconds(options.frame_interval),
        Path::new(&options.input),
        Path::new(&options.output),
        match options.format.as_str() {
            "jpeg" | "jpg" => ImageOutputFormat::Jpeg(options.quality),
            "png" => ImageOutputFormat::Png,
            "bmp" => ImageOutputFormat::Bmp,
            _ => panic!("Unsupported image format: {}", options.format),
        },
        options.scaler,
        flags,
    )?;

    Ok(())
}
