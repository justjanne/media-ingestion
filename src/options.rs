use image::ImageFormat as ImageOutputFormat;

pub struct ExtractOptions {
    pub max_size: u32,
    pub num_horizontal: u32,
    pub num_vertical: u32,
    pub frame_interval: media_time::MediaTime,
    pub format: ImageOutputFormat,
    pub timelens: bool,
}
