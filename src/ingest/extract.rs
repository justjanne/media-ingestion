use std::path::Path;

use failure::{format_err, Error};

use crate::ffmpeg_api::api::*;
use crate::ffmpeg_api::enums::*;
use crate::ingest::spritesheet::*;
use crate::util::media_time::*;
use crate::util::stream_metadata::*;

pub fn extract(
    max_size: u32,
    num_horizontal: u32,
    num_vertical: u32,
    frame_interval: MediaTime,
    input_file: &Path,
    output_folder: &Path,
) -> Result<(), Error> {
    let mut avformat_context = AVFormatContext::new()
        .map_err(|error| format_err!("Could not open video input: {}", error))?;
    avformat_context
        .open_input(input_file)
        .map_err(|error| format_err!("Could not open video input: {}", error))?;
    let duration = avformat_context.duration()?;

    let spritesheet_path = output_folder.join("spritesheets");
    std::fs::create_dir_all(&spritesheet_path)?;
    let mut spritesheet_manager = SpritesheetManager::new(
        max_size,
        num_horizontal,
        num_vertical,
        frame_interval,
        spritesheet_path,
        "preview",
    );

    let mut stream: AVStream = avformat_context
        .find_stream(|stream| {
            if let Ok(codec_parameters) = stream.codec_parameters() {
                return codec_parameters.codec_type() == AVMediaType::Video;
            }

            false
        })
        .ok_or_else(|| format_err!("Could not find video stream"))?;
    stream.set_discard(AVDiscard::NonKey);

    let index = stream.index();
    let time_base = stream.time_base();

    let codec_parameters = stream.codec_parameters()?;
    let local_codec = codec_parameters.find_decoder()?;

    println!(
        "Stream #{}, type: {:#?}, codec: {:#?}",
        index,
        codec_parameters.codec_type(),
        local_codec.name()?
    );

    let mut metadata = StreamMetadata::new(
        avformat_context
            .input_format()?
            .determine_mime(local_codec.name()?)?,
        duration,
        codec_parameters.bit_rate() / 1000,
    );

    let mut output_frame =
        AVFrame::new().map_err(|error| format_err!("Could not create output frame: {}", error))?;

    if codec_parameters.codec_type() == AVMediaType::Video {
        let mut codec_context = AVCodecContext::new(&local_codec)
            .map_err(|error| format_err!("Could not init codec context: {}", error))?;
        codec_context.set_parameters(&codec_parameters);
        codec_context.open(&local_codec);

        codec_context.set_skip_loop_filter(AVDiscard::NonKey);
        codec_context.set_skip_idct(AVDiscard::NonKey);
        codec_context.set_skip_frame(AVDiscard::NonKey);

        let mut packet = AVPacket::new()
            .map_err(|error| format_err!("Could not init temporary packet: {}", error))?;

        let mut frame = AVFrame::new()
            .map_err(|error| format_err!("Could not create input frame: {}", error))?;

        let mut scale_context = SwsContext::new();

        while avformat_context.read_frame(&mut packet).is_ok() {
            if packet.stream_index() == index {
                codec_context
                    .in_packet(&mut packet)
                    .map_err(|error| format_err!("Could not load packet: {}", error))?;
                while codec_context.out_frame(&mut frame).is_ok() {
                    let timestamp = MediaTime::from_rational(frame.pts(), time_base)?;

                    println!(
                        "Frame {}: {} @ {}",
                        frame.coded_picture_number(),
                        timestamp,
                        frame.key_frame()
                    );

                    if spritesheet_manager.fulfils_frame_interval(timestamp) {
                        if !spritesheet_manager.initialized() {
                            spritesheet_manager
                                .initialize(frame.width() as u32, frame.height() as u32);
                            metadata.set_frame_size(frame.width(), frame.height());
                            output_frame
                                .init(
                                    spritesheet_manager.sprite_width() as i32,
                                    spritesheet_manager.sprite_height() as i32,
                                    AVPixelFormat::RGB24,
                                )
                                .map_err(|error| {
                                    format_err!("Could not init output frame: {}", error)
                                })?;
                            scale_context
                                .reinit(&frame, &output_frame, SwsScaler::FastBilinear)
                                .map_err(|error| {
                                    format_err!("Could not reinit scale context: {}", error)
                                })?;
                        }

                        scale_context.scale(&frame, &mut output_frame);

                        spritesheet_manager.add_image(
                            timestamp,
                            image::ImageBuffer::from_raw(
                                output_frame.width() as u32,
                                output_frame.height() as u32,
                                output_frame.data(0).to_vec(),
                            )
                            .ok_or_else(|| format_err!("Could not process frame"))?,
                        )?;
                    }
                }
            }
        }

        spritesheet_manager.end_frame(duration);
        spritesheet_manager.save()?;
    }

    metadata
        .save(output_folder.join("metadata.json"))
        .map_err(|error| format_err!("Could not write stream metadata: {}", error))?;

    Ok(())
}
