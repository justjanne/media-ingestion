#![allow(dead_code)]

use std::path::Path;

use anyhow::format_err;
use ffmpeg_api::api::*;
use ffmpeg_api::enums::*;

pub mod spritesheet;
pub mod timelens;

#[allow(clippy::too_many_arguments)]
pub fn extract(
    input_file: &Path,
    output_folder: &Path,
    spritesheet_options: Option<spritesheet::SpritesheetOptions>,
    timelens_options: Option<timelens::TimelensOptions>,
    discard: AVDiscard,
    scaler: SwsScaler,
    flags: SwsFlags,
) -> anyhow::Result<()> {
    let mut avformat_context = AVFormatContext::new()?;
    avformat_context.open_input(input_file)?;
    let duration = avformat_context.duration()?;

    std::fs::create_dir_all(output_folder)?;

    let mut stream: AVStream = avformat_context
        .streams()
        .find(|stream| {
            if let Ok(codec_parameters) = stream.codec_parameters() {
                return codec_parameters.codec_type() == AVMediaType::Video;
            }

            false
        })
        .ok_or_else(|| format_err!("Could not find video stream"))?;
    stream.set_discard(discard);

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

    let mut spritesheet_manager: Option<spritesheet::SpritesheetManager> = if let Some(options) = spritesheet_options {
        Some(spritesheet::SpritesheetManager::new(
            options,
            output_folder,
            "preview",
        )?)
    } else {
        None
    };

    let mut timelens_manager: Option<timelens::TimelensManager> = if let Some(options) = timelens_options {
        Some(timelens::TimelensManager::new(
            options,
            stream.duration()?,
            output_folder,
            "preview",
        )?)
    } else {
        None
    };

    if codec_parameters.codec_type() == AVMediaType::Video {
        let mut codec_context = AVCodecContext::new(&local_codec)
            .map_err(|error| format_err!("Could not init codec context: {}", error))?;
        codec_context.set_parameters(&codec_parameters);
        codec_context.open(&local_codec);

        codec_context.set_skip_loop_filter(discard);
        codec_context.set_skip_idct(discard);
        codec_context.set_skip_frame(discard);

        let mut packet = AVPacket::new()
            .map_err(|error| format_err!("Could not init temporary packet: {}", error))?;

        let mut frame = AVFrame::new()
            .map_err(|error| format_err!("Could not create input frame: {}", error))?;

        let mut scale_context = SwsContext::new();

        while avformat_context.read_frame(&mut packet).is_ok() {
            if packet.stream_index() == index {
                let duration = media_time::MediaTime::from_rational(packet.duration(), &time_base)?;
                let start = media_time::MediaTime::from_rational(packet.pts(), &time_base)?;
                let end = start + duration;

                let mut skip = true;
                if let Some(spritesheet_manager) = &mut spritesheet_manager {
                    if spritesheet_manager.fulfils_frame_interval(end) {
                        skip = false;
                    }
                }
                if let Some(timelens_manager) = &mut timelens_manager {
                    if timelens_manager.fulfils_frame_interval(end) {
                        skip = false;
                    }
                }

                if !skip {
                    codec_context
                        .in_packet(&mut packet)
                        .map_err(|error| format_err!("Could not load packet: {}", error))?;
                    while codec_context.out_frame(&mut frame).is_ok() {
                        let timestamp = media_time::MediaTime::from_rational(frame.pts(), &time_base)?;

                        println!(
                            "Frame {}: {} @ {}",
                            frame.coded_picture_number(),
                            timestamp,
                            frame.key_frame()
                        );

                        if let Some(spritesheet_manager) = &mut spritesheet_manager {
                            if spritesheet_manager.fulfils_frame_interval(timestamp) {
                                if !spritesheet_manager.initialized() {
                                    spritesheet_manager.initialize(frame.width() as u32, frame.height() as u32)?;
                                }
                                spritesheet_manager.add_frame(&mut scale_context, scaler, flags, timestamp, &frame)?;
                            }
                        }

                        if let Some(timelens_manager) = &mut timelens_manager {
                            if timelens_manager.fulfils_frame_interval(timestamp) {
                                if !timelens_manager.initialized() {
                                    timelens_manager.initialize()?;
                                }
                                timelens_manager.add_frame(&mut scale_context, scaler, flags, timestamp, &frame)?;
                            }
                        }
                    }
                }
            }
        }

        if let Some(spritesheet_manager) = &mut spritesheet_manager {
            spritesheet_manager.end_frame(duration);
            spritesheet_manager.save()?;
        }

        if let Some(timelens_manager) = &mut timelens_manager {
            timelens_manager.save()?;
        }
    }

    Ok(())
}
