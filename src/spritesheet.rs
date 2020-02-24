use image::{RgbImage, ImageBuffer};
use crate::media_time::MediaTime;
use crate::webvtt::{WebVTTFile, WebVTTCue};

pub struct SpritesheetManager {
    num_horizontal: u32,
    num_vertical: u32,
    max_side: u32,
    sprite_width: u32,
    sprite_height: u32,
    spritesheet: RgbImage,
    current_image: u32,
    last_timestamp: MediaTime,
    metadata: WebVTTFile,
    output_path: std::string::String,
    initialized: bool,
}

impl SpritesheetManager {
    pub fn new<T: AsRef<str>>(max_side: u32, num_horizontal: u32, num_vertical: u32, output_path: T) -> SpritesheetManager {
        SpritesheetManager {
            num_horizontal,
            num_vertical,
            max_side,
            sprite_width: 0,
            sprite_height: 0,
            spritesheet: ImageBuffer::new(0, 0),
            current_image: 0,
            last_timestamp: MediaTime::from_millis(0),
            metadata: WebVTTFile::new(),
            output_path: std::string::String::from(output_path.as_ref()),
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
        self.reinit_buffer();
        self.initialized = true;
    }

    fn reinit_buffer(&mut self) {
        self.spritesheet = ImageBuffer::new(
            self.sprite_width * self.num_horizontal,
            self.sprite_height * self.num_vertical,
        );
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

    fn y(&self, current: u32) -> u32 {
        let index = (current / self.num_horizontal) % self.num_vertical;
        index * self.sprite_height
    }

    pub fn add_image(&mut self, timestamp: MediaTime, image: RgbImage) {
        if image.width() != self.sprite_width || image.height() != self.sprite_height {
            panic!(
                "Wrong image size: {}x{}, but expected {}x{}",
                image.width(), image.height(),
                self.sprite_width, self.sprite_height
            )
        }

        let x = self.x(self.current_image);
        let y = self.y(self.current_image);
        image::imageops::overlay(
            &mut self.spritesheet,
            &image,
            x, y,
        );

        if self.current_image != 0 {
            self.end_frame(timestamp);
        }

        self.last_timestamp = timestamp;
        self.current_image += 1;

        if self.sprite_index(self.current_image) == 0 {
            self.save();
        }
    }

    pub fn end_frame(&mut self, timestamp: MediaTime) {
        self.metadata.add(WebVTTCue::new(
            self.last_timestamp,
            timestamp,
            format!(
                "spritesheet_{}.jpg#xywh={},{},{},{}",
                self.spritesheet_index(self.current_image - 1),
                self.x(self.current_image - 1),
                self.y(self.current_image - 1),
                self.sprite_width,
                self.sprite_height
            ),
        ));
    }

    fn save_spritesheet(&mut self) {
        self.spritesheet.save(
            format!("{}/spritesheet_{}.png", self.output_path, self.spritesheet_index(self.current_image))
        ).unwrap_or_else(|error| {
            panic!("Could not write spritesheet: {}", error)
        });
        self.reinit_buffer();
    }

    pub fn save(&mut self) {
        self.save_spritesheet();
        self.metadata.save(format!("{}/spritesheet.vtt", self.output_path)).unwrap_or_else(|error| {
            panic!("Could not write spritesheet metadata: {}", error)
        });
    }
}