use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::{bail, Error, format_err};
use image::{DynamicImage, GenericImageView, ImageFormat as ImageOutputFormat, RgbImage};
use media_time::MediaTime;
use webvtt::{WebVTTCue, WebVTTFile};
use crate::options::ExtractOptions;

pub enum ImageFormat {
    Jpeg(i32),
    Png,
}

pub struct SpritesheetManager {
    num_horizontal: u32,
    num_vertical: u32,
    max_side: u32,
    sprite_width: u32,
    sprite_height: u32,
    spritesheet: RgbImage,
    current_image: u32,
    last_timestamp: MediaTime,
    frame_interval: MediaTime,
    metadata: WebVTTFile,
    output_path: PathBuf,
    name: String,
    format: ImageOutputFormat,
    initialized: bool,
    timelens: bool,
}

impl SpritesheetManager {
    pub fn new(
        options: ExtractOptions,
        frame_count: u32,
        output_path: impl Into<PathBuf>,
        name: impl AsRef<str>,
    ) -> SpritesheetManager {
        SpritesheetManager {
            num_horizontal: if options.timelens { frame_count } else { options.num_horizontal },
            num_vertical: if options.timelens { 1 } else { options.num_vertical },
            max_side: options.max_size,
            sprite_width: 0,
            sprite_height: 0,
            spritesheet: RgbImage::new(0, 0),
            current_image: 0,
            last_timestamp: MediaTime::from_millis(0),
            frame_interval: options.frame_interval,
            metadata: WebVTTFile::new(),
            output_path: output_path.into(),
            name: String::from(name.as_ref()),
            format: options.format,
            initialized: false,
            timelens: options.timelens,
        }
    }

    pub fn initialize(&mut self, width: u32, height: u32) {
        if self.timelens {
            self.sprite_width = 1;
            self.sprite_height = self.max_side;
        } else if width >= height {
            self.sprite_width = self.max_side;
            self.sprite_height = self.sprite_width * height / width;
        } else {
            self.sprite_height = self.max_side;
            self.sprite_width = self.sprite_height * width / height;
        }
        self.spritesheet = self.reinit_buffer();
        self.initialized = true;
    }

    fn reinit_buffer(&self) -> RgbImage {
        RgbImage::new(
            self.sprite_width * self.num_horizontal,
            self.num_vertical * self.sprite_height,
        )
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }

    pub fn sprite_width(&self) -> u32 {
        self.sprite_width
    }

    pub fn sprite_height(&self) -> u32 {
        self.sprite_height
    }

    fn sprite_index(&self, current: u32) -> u32 {
        current % (self.num_horizontal * self.num_vertical)
    }

    fn spritesheet_index(&self, current: u32) -> u32 {
        current / (self.num_horizontal * self.num_vertical)
    }

    fn x(&self, current: u32) -> u32 {
        let index = current % self.num_horizontal;
        index * self.sprite_width
    }

    fn ending(&self) -> String {
        String::from(match self.format {
            ImageOutputFormat::Png => "png",
            ImageOutputFormat::Jpeg => "jpeg",
            ImageOutputFormat::Bmp => "bmp",
            _ => panic!("Invalid image format: {:?}", self.format),
        })
    }

    fn y(&self, current: u32) -> u32 {
        let index = (current / self.num_horizontal) % self.num_vertical;
        index * self.sprite_height
    }

    pub fn fulfils_frame_interval(&self, timestamp: MediaTime) -> bool {
        self.current_image == 0 || timestamp - self.last_timestamp > self.frame_interval
    }

    pub fn add_image(&mut self, timestamp: MediaTime, image: RgbImage) -> Result<(), Error> {
        if image.width() != self.sprite_width || image.height() != self.sprite_height {
            bail!(
                "Wrong image size: {}x{}, but expected {}x{}",
                image.width(),
                image.height(),
                self.sprite_width,
                self.sprite_height
            )
        }

        let x: i64 = self.x(self.current_image).into();
        let y: i64 = self.y(self.current_image).into();
        image::imageops::overlay(&mut self.spritesheet, &image, x, y);

        if self.current_image != 0 {
            self.end_frame(timestamp);
        }

        if self.sprite_index(self.current_image + 1) == 0 {
            self.save_spritesheet()?;
        }

        self.last_timestamp = timestamp;
        self.current_image += 1;

        Ok(())
    }

    pub fn end_frame(&mut self, timestamp: MediaTime) {
        let prev_image = if self.current_image > 0 {
            self.current_image - 1
        } else {
            0
        };

        self.metadata.add(WebVTTCue::new(
            self.last_timestamp,
            timestamp,
            format!(
                "{}_{}.{}#xywh={},{},{},{}",
                self.name,
                self.spritesheet_index(prev_image),
                self.ending(),
                self.x(prev_image),
                self.y(prev_image),
                self.sprite_width,
                self.sprite_height
            ),
        ));
    }

    fn save_spritesheet(&mut self) -> Result<(), Error> {
        let name = format!(
            "{}_{}.{}",
            self.name,
            self.spritesheet_index(self.current_image),
            self.ending(),
        );

        let file = File::create(self.output_path.join(&name))
            .map_err(|err| format_err!("Could not create spritesheet {}: {}", &name, err))?;

        let output = if self.timelens {
            DynamicImage::ImageRgb8(self.spritesheet.view(0, 0, self.current_image, self.sprite_height).to_image())
        } else {
            let new_buffer = self.reinit_buffer();
            DynamicImage::ImageRgb8(std::mem::replace(&mut self.spritesheet, new_buffer))
        };

        output
            .write_to(&mut BufWriter::new(file), self.format)
            .map_err(|err| format_err!("Could not write spritesheet {}: {}", &name, err))?;

        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.save_spritesheet()?;
        self.metadata
            .save(self.output_path.join(format!("{}.vtt", self.name)))
            .map_err(|error| format_err!("Could not write spritesheet metadata: {}", error))?;
        Ok(())
    }
}
