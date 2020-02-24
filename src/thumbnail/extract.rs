use crate::ffmpeg_api::api::*;
use crate::ffmpeg_api::enums::*;
use crate::util::media_time::*;
use crate::thumbnail::spritesheet::*;

use failure::format_err;

pub fn extract<T: AsRef<str>, U: AsRef<str>>(
    max_size: u32,
    num_horizontal: u32, num_vertical: u32,
    frame_interval: MediaTime,
    input_file: T,
    output_folder: U,
) -> Result<(), failure::Error> {
    let mut avformat_context = AVFormatContext::new().map_err(|error| {
        format_err!("Could not allocate a context to process the video: {:?}", error)
    })?;
    avformat_context.open_input(input_file.as_ref()).map_err(|error| {
        format_err!("Could not open video input: {:?}", error)
    })?;

    let mut spritesheet_manager = SpritesheetManager::new(
        max_size,
        num_horizontal, num_vertical,
        frame_interval,
        &output_folder,
        "preview"
    );

    let mut stream: AVStream = avformat_context.find_stream(|stream| {
        stream.codec_parameters().codec_type() == AVMediaType::Video
    }).ok_or_else(|| {
        format_err!("Could not find video stream")
    })?;

    stream.set_discard(AVDiscard::NonKey);

    let codec_parameters = stream.codec_parameters();
    let local_codec = codec_parameters.find_decoder();

    println!(
        "Stream #{}, type: {:#?}, codec: {:#?}",
        stream.index(),
        codec_parameters.codec_type(),
        local_codec.name()
    );

    let mut output_frame = AVFrame::new().map_err(|error| {
        format_err!("Could not create output frame: {:?}", error)
    })?;

    if codec_parameters.codec_type() == AVMediaType::Video {
        let mut codec_context = AVCodecContext::new(&local_codec).map_err(|error| {
            format_err!("Could not init codec context: {:?}", error)
        })?;
        codec_context.set_parameters(&codec_parameters);
        codec_context.open(&local_codec);

        codec_context.set_skip_loop_filter(AVDiscard::NonKey);
        codec_context.set_skip_idct(AVDiscard::NonKey);
        codec_context.set_skip_frame(AVDiscard::NonKey);

        let mut packet = AVPacket::new().map_err(|error| {
            format_err!("Could not init temporary packet: {:?}", error)
        })?;

        let mut frame = AVFrame::new().map_err(|error| {
            format_err!("Could not create input frame: {:?}", error)
        })?;

        let mut scale_context = SwsContext::new();

        while avformat_context.read_frame(&mut packet).is_ok() {
            if packet.stream_index() == stream.index() {
                codec_context.in_packet(&mut packet).map_err(|error| {
                    format_err!("Could not load packet: {:?}", error)
                })?;
                while codec_context.out_frame(&mut frame).is_ok() {
                    println!(
                        "Frame {}: {} @ {}",
                        frame.coded_picture_number(),
                        stream.timestamp(frame.pts())?,
                        frame.key_frame()
                    );

                    if spritesheet_manager.fulfils_frame_interval(stream.timestamp(frame.pts())?) {
                        if !spritesheet_manager.initialized() {
                            spritesheet_manager.initialize(frame.width() as u32, frame.height() as u32);
                            output_frame.init(
                                spritesheet_manager.sprite_width() as i32,
                                spritesheet_manager.sprite_height() as i32,
                                AVPixelFormat::RGB24,
                            ).map_err(|error| {
                                format_err!("Could not init output frame: {:?}", error)
                            })?;
                            scale_context.reinit(
                                &frame,
                                &output_frame,
                                SwsScaler::FastBilinear,
                            ).map_err(|error| {
                                format_err!("Could not reinit scale context: {:?}", error)
                            })?;
                        }

                        scale_context.scale(&frame, &mut output_frame);

                        spritesheet_manager.add_image(
                            stream.timestamp(frame.pts())?,
                            image::ImageBuffer::from_raw(
                                output_frame.width() as u32,
                                output_frame.height() as u32,
                                output_frame.data(0).to_vec(),
                            ).ok_or_else(|| {
                                format_err!("Could not process frame")
                            })?
                        )?;
                    }
                }
            }
        }

        spritesheet_manager.end_frame(stream.duration()?);
        spritesheet_manager.save()?;
    }

    Ok(())
}