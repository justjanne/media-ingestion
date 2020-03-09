use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use failure::{bail, format_err, Error};
use image::{DynamicImage, ImageOutputFormat, RgbImage};

use crate::util::media_time::MediaTime;
use crate::util::webvtt::{WebVTTCue, WebVTTFile};

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
}

impl SpritesheetManager {
    pub fn new(
        max_side: u32,
        num_horizontal: u32,
        num_vertical: u32,
        frame_interval: MediaTime,
        output_path: impl Into<PathBuf>,
        name: impl AsRef<str>,
        format: ImageOutputFormat,
    ) -> SpritesheetManager {
        SpritesheetManager {
            num_horizontal,
            num_vertical,
            max_side,
            sprite_width: 0,
            sprite_height: 0,
            spritesheet: RgbImage::new(0, 0),
            current_image: 0,
            last_timestamp: MediaTime::from_millis(0),
            frame_interval,
            metadata: WebVTTFile::new(),
            output_path: output_path.into(),
            name: String::from(name.as_ref()),
            format,
            initialized: false,
        }
    }

    pub fn initialize(&mut self, width: u32, height: u32) {
        if width >= height {
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
            self.sprite_height * self.num_vertical,
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
            ImageOutputFormat::Jpeg(_) => "jpeg",
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

        let x = self.x(self.current_image);
        let y = self.y(self.current_image);
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
        self.metadata.add(WebVTTCue::new(
            self.last_timestamp,
            timestamp,
            format!(
                "{}_{}.{}#xywh={},{},{},{}",
                self.name,
                self.spritesheet_index(self.current_image - 1),
                self.ending(),
                self.x(self.current_image - 1),
                self.y(self.current_image - 1),
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

        let new_buffer = self.reinit_buffer();
        DynamicImage::ImageRgb8(std::mem::replace(&mut self.spritesheet, new_buffer))
            .write_to(&mut BufWriter::new(file), self.format.clone())
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
