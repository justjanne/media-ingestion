use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use anyhow::{Error, format_err};
use ffmpeg_api::api::{AVFrame, SwsContext};
use ffmpeg_api::enums::{AVPixelFormat, SwsFlags, SwsScaler};
use image::{DynamicImage, GenericImageView, ImageFormat as ImageOutputFormat, RgbImage};
use media_time::MediaTime;

pub enum ImageFormat {
    Jpeg(i32),
    Png,
}

#[derive(Copy, Clone, Debug)]
pub struct TimelensOptions {
    pub width: u32,
    pub height: u32,
    pub frame_interval: MediaTime,
    pub format: ImageOutputFormat,
}

pub struct TimelensManager {
    width: u32,
    height: u32,
    sprite_width: u32,
    sprite_height: u32,
    spritesheet: RgbImage,
    current_image: u32,
    last_timestamp: MediaTime,
    frame_interval: MediaTime,
    output_path: PathBuf,
    name: String,
    format: ImageOutputFormat,
    initialized: bool,
    buffer: AVFrame,
}

impl TimelensManager {
    pub fn new(
        options: TimelensOptions,
        duration: MediaTime,
        output_path: impl Into<PathBuf>,
        name: impl AsRef<str>,
    ) -> Result<TimelensManager, Error> {
        let frame_count = duration.milliseconds() as f64 / options.frame_interval.milliseconds() as f64;
        let frame_count = frame_count.ceil() as u32;
        Ok(TimelensManager {
            width: options.width,
            height: options.height,
            sprite_width: 1,
            sprite_height: options.height,
            spritesheet: RgbImage::new(frame_count, options.height),
            current_image: 0,
            last_timestamp: MediaTime::from_millis(0),
            frame_interval: options.frame_interval,
            output_path: output_path.into(),
            name: String::from(name.as_ref()),
            format: options.format,
            initialized: false,
            buffer: AVFrame::new()
                .map_err(|error| format_err!("Could not create output frame: {}", error))?,
        })
    }

    pub fn initialize(&mut self) -> Result<(), Error> {
        self.buffer.init(
            self.sprite_width as i32, self.sprite_height as i32,
            AVPixelFormat::RGB24,
        ).map_err(|error| {
            format_err!("Could not init output frame: {}", error)
        })?;
        self.initialized = true;
        Ok(())
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }

    fn ending(&self) -> String {
        String::from(match self.format {
            ImageOutputFormat::Png => "png",
            ImageOutputFormat::Jpeg => "jpeg",
            ImageOutputFormat::Bmp => "bmp",
            _ => panic!("Invalid image format: {:?}", self.format),
        })
    }

    pub fn fulfils_frame_interval(&self, timestamp: MediaTime) -> bool {
        self.current_image == 0 || timestamp - self.last_timestamp >= self.frame_interval
    }

    pub fn add_frame(&mut self, context: &mut SwsContext, scaler: SwsScaler, flags: SwsFlags, timestamp: MediaTime, frame: &AVFrame) -> Result<(), Error> {
        context.reinit(frame, &self.buffer, scaler, flags)?;
        context.scale(frame, &mut self.buffer);

        let image = image::ImageBuffer::from_raw(
            self.buffer.width() as u32,
            self.buffer.height() as u32,
            self.buffer.data(0).to_vec(),
        ).ok_or_else(|| format_err!("Could not process frame"))?;

        image::imageops::overlay(&mut self.spritesheet, &image, self.current_image.into(), 0);

        self.last_timestamp = timestamp;
        self.current_image += 1;

        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Error> {
        let name = format!(
            "{}_{}.{}",
            self.name,
            "timelens",
            self.ending(),
        );

        let file = File::create(self.output_path.join(&name))
            .map_err(|err| format_err!("Could not create spritesheet {}: {}", &name, err))?;

          let view = self.spritesheet.view(
            0,
            0,
            self.current_image * self.sprite_width,
            self.sprite_height,
        ).to_image();

        let mut context = SwsContext::new();
        let mut source = AVFrame::new()?;
        source.init(
            view.width() as i32,
            view.height() as i32,
            AVPixelFormat::RGB24,
        ).map_err(|error| {
            format_err!("Could not init output frame: {}", error)
        })?;
        source.data_mut(0usize).write_all(
            view.as_raw().as_slice()
        )?;
        let mut scaled_result = AVFrame::new()?;
        scaled_result.init(
            self.width as i32, self.height as i32,
            AVPixelFormat::RGB24,
        ).map_err(|error| {
            format_err!("Could not init output frame: {}", error)
        })?;
        context.reinit(&source, &scaled_result, SwsScaler::Point, SwsFlags::empty())?;
        context.scale(&source, &mut scaled_result);

        let image = image::ImageBuffer::from_raw(
            scaled_result.width() as u32,
            scaled_result.height() as u32,
            scaled_result.data(0).to_vec(),
        ).ok_or_else(|| format_err!("Could not process frame"))?;

        DynamicImage::ImageRgb8(image)
            .write_to(&mut BufWriter::new(file), self.format)
            .map_err(|err| format_err!("Could not write spritesheet {}: {}", &name, err))?;

        Ok(())
    }
}
