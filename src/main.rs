use std::path::Path;

use ffmpeg_api::enums::{SwsFlags, SwsScaler};
use image::ImageFormat as ImageOutputFormat;
use media_time::MediaTime;
use structopt::StructOpt;
use media_ingestion::ExtractOptions;

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
    #[structopt(long = "scaler", default_value = "area", parse(try_from_str = parse_scaler))]
    scaler: SwsScaler,
    #[structopt(long = "fast-chroma")]
    fast_chroma: bool,
    #[structopt(long = "fast-rounding")]
    fast_rounding: bool,
    #[structopt(long = "fast-scaling")]
    fast_scaling: bool,
}

fn main() -> anyhow::Result<()> {
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

    if let Err(err) = media_ingestion::extract(
        Path::new(&options.input),
        Path::new(&options.output),
        ExtractOptions {
            max_size: options.max_size,
            num_horizontal: options.num_horizontal,
            num_vertical: options.num_vertical,
            frame_interval: MediaTime::from_seconds(options.frame_interval),
            format: match options.format.as_str() {
                "jpeg" | "jpg" => ImageOutputFormat::Jpeg,
                "png" => ImageOutputFormat::Png,
                "bmp" => ImageOutputFormat::Bmp,
                _ => panic!("Unsupported image format: {}", options.format),
            }
        },
        options.scaler,
        flags,
    ) {
        eprintln!("Error: {}", err)
    }

    Ok(())
}
