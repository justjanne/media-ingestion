use std::path::Path;

use ffmpeg_api::enums::{AVDiscard, SwsFlags, SwsScaler};
use image::ImageFormat as ImageOutputFormat;
use media_time::MediaTime;
use structopt::StructOpt;

use preview_generator::spritesheet::SpritesheetOptions;
use preview_generator::timelens::TimelensOptions;

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
    #[structopt(long = "keyframe-interval", default_value = "2")]
    keyframe_interval: f64, // in seconds
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

    #[structopt(long = "timelens")]
    timelens: bool,
    #[structopt(long = "timelens-width", default_value = "1000")]
    timelens_width: u32,
    #[structopt(long = "timelens-height", default_value = "90")]
    timelens_height: u32,

    #[structopt(long = "spritesheet")]
    spritesheet: bool,
    #[structopt(long = "spritesheet-columns", default_value = "5")]
    spritesheet_columns: u32,
    #[structopt(long = "spritesheet-rows", default_value = "5")]
    spritesheet_rows: u32,
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

    if let Err(err) = preview_generator::extract(
        Path::new(&options.input),
        Path::new(&options.output),
        if options.spritesheet {
            Some(SpritesheetOptions {
                max_size: options.max_size,
                columns: options.spritesheet_columns,
                rows: options.spritesheet_rows,
                frame_interval: MediaTime::from_seconds_f64(options.keyframe_interval),
                format: match options.format.as_str() {
                    "jpeg" | "jpg" => ImageOutputFormat::Jpeg,
                    "png" => ImageOutputFormat::Png,
                    "bmp" => ImageOutputFormat::Bmp,
                    _ => panic!("Unsupported image format: {}", options.format),
                },
            })
        } else { None },
        if options.timelens {
            Some(TimelensOptions {
                width: options.timelens_width,
                height: options.timelens_height,
                frame_interval: MediaTime::from_seconds_f64(options.keyframe_interval),
                format: match options.format.as_str() {
                    "jpeg" | "jpg" => ImageOutputFormat::Jpeg,
                    "png" => ImageOutputFormat::Png,
                    "bmp" => ImageOutputFormat::Bmp,
                    _ => panic!("Unsupported image format: {}", options.format),
                },
            })
        } else { None },
        AVDiscard::NonKey,
        options.scaler,
        flags,
    ) {
        eprintln!("Error: {}", err)
    }

    Ok(())
}
